#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Colour {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
    Light,
    Dark,
}

impl Colour {
    pub fn text_background_class(self) -> &'static str {
        match self {
            Self::Primary => "text-bg-primary",
            Self::Secondary => "text-bg-secondary",
            Self::Success => "text-bg-success",
            Self::Danger => "text-bg-danger",
            Self::Warning => "text-bg-warning",
            Self::Info => "text-bg-info",
            Self::Light => "text-bg-light",
            Self::Dark => "text-bg-dark",
        }
    }
}
