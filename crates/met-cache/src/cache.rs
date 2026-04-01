//! Main [`TieredCache`] implementation.
//!
//! The cache stores values of type `V: Clone` keyed by `String`. It combines
//! multi-factor eviction, tier classification, access prediction, and optional
//! adaptive sizing via a memory pressure callback.

use super::entry::{CacheEntry, CachePriority, CacheTier};
use super::eviction::{calculate_eviction_score, EvictionPolicy, TierThresholds};
use super::predictor::AccessPredictor;
use super::statistics::{CacheStatistics, CacheStats};
use crate::MemoryPressure;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use web_time::Instant;

/// Maximum number of entries (hard bound regardless of byte budget).
pub const MAX_CACHE_ENTRIES: u32 = 10_000;

const INITIAL_CAPACITY: u32 = 64;

/// Minimum cache size in bytes (1 MB) to prevent degenerate shrinkage.
const MIN_CACHE_SIZE: u64 = 1_048_576;

/// Multi-tier cache with smart eviction and adaptive sizing.
///
/// Generic over value type `V`. For zero-copy retrieval, use `V = Arc<T>` —
/// [`TieredCache::get`] returns `V` via `Clone`, which for `Arc` only
/// increments the reference count.
///
/// # Eviction
///
/// When a `put` would exceed the byte budget, entries are scored by a
/// weighted combination of recency, frequency, computation cost, and size.
/// Lower-scoring entries are evicted first. Priority and tier modifiers
/// shift the score — `Critical`/`Hot` entries resist eviction.
///
/// # Adaptive Sizing
///
/// Call [`set_pressure_fn`](TieredCache::set_pressure_fn) to wire in
/// memory pressure feedback. On each `put`, the cache queries the callback
/// and adjusts its max size accordingly.
pub struct TieredCache<V: Clone> {
    entries: HashMap<String, CacheEntry<V>>,
    eviction_policy: EvictionPolicy,
    max_size_bytes: AtomicU64,
    current_size_bytes: AtomicU64,
    stats: CacheStatistics,
    access_predictor: AccessPredictor,
    tier_thresholds: TierThresholds,
    pressure_fn: Option<Box<dyn Fn() -> Option<MemoryPressure>>>,
}

impl<V: Clone> TieredCache<V> {
    /// Create a cache with the given size budget in megabytes.
    pub fn new(initial_size_mb: u32) -> Self {
        Self {
            entries: HashMap::with_capacity(INITIAL_CAPACITY as usize),
            eviction_policy: EvictionPolicy::default(),
            max_size_bytes: AtomicU64::new(initial_size_mb as u64 * 1024 * 1024),
            current_size_bytes: AtomicU64::new(0),
            stats: CacheStatistics::new(),
            access_predictor: AccessPredictor::new(),
            tier_thresholds: TierThresholds::default(),
            pressure_fn: None,
        }
    }

    /// Set a custom eviction policy.
    pub fn set_eviction_policy(&mut self, policy: EvictionPolicy) {
        self.eviction_policy = policy;
    }

    /// Set custom tier thresholds.
    pub fn set_tier_thresholds(&mut self, thresholds: TierThresholds) {
        self.tier_thresholds = thresholds;
    }

    /// Set a memory pressure callback for adaptive sizing.
    ///
    /// The callback is invoked on each [`put`](TieredCache::put). Return
    /// `None` to skip adaptation, or a [`MemoryPressure`] level to trigger
    /// size adjustment.
    pub fn set_pressure_fn(&mut self, f: impl Fn() -> Option<MemoryPressure> + 'static) {
        self.pressure_fn = Some(Box::new(f));
    }

    /// Retrieve a value with access tracking.
    ///
    /// `last_key` is the previously accessed key (if any), used to train the
    /// access predictor. Returns `None` on miss.
    pub fn get(&mut self, key: &str, last_key: Option<&str>, now: Instant) -> Option<V> {
        // Tiger Style: Precondition assertions
        debug_assert!(!key.is_empty(), "key cannot be empty");
        debug_assert!(self.entries.len() <= MAX_CACHE_ENTRIES as usize, "entries count exceeds maximum");

        self.access_predictor.record_access(last_key, key);

        let (data, cost_ms) = if let Some(entry) = self.entries.get_mut(key) {
            entry.record_access(now);

            let new_tier = self.tier_thresholds.determine_tier(entry, now);
            if new_tier != entry.tier {
                if new_tier < entry.tier {
                    self.stats.record_promotion();
                } else {
                    self.stats.record_demotion();
                }
                entry.tier = new_tier;
            }

            (Some(entry.get_data()), entry.computation_cost_ms)
        } else {
            self.stats.record_miss();
            return None;
        };

        self.stats.record_hit(cost_ms);
        
        // Tiger Style: Postcondition assertion
        debug_assert!(data.is_some(), "data should be Some when hit is recorded");
        
        data
    }

