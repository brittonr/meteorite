//! Eviction policy and scoring.
//!
//! The eviction score is a weighted combination of four factors:
//!
//! | Factor    | Meaning                                      |
//! |-----------|----------------------------------------------|
//! | Recency   | How recently was the entry accessed           |
//! | Frequency | How often is the entry accessed               |
//! | Cost      | How expensive was the entry to compute        |
//! | Size      | Larger entries are cheaper to evict per-byte  |
//!
//! The raw score is multiplied by priority and tier modifiers. Lower scores
//! are evicted first.

use super::entry::{CacheEntry, CachePriority, CacheTier};
use web_time::Instant;

/// Eviction policy weights.
///
/// The four weights should sum to 1.0. The `Default` implementation uses
/// 0.3/0.3/0.3/0.1 and validates the sum with a debug assertion.
#[derive(Debug, Clone)]
pub struct EvictionPolicy {
    /// Weight for recency (0.0–1.0)
    pub recency_weight: f64,
    /// Weight for frequency (0.0–1.0)
    pub frequency_weight: f64,
    /// Weight for computation cost (0.0–1.0)
    pub cost_weight: f64,
    /// Weight for size (0.0–1.0)
    pub size_weight: f64,
    /// Minimum age in seconds before an entry can be evicted
    pub min_age_seconds: u64,
}

impl Default for EvictionPolicy {
    fn default() -> Self {
        let policy = Self {
            recency_weight: 0.3,
            frequency_weight: 0.3,
            cost_weight: 0.3,
            size_weight: 0.1,
            min_age_seconds: 60,
        };
        let sum = policy.recency_weight
            + policy.frequency_weight
            + policy.cost_weight
            + policy.size_weight;
        debug_assert!(
            (sum - 1.0).abs() < 0.01,
            "EvictionPolicy weights must sum to 1.0, got {sum}"
        );
        policy
    }
}

/// Thresholds for tier promotion/demotion.
///
/// Entries with access frequency above `hot_frequency` are Hot. Between
/// `warm_frequency` and `hot_frequency` they're Warm. Entries idle longer
/// than `cold_age_seconds` become Cold.
#[derive(Debug, Clone)]
pub struct TierThresholds {
    /// Access frequency (per minute) to qualify as Hot
    pub hot_frequency: f64,
    /// Access frequency (per minute) to qualify as Warm
    pub warm_frequency: f64,
    /// Idle time in seconds before demotion to Cold
    pub cold_age_seconds: u64,
}

impl Default for TierThresholds {
    fn default() -> Self {
        Self {
            hot_frequency: 5.0,
            warm_frequency: 1.0,
            cold_age_seconds: 300,
        }
    }
}

impl TierThresholds {
    /// Determine the appropriate tier for an entry based on its access patterns.
    pub fn determine_tier<V: Clone>(&self, entry: &CacheEntry<V>, now: Instant) -> CacheTier {
        if entry.access_frequency >= self.hot_frequency {
            CacheTier::Hot
        } else if entry.access_frequency >= self.warm_frequency {
            CacheTier::Warm
        } else if now.duration_since(entry.last_access).as_secs() >= self.cold_age_seconds {
            CacheTier::Cold
        } else {
            entry.tier
        }
    }
}

/// Calculate the eviction score for an entry.
///
/// **Lower scores are evicted first.** The score combines four weighted
/// factors (recency, frequency, cost, size) multiplied by priority and
/// tier modifiers.
pub fn calculate_eviction_score<V: Clone>(
    policy: &EvictionPolicy,
    entry: &CacheEntry<V>,
    now: Instant,
) -> f64 {
    let age_seconds = now.duration_since(entry.last_access).as_secs() as f64;
    let recency_score = 1.0 / (1.0 + age_seconds / 3600.0);

    let frequency_score = (entry.access_frequency / 10.0).min(1.0);
    let cost_score = (entry.computation_cost_ms as f64 / 1000.0).min(1.0);
    let size_score = 1.0 - (entry.size_bytes as f64 / (10 * 1024 * 1024) as f64).min(1.0);

    let priority_multiplier = match entry.priority {
        CachePriority::Low => 0.5,
        CachePriority::Normal => 1.0,
        CachePriority::High => 2.0,
        CachePriority::Critical => 10.0,
    };

    let tier_multiplier = match entry.tier {
        CacheTier::Hot => 2.0,
        CacheTier::Warm => 1.0,
        CacheTier::Cold => 0.5,
    };

    let base_score = policy.recency_weight * recency_score
        + policy.frequency_weight * frequency_score
        + policy.cost_weight * cost_score
        + policy.size_weight * size_score;

    base_score * priority_multiplier * tier_multiplier
}
