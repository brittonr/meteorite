//! Custom Dioxus hooks: debounce, throttle, breakpoint, clipboard, local storage.

pub mod use_debounce;
pub mod use_local_storage;
pub mod use_toggle;

pub use use_debounce::use_debounce;
pub use use_local_storage::use_local_storage;
pub use use_toggle::use_toggle;
