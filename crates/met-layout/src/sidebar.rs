use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SidebarProps {
    #[props(default = "250px".to_string())]
    pub width: String,
    #[props(default = false)]
    pub collapsed: bool,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    let w = if props.collapsed {
        "0px".to_string()
    } else {
        props.width.clone()
    };
    let class = format!(
        "met-sidebar {} {}",
        if props.collapsed { "met-sidebar-collapsed" } else { "" },
        props.class
    );

    rsx! {
        aside {
            class: "{class}",
            style: "width: {w}; overflow: hidden; transition: width 0.2s;",
            {props.children}
        }
    }
}
