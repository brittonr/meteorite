/// Component sizing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl Size {
    pub fn class(&self) -> &'static str {
        match self {
            Size::Xs => "met-xs",
            Size::Sm => "met-sm",
            Size::Md => "met-md",
            Size::Lg => "met-lg",
            Size::Xl => "met-xl",
        }
    }
}
