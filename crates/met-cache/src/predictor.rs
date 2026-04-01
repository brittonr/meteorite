//! Markov chain access predictor for cache warming.
//!
//! Learns transition patterns between cache keys (A → B seen 8 times,
//! A → C seen 2 times) and predicts the next likely access given the
//! current key. Predictions that don't meet the confidence threshold
//! are filtered out.
//!
//! The predictor is bounded: at most [`MAX_PREDICTOR_PATTERNS`] source keys
//! and [`MAX_TRANSITIONS_PER_KEY`] transitions per key. When full, new
//! patterns are silently dropped.

use std::collections::HashMap;

/// Maximum number of unique source keys tracked.
pub const MAX_PREDICTOR_PATTERNS: u32 = 1_000;
/// Maximum number of transitions stored per source key.
pub const MAX_TRANSITIONS_PER_KEY: u32 = 100;

const INITIAL_CAPACITY: usize = 64;

/// Markov chain access predictor.
///
/// Records `(from_key, to_key)` transitions and predicts which key is
/// likely to be accessed next given the current key.
pub struct AccessPredictor {
    /// from_key → (to_key → count)
    patterns: HashMap<String, HashMap<String, u32>>,
    /// Minimum confidence (0.0–1.0) for a prediction to be returned
    confidence_threshold: f64,
}

impl AccessPredictor {
    /// Create a predictor with the default 0.7 confidence threshold.
    pub fn new() -> Self {
        Self {
            patterns: HashMap::with_capacity(INITIAL_CAPACITY),
            confidence_threshold: 0.7,
        }
    }

    /// Create a predictor with a custom confidence threshold (clamped to 0.0–1.0).
    pub fn with_confidence_threshold(threshold: f64) -> Self {
        Self {
            patterns: HashMap::with_capacity(INITIAL_CAPACITY),
            confidence_threshold: threshold.clamp(0.0, 1.0),
        }
    }

    /// Record a transition from `from` to `to`.
    ///
    /// If `from` is `None`, no transition is recorded. Bounded at
    /// [`MAX_PREDICTOR_PATTERNS`] source keys and [`MAX_TRANSITIONS_PER_KEY`]
    /// transitions per key.
    pub fn record_access(&mut self, from: Option<&str>, to: &str) {
        let Some(from_key) = from else { return };

        if !self.patterns.contains_key(from_key)
            && self.patterns.len() >= MAX_PREDICTOR_PATTERNS as usize
        {
            return;
        }

        let transitions = self
            .patterns
            .entry(from_key.to_string())
            .or_insert_with(|| HashMap::with_capacity(16));

        if !transitions.contains_key(to)
            && transitions.len() >= MAX_TRANSITIONS_PER_KEY as usize
        {
            return;
        }

        *transitions.entry(to.to_string()).or_insert(0) =
            transitions.get(to).copied().unwrap_or(0).saturating_add(1);
    }

    /// Predict the next likely keys given `current`.
    ///
    /// Returns `(key, confidence)` pairs sorted by confidence descending,
    /// filtered to the configured threshold.
    pub fn predict_next(&self, current: &str) -> Vec<(String, f64)> {
        self.predictions(current, true)
    }

    /// Return all predictions for `current`, ignoring the confidence threshold.
    pub fn predict_all(&self, current: &str) -> Vec<(String, f64)> {
        self.predictions(current, false)
    }

    /// Clear all recorded patterns.
    pub fn clear(&mut self) {
        self.patterns.clear();
    }

    /// Number of unique source keys being tracked.
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    fn predictions(&self, current: &str, filter: bool) -> Vec<(String, f64)> {
        let Some(transitions) = self.patterns.get(current) else {
            return Vec::new();
        };

        let total: u32 = transitions.values().sum();
        if total == 0 {
            return Vec::new();
        }

        let threshold = if filter { self.confidence_threshold } else { 0.0 };

        let mut predictions: Vec<_> = transitions
            .iter()
            .map(|(next, count)| (next.clone(), *count as f64 / total as f64))
            .filter(|(_, conf)| *conf >= threshold)
            .collect();

        predictions.sort_by(|a, b| b.1.total_cmp(&a.1));
        predictions
    }
}

impl Default for AccessPredictor {
    fn default() -> Self {
        Self::new()
    }
}
