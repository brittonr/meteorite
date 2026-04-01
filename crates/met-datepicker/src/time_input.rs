//! Time input component (HH:MM).

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TimeInputProps {
    /// Current hours value (0–23).
    #[props(default)]
    pub hours: u8,
    /// Current minutes value (0–59).
    #[props(default)]
    pub minutes: u8,
    /// Called when the time changes. Tuple is (hours, minutes).
    #[props(default)]
    pub on_change: Option<EventHandler<(u8, u8)>>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn TimeInput(props: TimeInputProps) -> Element {
    let mut hours = use_signal(|| props.hours.min(23));
    let mut minutes = use_signal(|| props.minutes.min(59));

    // Sync with external props.
    use_effect({
        let h = props.hours;
        let m = props.minutes;
        move || {
            hours.set(h.min(23));
            minutes.set(m.min(59));
        }
    });

    let emit = {
        let on_change = props.on_change.clone();
        move |h: u8, m: u8| {
            if let Some(ref handler) = on_change {
                handler.call((h, m));
            }
        }
    };

    let class = format!("met-time-input {}", props.class);

    rsx! {
        div { class: "{class}",
            input {
                class: "met-time-input-field met-time-input-hours",
                r#type: "number",
                min: "0",
                max: "23",
                value: "{hours:02}",
                oninput: {
                    let emit = emit.clone();
                    move |evt: FormEvent| {
                        if let Ok(h) = evt.value().parse::<u8>() {
                            let h = h.min(23);
                            hours.set(h);
                            emit(h, *minutes.read());
                        }
                    }
                },
            }
            span { class: "met-time-input-sep", ":" }
            input {
                class: "met-time-input-field met-time-input-minutes",
                r#type: "number",
                min: "0",
                max: "59",
                value: "{minutes:02}",
                oninput: {
                    let emit = emit.clone();
                    move |evt: FormEvent| {
                        if let Ok(m) = evt.value().parse::<u8>() {
                            let m = m.min(59);
                            minutes.set(m);
                            emit(*hours.read(), m);
                        }
                    }
                },
            }
        }
    }
}