    /// Store a value with metadata.
    ///
    /// `size_bytes` is the caller's estimate of value size (used for eviction
    /// budgeting). `computation_cost_ms` records how long the value took to
    /// produce — expensive entries resist eviction.
    ///
    /// Returns `Err` if eviction cannot free enough space.
    pub fn put(
        &mut self,
        key: String,
        data: V,
        size_bytes: u64,
        computation_cost_ms: u64,
        priority: CachePriority,
        now: Instant,
    ) -> Result<(), String> {
        // Tiger Style: Precondition assertions
        debug_assert!(!key.is_empty(), "key cannot be empty");
        debug_assert!(size_bytes > 0, "size_bytes must be greater than zero");
        debug_assert!(self.entries.len() <= MAX_CACHE_ENTRIES as usize, "entries count exceeds maximum");

        let initial_size = self.current_size_bytes();

        if !self.entries.contains_key(&key) && self.entries.len() >= MAX_CACHE_ENTRIES as usize {
            self.evict_oldest_entry()?;
        }

        self.evict_if_needed(size_bytes, now)?;
        self.adapt_cache_size();

        let entry = CacheEntry::new(data, size_bytes, computation_cost_ms, priority, now);
        self.entries.insert(key, entry);
        self.current_size_bytes
            .fetch_add(size_bytes, Ordering::Relaxed);

        // Tiger Style: Postcondition assertions
        debug_assert!(self.current_size_bytes() >= initial_size, "current size should not decrease");
        debug_assert!(self.entries.len() <= MAX_CACHE_ENTRIES as usize, "entry count still within bounds");

        Ok(())
    }

    /// Snapshot current statistics.
    pub fn stats(&self) -> CacheStats {
        // Tiger Style: Precondition assertions
        debug_assert!(self.entries.len() <= MAX_CACHE_ENTRIES as usize, "entries count exceeds maximum");

        let hits = self.stats.total_hits.load(Ordering::Relaxed);
        let misses = self.stats.total_misses.load(Ordering::Relaxed);
        let total = hits + misses;

        let stats = CacheStats {
            total_entries: self.entries.len() as u32,
            total_size_bytes: self.current_size_bytes.load(Ordering::Relaxed),
            max_size_bytes: self.max_size_bytes.load(Ordering::Relaxed),
            hit_rate: if total > 0 { hits as f64 / total as f64 } else { 0.0 },
            total_hits: hits,
            total_misses: misses,
            total_evictions: self.stats.total_evictions.load(Ordering::Relaxed),
            time_saved_ms: self.stats.time_saved_ms.load(Ordering::Relaxed),
            tier_distribution: self.tier_distribution(),
            avg_computation_cost_ms: self.avg_computation_cost(),
            memory_efficiency: self.memory_efficiency(),
        };

        // Tiger Style: Postcondition assertions
        debug_assert!(stats.hit_rate >= 0.0 && stats.hit_rate <= 1.0, "hit rate should be between 0 and 1");
        debug_assert!(stats.total_entries as usize == self.entries.len(), "entry count should match actual entries");

        stats
    }

    /// Suggest optimizations based on current statistics.
    pub fn suggest_optimizations(&self) -> Vec<String> {
        self.stats().suggest_optimizations()
    }

    /// Predict next likely keys given the current key.
    pub fn predict_next(&self, current: &str) -> Vec<(String, f64)> {
        self.access_predictor.predict_next(current)
    }

