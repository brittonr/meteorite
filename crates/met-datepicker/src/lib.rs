//! Calendar date picker and time input components for Dioxus.
//!
//! # Usage
//!
//! ```rust,ignore
//! use met_datepicker::{DatePicker, CalDate};
//!
//! rsx! {
//!     DatePicker {
//!         value: CalDate::new(2026, 4, 1),
//!         on_change: move |d: CalDate| log::info!("selected {d:?}"),
//!     }
//! }
//! ```

pub mod calendar;
pub mod time_input;
pub mod datepicker;

pub use calendar::CalDate;
pub use datepicker::DatePicker;
pub use time_input::TimeInput;
