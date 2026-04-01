//! Cache statistics tracking and reporting.
//!
//! Two types:
//! - [`CacheStatistics`] — live atomic counters updated during cache operations
//! - [`CacheStats`] — immutable snapshot for reporting and analysis

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Live atomic counters for cache operations.
///
/// All operations use `Relaxed` ordering — stats are advisory, not
/// synchronization primitives.
pub struct CacheStatistics {
    pub total_hits: AtomicU64,
    pub total_misses: AtomicU64,
    pub total_evictions: AtomicU64,
    pub bytes_saved: AtomicU64,
    pub time_saved_ms: AtomicU64,
    pub tier_promotions: AtomicU64,
    pub tier_demotions: AtomicU64,
}

impl CacheStatistics {
    pub fn new() -> Self {
        Self {
            total_hits: AtomicU64::new(0),
            total_misses: AtomicU64::new(0),
            total_evictions: AtomicU64::new(0),
            bytes_saved: AtomicU64::new(0),
            time_saved_ms: AtomicU64::new(0),
            tier_promotions: AtomicU64::new(0),
            tier_demotions: AtomicU64::new(0),
        }
    }

    pub fn record_hit(&self, time_saved_ms: u64) {
        self.total_hits.fetch_add(1, Ordering::Relaxed);
        self.time_saved_ms
            .fetch_add(time_saved_ms, Ordering::Relaxed);
    }

    pub fn record_miss(&self) {
        self.total_misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_eviction(&self) {
        self.total_evictions.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_promotion(&self) {
        self.tier_promotions.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_demotion(&self) {
        self.tier_demotions.fetch_add(1, Ordering::Relaxed);
    }

    /// Current hit rate (0.0–1.0). Returns 0.0 if no accesses.
    pub fn hit_rate(&self) -> f64 {
        let hits = self.total_hits.load(Ordering::Relaxed);
        let total = hits + self.total_misses.load(Ordering::Relaxed);
        if total > 0 { hits as f64 / total as f64 } else { 0.0 }
    }
}

impl Default for CacheStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// Immutable statistics snapshot.
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: u32,
    pub total_size_bytes: u64,
    pub max_size_bytes: u64,
    pub hit_rate: f64,
    pub total_hits: u64,
    pub total_misses: u64,
    pub total_evictions: u64,
    pub time_saved_ms: u64,
    pub tier_distribution: HashMap<String, u32>,
    pub avg_computation_cost_ms: f64,
    pub memory_efficiency: f64,
}

impl std::fmt::Display for CacheStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cache Stats: {} entries, {:.1}MB / {:.1}MB, Hit rate: {:.1}%, Time saved: {:.1}s",
            self.total_entries,
            self.total_size_bytes as f64 / (1024.0 * 1024.0),
            self.max_size_bytes as f64 / (1024.0 * 1024.0),
            self.hit_rate * 100.0,
            self.time_saved_ms as f64 / 1000.0
        )
    }
}

impl CacheStats {
    /// Suggest optimizations based on current stats.
    ///
    /// Returns at most 9 lines of human-readable advice grouped by category.
    pub fn suggest_optimizations(&self) -> Vec<String> {
        let mut suggestions = Vec::with_capacity(9);

        if self.hit_rate < 0.5 {
            suggestions.push("[WARN] Low cache hit rate. Consider:".to_string());
            suggestions.push("  - Increasing cache size".to_string());
            suggestions.push("  - Adjusting eviction weights".to_string());
        }

        if self.memory_efficiency < 0.1 {
            suggestions.push("[INFO] Low memory efficiency. Consider:".to_string());
            suggestions.push("  - Caching smaller, more frequently accessed items".to_string());
            suggestions.push("  - Using compression for large entries".to_string());
        }

        let cold_count = *self.tier_distribution.get("cold").unwrap_or(&0);
        if self.total_entries > 0 {
            let cold_ratio = cold_count as f64 / self.total_entries as f64;
            if cold_ratio > 0.5 {
                suggestions.push("[COLD] Many cold entries. Consider:".to_string());
                suggestions.push("  - More aggressive eviction of cold entries".to_string());
                suggestions.push("  - Implementing disk-based cold storage".to_string());
            }
        }

        suggestions
    }
}
