#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub enum Alignment {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceEvenly,
    SpaceAround,
}

impl Alignment {
    pub const fn is_not_start(&self) -> bool {
        !matches!(self, Self::Start)
    }

    pub const fn is_spaced(&self) -> bool {
        matches!(
            self,
            Self::SpaceBetween | Self::SpaceAround | Self::SpaceEvenly
        )
    }

    pub fn pretty(&self) -> String {
        match self {
            Self::Start => "start".to_string(),
            Self::Center => "center".to_string(),
            Self::End => "end".to_string(),
            Self::SpaceBetween => "space-between".to_string(),
            Self::SpaceEvenly => "space-evenly".to_string(),
            Self::SpaceAround => "space-around".to_string(),
        }
    }
}
