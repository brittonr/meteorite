use dioxus::prelude::*;

/// A boolean signal with a toggle callback.
pub fn use_toggle(initial: bool) -> (Signal<bool>, impl Fn()) {
    let value = use_signal(|| initial);
    let toggle = move || {
        let current = *value.read();
        // Writing through a clone avoids &mut borrow on the closure
        let mut w = value;
        w.set(!current);
    };
    (value, toggle)
}
