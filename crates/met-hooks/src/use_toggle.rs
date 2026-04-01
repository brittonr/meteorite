use dioxus::prelude::*;

/// A boolean signal with a toggle callback.
pub fn use_toggle(initial: bool) -> (Signal<bool>, impl Fn()) {
    let mut value = use_signal(|| initial);
    let toggle = move || {
        let current = *value.read();
        value.set(!current);
    };
    (value, toggle)
}
