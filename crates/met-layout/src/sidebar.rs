use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SidebarProps {
    /// Width when expanded. Defaults to 250px.
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
    let collapsed = if props.collapsed { "met-sidebar-collapsed" } else { "" };
    let w = if props.collapsed { "0px".to_string() } else { props.width.clone() };

    rsx! {
        aside {
            class: "met-sidebar {collapsed} {props.class}",
            style: "width: {w};",
            {props.children}
        }
    }
}
