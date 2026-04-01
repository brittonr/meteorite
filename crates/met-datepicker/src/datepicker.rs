//! Calendar date picker Dioxus component.

use dioxus::prelude::*;

use crate::calendar::CalDate;

const DAY_HEADERS: [&str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];

#[derive(Props, Clone, PartialEq)]
pub struct DatePickerProps {
    /// Currently selected date.
    #[props(default)]
    pub value: Option<CalDate>,
    /// Called when a date is picked.
    pub on_change: EventHandler<CalDate>,
    /// Today's date (for highlighting). Defaults to 2026-04-01 if not set.
    #[props(default)]
    pub today: Option<CalDate>,
    /// Extra CSS class.
    #[props(default)]
    pub class: String,
}

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    let today = props.today.unwrap_or(CalDate::new(2026, 4, 1));
    let initial = props.value.unwrap_or(today);

    let mut display_year = use_signal(|| initial.year);
    let mut display_month = use_signal(|| initial.month);

    // Sync display month when value changes externally.
    use_effect({
        let val = props.value;
        move || {
            if let Some(v) = val {
                display_year.set(v.year);
                display_month.set(v.month);
            }
        }
    });

    let dy = *display_year.read();
    let dm = *display_month.read();
    let first_of_month = CalDate::new(dy, dm, 1);
    let days_in_month = first_of_month.days_in_month();
    let start_weekday = first_of_month.weekday(); // 0=Mon

    let month_label = format!("{} {}", first_of_month.month_name(), dy);

    let class = format!("met-datepicker {}", props.class);

    rsx! {
        div { class: "{class}",
            // Header: ◂ Month Year ▸
            div { class: "met-datepicker-header",
                button {
                    class: "met-datepicker-nav",
                    onclick: move |_| {
                        let prev = CalDate::new(dy, dm, 1).prev_month();
                        display_year.set(prev.year);
                        display_month.set(prev.month);
                    },
                    "◂"
                }
                span { class: "met-datepicker-title", "{month_label}" }
                button {
                    class: "met-datepicker-nav",
                    onclick: move |_| {
                        let next = CalDate::new(dy, dm, 1).next_month();
                        display_year.set(next.year);
                        display_month.set(next.month);
                    },
                    "▸"
                }
            }

            // Day-of-week headers
            div { class: "met-datepicker-grid",
                for dh in DAY_HEADERS.iter() {
                    span { class: "met-datepicker-weekday", "{dh}" }
                }

                // Leading blanks
                for _ in 0..start_weekday {
                    span { class: "met-datepicker-blank" }
                }

                // Day cells
                for d in 1..=days_in_month {
                    {
                        let date = CalDate::new(dy, dm, d);
                        let is_selected = props.value == Some(date);
                        let is_today = date == today;
                        let is_weekend = date.weekday() >= 5;

                        let mut cell_class = String::from("met-datepicker-day");
                        if is_selected { cell_class.push_str(" met-datepicker-day-selected"); }
                        if is_today { cell_class.push_str(" met-datepicker-day-today"); }
                        if is_weekend { cell_class.push_str(" met-datepicker-day-weekend"); }

                        let on_change = props.on_change.clone();

                        rsx! {
                            button {
                                class: "{cell_class}",
                                onclick: move |_| on_change.call(date),
                                "{d}"
                            }
                        }
                    }
                }
            }
        }
    }
}
