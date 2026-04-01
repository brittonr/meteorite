use dioxus::prelude::*;
use std::time::Duration;

/// Returns a signal that updates only after the given duration of inactivity.
///
/// Each call to `set` resets the timer. The signal value only updates
/// once the caller stops calling `set` for `delay` duration.
pub fn use_debounce<T: Clone + PartialEq + 'static>(
    initial: T,
    delay: Duration,
) -> (Signal<T>, impl Fn(T)) {
    let mut value = use_signal(|| initial.clone());
    let mut pending = use_signal(|| initial);

    let set = move |new_val: T| {
        pending.set(new_val.clone());
        spawn(async move {
            tokio::time::sleep(delay).await;
            let current = pending.read().clone();
            if *value.read() != current {
                value.set(current);
            }
        });
    };

    (value, set)
}
