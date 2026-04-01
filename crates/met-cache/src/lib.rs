//! Multi-tier cache with smart eviction, access prediction, and adaptive sizing.
//!
//! `met-cache` is a generic in-memory caching library that provides:
//!
//! - **Multi-factor eviction scoring** — weighted combination of recency, frequency,
//!   computation cost, and entry size determines what gets evicted first
//! - **Hot/Warm/Cold tiering** — entries are classified by access patterns and
//!   tier status influences eviction priority
//! - **Access pattern prediction** — Markov chain predictor learns key transition
//!   patterns for prefetch hints
//! - **Adaptive sizing** — optional memory pressure callback lets the cache grow
//!   or shrink based on system conditions
//! - **Comprehensive statistics** — atomic hit/miss/eviction counters with
//!   optimization suggestions
//!
//! The cache is generic over the value type `V: Clone`. Keys are `String`.
//!
//! # Example
//!
//! ```
//! use met_cache::{TieredCache, CachePriority};
//! use web_time::Instant;
//!
//! let mut cache: TieredCache<Vec<u8>> = TieredCache::new(100); // 100 MB
//!
//! let data = vec![1u8, 2, 3, 4];
//! let now = Instant::now();
//!
//! cache.put("key1".into(), data, 4, 50, CachePriority::Normal, now).unwrap();
//!
//! if let Some(value) = cache.get("key1", None, now) {
//!     assert_eq!(value, vec![1, 2, 3, 4]);
//! }
//!
//! let stats = cache.stats();
//! println!("{stats}"); // "Cache Stats: 1 entries, ..."
//! ```
//!
//! # Adaptive Sizing
//!
//! Wire in memory pressure feedback to let the cache resize itself:
//!
//! ```
//! use met_cache::{TieredCache, MemoryPressure};
//!
//! let mut cache: TieredCache<String> = TieredCache::new(100);
//! cache.set_pressure_fn(|| {
//!     // Your system memory check here
//!     Some(MemoryPressure::Low)
//! });
//! ```

pub mod cache;
pub mod entry;
pub mod eviction;
pub mod predictor;
pub mod statistics;

pub use cache::{TieredCache, MAX_CACHE_ENTRIES};
pub use entry::{CacheEntry, CachePriority, CacheTier, MAX_ACCESS_PATTERN_HISTORY};
pub use eviction::{calculate_eviction_score, EvictionPolicy, TierThresholds};
pub use predictor::{AccessPredictor, MAX_PREDICTOR_PATTERNS, MAX_TRANSITIONS_PER_KEY};
pub use statistics::{CacheStatistics, CacheStats};

/// Memory pressure levels for adaptive cache sizing.
///
/// When provided via [`TieredCache::set_pressure_fn`], the cache adjusts its
/// maximum size on each `put`:
///
/// - `Low` — grow 10%
/// - `Medium` — hold steady
/// - `High` — shrink 10%
/// - `Critical` — shrink 50%
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPressure {
    Low,
    Medium,
    High,
    Critical,
}
