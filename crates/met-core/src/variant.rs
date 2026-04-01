/// Visual variant for components.
///
/// The built-in variants cover common UI patterns. Use `Custom` for
/// brand-specific or domain-specific colors registered via
/// [`Palette::extra`](crate::Palette).
///
/// ```rust,ignore
/// // Built-in
/// Button { variant: Variant::Primary }
///
/// // Custom — requires a matching palette entry
/// Button { variant: Variant::Custom("brand".into()) }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Ghost,
    /// User-defined variant. The string must match a key in
    /// `Palette::extra` and will emit the CSS class `met-{name}`.
    Custom(String),
}

impl Variant {
    /// CSS class name emitted on the DOM element.
    pub fn class(&self) -> String {
        match self {
            Self::Default => "met-default".into(),
            Self::Primary => "met-primary".into(),
            Self::Secondary => "met-secondary".into(),
            Self::Success => "met-success".into(),
            Self::Warning => "met-warning".into(),
            Self::Danger => "met-danger".into(),
            Self::Ghost => "met-ghost".into(),
            Self::Custom(name) => format!("met-{name}"),
        }
    }
}
