#[derive(PartialEq, Clone, Debug, Default)]
pub enum Content {
    #[default]
    Normal,
    Fit,
}

impl Content {
    pub fn is_fit(&self) -> bool {
        self == &Self::Fit
    }
}
