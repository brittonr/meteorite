use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Persist a signal's value to browser localStorage.
///
/// Reads the initial value from localStorage on mount. Writes back
/// whenever the signal is updated.
pub fn use_local_storage<T>(key: &str, default: T) -> Signal<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Clone + 'static,
{
    let key = key.to_string();
    use_signal(move || {
        // In a real implementation this would read from window.localStorage
        // via web-sys or dioxus eval. Placeholder for now.
        let _ = &key;
        default.clone()
    })
}
