#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub enum Content {
    #[default]
    Normal,
    Fit,
    Flex,
    Wrap,
}

impl Content {
    pub fn is_fit(&self) -> bool {
        self == &Self::Fit
    }

    pub fn is_flex(&self) -> bool {
        self == &Self::Flex
    }

    pub fn is_wrap(&self) -> bool {
        self == &Self::Wrap
    }

    pub fn allows_alignments(&self) -> bool {
        matches!(self, Self::Normal | Self::Flex | Self::Fit)
    }
}

impl Content {
    pub fn pretty(&self) -> String {
        match self {
            Self::Normal => "normal".to_owned(),
            Self::Fit => "fit".to_owned(),
            Self::Flex => "flex".to_owned(),
            Self::Wrap => "wrap".to_owned(),
        }
    }
}
