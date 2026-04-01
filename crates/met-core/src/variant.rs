/// Visual variant for components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Ghost,
}

impl Variant {
    pub fn class(&self) -> &'static str {
        match self {
            Variant::Default => "met-default",
            Variant::Primary => "met-primary",
            Variant::Secondary => "met-secondary",
            Variant::Success => "met-success",
            Variant::Warning => "met-warning",
            Variant::Danger => "met-danger",
            Variant::Ghost => "met-ghost",
        }
    }
}