    /// Remove all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_size_bytes.store(0, Ordering::Relaxed);
    }

    /// Number of cached entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Current total size in bytes.
    pub fn current_size_bytes(&self) -> u64 {
        self.current_size_bytes.load(Ordering::Relaxed)
    }

    /// Maximum size budget in bytes.
    pub fn max_size_bytes(&self) -> u64 {
        self.max_size_bytes.load(Ordering::Relaxed)
    }

    /// Direct entry access (for testing / introspection).
    pub fn get_entry(&self, key: &str) -> Option<&CacheEntry<V>> {
        self.entries.get(key)
    }

    // ── internals ──────────────────────────────────────────────────────

    fn evict_if_needed(&mut self, needed_bytes: u64, now: Instant) -> Result<(), String> {
        // Tiger Style: Precondition assertions
        debug_assert!(needed_bytes > 0, "needed_bytes must be greater than zero");
        debug_assert!(self.max_size_bytes() >= MIN_CACHE_SIZE, "max_size_bytes below minimum");

        let max_size = self.max_size_bytes.load(Ordering::Relaxed);
        let current_size = self.current_size_bytes.load(Ordering::Relaxed);
        let initial_entries = self.entries.len();

        if current_size + needed_bytes <= max_size {
            return Ok(());
        }

        let mut candidates: Vec<(String, f64, u64)> = self
            .entries
            .iter()
            .filter(|(_, entry)| {
                now.duration_since(entry.created_at).as_secs()
                    >= self.eviction_policy.min_age_seconds
            })
            .map(|(key, entry)| {
                (
                    key.clone(),
                    calculate_eviction_score(&self.eviction_policy, entry, now),
                    entry.size_bytes,
                )
            })
            .collect();

        candidates.sort_by(|a, b| a.1.total_cmp(&b.1));

        let mut freed: u64 = 0;
        let target = needed_bytes + (max_size / 10);

        for (key, _score, size) in candidates {
            if freed >= target {
                break;
            }
            if self.entries.remove(&key).is_some() {
                freed += size;
                self.current_size_bytes.fetch_sub(size, Ordering::Relaxed);
                self.stats.record_eviction();
            }
        }

        // Tiger Style: Postcondition assertions
        debug_assert!(self.entries.len() <= initial_entries, "entry count should not increase during eviction");
        debug_assert!(freed == 0 || self.current_size_bytes() < current_size, "size should decrease if entries were freed");

        if freed < needed_bytes {
            Err(format!(
                "Could not free enough space. Needed: {needed_bytes}, Freed: {freed}"
            ))
        } else {
            Ok(())
        }
    }

    fn evict_oldest_entry(&mut self) -> Result<(), String> {
        // Tiger Style: Precondition assertions
        debug_assert!(self.entries.len() <= MAX_CACHE_ENTRIES as usize, "entries count exceeds maximum");

        let initial_count = self.entries.len();
        let initial_size = self.current_size_bytes();

        let oldest_key = self
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.last_access)
            .map(|(key, _)| key.clone());

        if let Some(key) = oldest_key {
            if let Some(entry) = self.entries.remove(&key) {
                self.current_size_bytes
                    .fetch_sub(entry.size_bytes, Ordering::Relaxed);
                self.stats.record_eviction();

                // Tiger Style: Postcondition assertions
                debug_assert!(self.entries.len() < initial_count, "entry count should decrease");
                debug_assert!(self.current_size_bytes() < initial_size, "size should decrease");

                Ok(())
            } else {
                Err("Failed to remove oldest entry".to_string())
            }
        } else {
            // Tiger Style: No entries to evict
            debug_assert!(self.entries.is_empty(), "if no oldest key found, cache should be empty");
            Ok(())
        }
    }

    fn adapt_cache_size(&mut self) {
        // Tiger Style: Precondition assertions
        debug_assert!(self.max_size_bytes() >= MIN_CACHE_SIZE, "max size below minimum cache size");

        let pressure = self.pressure_fn.as_ref().and_then(|f| f());
        let Some(pressure) = pressure else { return };

        let current_max = self.max_size_bytes.load(Ordering::Relaxed);

        let new_max = match pressure {
            MemoryPressure::Low => current_max.saturating_add(current_max / 10),
            MemoryPressure::Medium => current_max,
            MemoryPressure::High => current_max
                .saturating_sub(current_max / 10)
                .max(MIN_CACHE_SIZE),
            MemoryPressure::Critical => (current_max / 2).max(MIN_CACHE_SIZE),
        };

        self.max_size_bytes.store(new_max, Ordering::Relaxed);

        // Tiger Style: Postcondition assertions
        debug_assert!(self.max_size_bytes() >= MIN_CACHE_SIZE, "new max size still above minimum");
        debug_assert!(new_max >= MIN_CACHE_SIZE, "calculated new_max respects minimum");
    }

    fn tier_distribution(&self) -> HashMap<String, u32> {
        // Tiger Style: Precondition assertions
        debug_assert!(self.entries.len() <= MAX_CACHE_ENTRIES as usize, "entries count exceeds maximum");

        let mut distribution = HashMap::with_capacity(3);
        distribution.insert("hot".to_string(), 0);
        distribution.insert("warm".to_string(), 0);
        distribution.insert("cold".to_string(), 0);

        for entry in self.entries.values() {
            match entry.tier {
                CacheTier::Hot => *distribution.entry("hot".to_string()).or_insert(0) += 1,
                CacheTier::Warm => *distribution.entry("warm".to_string()).or_insert(0) += 1,
                CacheTier::Cold => *distribution.entry("cold".to_string()).or_insert(0) += 1,
            }
        }

        // Tiger Style: Postcondition assertions
        let total_distributed: u32 = distribution.values().sum();
        debug_assert_eq!(total_distributed, self.entries.len() as u32, "all entries should be distributed across tiers");

        distribution
    }

    fn avg_computation_cost(&self) -> f64 {
        // Tiger Style: Precondition assertions
        debug_assert!(self.entries.len() <= MAX_CACHE_ENTRIES as usize, "entries count exceeds maximum");

        if self.entries.is_empty() {
            return 0.0;
        }
        let total: u64 = self.entries.values().map(|e| e.computation_cost_ms).sum();
        let average = total as f64 / self.entries.len() as f64;

        // Tiger Style: Postcondition assertions
        debug_assert!(average >= 0.0, "average cost should be non-negative");

        average
    }

    fn memory_efficiency(&self) -> f64 {
        // Tiger Style: Precondition assertions
        debug_assert!(self.entries.len() <= MAX_CACHE_ENTRIES as usize, "entries count exceeds maximum");

        let total_value: f64 = self
            .entries
            .values()
            .map(|e| e.computation_cost_ms as f64 * e.access_count as f64)
            .sum();

        let total_bytes = self.current_size_bytes.load(Ordering::Relaxed) as f64;
        let efficiency = if total_bytes > 0.0 { total_value / total_bytes } else { 0.0 };

        // Tiger Style: Postcondition assertions
        debug_assert!(efficiency >= 0.0, "efficiency should be non-negative");

        efficiency
    }
}
