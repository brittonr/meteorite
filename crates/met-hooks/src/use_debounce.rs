use dioxus::prelude::*;

/// Returns a debounced signal.
///
/// `set` updates the pending value. The signal only propagates once
/// `flush` is called (typically on a timer from the caller's side).
pub fn use_debounce<T: Clone + PartialEq + 'static>(initial: T) -> UseDebounce<T> {
    let value = use_signal(|| initial.clone());
    let pending = use_signal(|| initial);

    UseDebounce { value, pending }
}

/// Handle returned by [`use_debounce`].
#[derive(Clone, Copy)]
pub struct UseDebounce<T: 'static> {
    pub value: Signal<T>,
    pending: Signal<T>,
}

impl<T: Clone + PartialEq + 'static> UseDebounce<T> {
    /// Stage a new value without immediately propagating it.
    pub fn set(&mut self, new_val: T) {
        self.pending.set(new_val);
    }

    /// Propagate the pending value to the output signal.
    pub fn flush(&mut self) {
        let current = self.pending.read().clone();
        if *self.value.read() != current {
            self.value.set(current);
        }
    }
}
