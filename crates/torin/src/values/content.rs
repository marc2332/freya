#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub enum Content {
    #[default]
    Normal,
    Fit,
    Flex,
}

impl Content {
    pub fn is_fit(&self) -> bool {
        self == &Self::Fit
    }

    pub fn is_flex(&self) -> bool {
        self == &Self::Flex
    }
}

impl Content {
    pub fn pretty(&self) -> String {
        match self {
            Self::Normal => "normal".to_owned(),
            Self::Fit => "fit".to_owned(),
            Self::Flex => "flex".to_owned(),
        }
    }
}
