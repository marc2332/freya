#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub enum FlexWrap {
    #[default]
    NoWrap,
    Wrap
}

impl FlexWrap {
    pub fn is_nowrap(&self) -> bool {
        self == &Self::NoWrap
    }

    pub fn is_wrap(&self) -> bool {
        self == &Self::Wrap
    }
}

impl FlexWrap {
    pub fn pretty(&self) -> String {
        match self {
            Self::NoWrap => "no-wrap".to_owned(),
            Self::Wrap => "wrap".to_owned()
        }
    }
}
