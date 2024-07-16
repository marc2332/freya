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

impl Content {
    pub fn pretty(&self) -> String {
        match self {
            Self::Normal => "normal".to_owned(),
            Self::Fit => "fit".to_owned(),
        }
    }
}
