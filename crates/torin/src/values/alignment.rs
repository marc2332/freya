#[derive(PartialEq, Clone, Debug, Default)]
pub enum Alignment {
    #[default]
    Start,
    Center,
    End,
}

impl Alignment {
    pub fn is_not_start(&self) -> bool {
        match self {
            Self::Start => false,
            _ => true,
        }
    }
}
