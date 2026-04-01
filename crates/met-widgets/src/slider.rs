use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SliderProps {
    pub value: f64,
    #[props(default = 0.0)]
    pub min: f64,
    #[props(default = 100.0)]
    pub max: f64,
    #[props(default = 1.0)]
    pub step: f64,
    #[props(default = false)]
    pub disabled: bool,
    #[props(default)]
    pub class: String,
    pub oninput: Option<EventHandler<FormEvent>>,
}

#[component]
pub fn Slider(props: SliderProps) -> Element {
    let class = format!("met-slider {}", props.class);

    rsx! {
        input {
            r#type: "range",
            class: "{class}",
            value: "{props.value}",
            min: "{props.min}",
            max: "{props.max}",
            step: "{props.step}",
            disabled: props.disabled,
            oninput: move |evt| {
                if let Some(handler) = &props.oninput {
                    handler.call(evt);
                }
            },
        }
    }
}
