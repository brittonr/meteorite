//! Searchable select with dropdown filtering.

use dioxus::prelude::*;
use met_core::Size;

#[derive(Props, Clone, PartialEq)]
pub struct SearchableSelectProps {
    /// Current selected value
    pub value: String,
    /// Called when the selection changes
    pub on_change: EventHandler<String>,
    /// Available options
    pub options: Vec<String>,
    /// Placeholder text
    #[props(default = "Select…".to_string())]
    pub placeholder: String,
    /// Whether the component is disabled
    #[props(default)]
    pub disabled: bool,
    /// Size variant
    #[props(default)]
    pub size: Size,
    #[props(default)]
    pub class: String,
}

/// Filterable dropdown select.
///
/// Type to filter the option list; press Enter to pick the first match,
/// Escape to cancel.
///
/// ```rust,ignore
/// SearchableSelect {
///     value: selected(),
///     options: vec!["Alpha".into(), "Beta".into(), "Gamma".into()],
///     on_change: move |v| selected.set(v),
/// }
/// ```
#[component]
pub fn SearchableSelect(props: SearchableSelectProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut search_term = use_signal(String::new);

    // Sync search term with value when closed
    use_effect({
        let value = props.value.clone();
        move || {
            if !is_open() {
                search_term.set(value.clone());
            }
        }
    });

    let filtered = filter_options(&props.options, &search_term());

    rsx! {
        div {
            class: "met-searchable-select {props.size.class()} {props.class}",

            // Input field
            input {
                class: "met-searchable-select-input",
                r#type: "text",
                value: "{search_term}",
                placeholder: "{props.placeholder}",
                disabled: props.disabled,
                autocomplete: "off",

                onfocus: move |_| { is_open.set(true); },

                onblur: move |_| {
                    // Small delay so click on option registers before close
                    spawn(async move {
                        #[cfg(target_arch = "wasm32")]
                        {
                            gloo_timers::future::TimeoutFuture::new(150).await;
                        }
                        is_open.set(false);
                    });
                },

                oninput: move |evt| {
                    search_term.set(evt.value());
                    is_open.set(true);
                },

                onkeydown: {
                    let filtered_for_key = filtered.clone();
                    let value_for_key = props.value.clone();
                    move |evt: KeyboardEvent| {
                        match evt.key() {
                            Key::Escape => {
                                is_open.set(false);
                                search_term.set(value_for_key.clone());
                            }
                            Key::Enter => {
                                if !filtered_for_key.is_empty() {
                                    let pick = if filtered_for_key.contains(&search_term()) {
                                        search_term()
                                    } else {
                                        filtered_for_key[0].clone()
                                    };
                                    props.on_change.call(pick.clone());
                                    search_term.set(pick);
                                    is_open.set(false);
                                }
                                evt.prevent_default();
                            }
                            Key::ArrowDown => {
                                if !is_open() { is_open.set(true); }
                                evt.prevent_default();
                            }
                            _ => {}
                        }
                    }
                },
            }

            // Dropdown
            if is_open() && !props.disabled {
                div {
                    class: "met-searchable-select-dropdown",
                    if filtered.is_empty() {
                        div { class: "met-searchable-select-empty", "No options found" }
                    } else {
                        for option in filtered.iter() {
                            {
                                let opt = option.clone();
                                rsx! {
                                    div {
                                        class: "met-searchable-select-option",
                                        onmousedown: move |_| {
                                            props.on_change.call(opt.clone());
                                            search_term.set(opt.clone());
                                            is_open.set(false);
                                        },
                                        {option.as_str()}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Pure filter — case-insensitive substring match.
fn filter_options(options: &[String], term: &str) -> Vec<String> {
    if term.is_empty() {
        return options.to_vec();
    }
    let lower = term.to_lowercase();
    options
        .iter()
        .filter(|o| o.to_lowercase().contains(&lower))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_empty_term_returns_all() {
        let opts = vec!["A".into(), "B".into()];
        assert_eq!(filter_options(&opts, ""), opts);
    }

    #[test]
    fn filter_narrows_results() {
        let opts = vec!["Apple".into(), "Banana".into(), "Apricot".into()];
        let result = filter_options(&opts, "ap");
        assert_eq!(result, vec!["Apple".to_string(), "Apricot".to_string()]);
    }

    #[test]
    fn filter_no_match_returns_empty() {
        let opts = vec!["Apple".into(), "Banana".into()];
        assert!(filter_options(&opts, "xyz").is_empty());
    }
}
