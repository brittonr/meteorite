//! Cache entry types and tier/priority definitions.
//!
//! Core data structures:
//! - [`CacheEntry<V>`] — cache entry with access tracking and tier metadata
//! - [`CacheTier`] — Hot/Warm/Cold classification
//! - [`CachePriority`] — user-defined priority levels

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use web_time::Instant;

/// Maximum number of access timestamps tracked per entry.
pub const MAX_ACCESS_PATTERN_HISTORY: u32 = 100;

/// Hot/Warm/Cold classification for tiered caching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CacheTier {
    /// Frequently accessed, highest eviction resistance
    Hot = 0,
    /// Moderately accessed
    Warm = 1,
    /// Rarely accessed, lowest eviction resistance
    Cold = 2,
}

/// Priority levels for cache entries.
///
/// Higher priority entries are harder to evict. `Critical` entries get a 10x
/// eviction score multiplier vs `Normal`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CachePriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Multi-factor cache entry with access tracking.
///
/// Generic over the value type `V`. The cache stores `V` directly — if you
/// want zero-copy retrieval, use `V = Arc<T>` so clones only bump the refcount.
#[derive(Clone)]
pub struct CacheEntry<V: Clone> {
    /// The cached value
    pub data: V,
    /// Size in bytes (caller-provided, used for eviction decisions)
    pub size_bytes: u64,
    /// When this entry was created
    pub created_at: Instant,
    /// When this entry was last accessed
    pub last_access: Instant,
    /// Total number of accesses
    pub access_count: u32,
    /// Access frequency (accesses per minute)
    pub access_frequency: f64,
    /// How long the value took to compute (milliseconds)
    pub computation_cost_ms: u64,
    /// User-defined priority
    pub priority: CachePriority,
    /// Current tier classification
    pub tier: CacheTier,
    /// Recent access timestamps, bounded by [`MAX_ACCESS_PATTERN_HISTORY`]
    pub access_pattern: VecDeque<Instant>,
}

impl<V: Clone> CacheEntry<V> {
    /// Create a new entry. Starts in the [`CacheTier::Hot`] tier.
    pub fn new(
        data: V,
        size_bytes: u64,
        computation_cost_ms: u64,
        priority: CachePriority,
        now: Instant,
    ) -> Self {
        Self {
            data,
            size_bytes,
            created_at: now,
            last_access: now,
            access_count: 1,
            access_frequency: 1.0,
            computation_cost_ms,
            priority,
            tier: CacheTier::Hot,
            access_pattern: VecDeque::with_capacity(MAX_ACCESS_PATTERN_HISTORY as usize),
        }
    }

    /// Record an access, updating counters and frequency.
    ///
    /// The access pattern history is bounded at [`MAX_ACCESS_PATTERN_HISTORY`]
    /// entries — oldest timestamps are dropped when the limit is reached.
    pub fn record_access(&mut self, now: Instant) {
        self.last_access = now;
        self.access_count = self.access_count.saturating_add(1);
        self.access_pattern.push_back(self.last_access);

        if self.access_pattern.len() > MAX_ACCESS_PATTERN_HISTORY as usize {
            self.access_pattern.pop_front();
        }

        let age_minutes = now.duration_since(self.created_at).as_secs() as f64 / 60.0;
        self.access_frequency = if age_minutes > 0.0 {
            self.access_count as f64 / age_minutes
        } else {
            self.access_count as f64
        };
    }

    /// Clone the cached value.
    ///
    /// For zero-copy access, use `V = Arc<T>` — the clone only bumps the
    /// reference count.
    pub fn get_data(&self) -> V {
        self.data.clone()
    }
}
