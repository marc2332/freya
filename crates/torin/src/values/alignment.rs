#[derive(PartialEq, Clone, Debug, Default)]
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
    pub fn is_not_start(&self) -> bool {
        *self != Self::Start
    }

    pub fn is_spaced(&self) -> bool {
        matches!(
            self,
            Self::SpaceBetween | Self::SpaceAround | Self::SpaceEvenly
        )
    }

    pub fn pretty(&self) -> String {
        match self {
            Alignment::Start => "start".to_string(),
            Alignment::Center => "center".to_string(),
            Alignment::End => "end".to_string(),
            Alignment::SpaceBetween => "space-between".to_string(),
            Alignment::SpaceEvenly => "space-evenly".to_string(),
            Alignment::SpaceAround => "space-around".to_string(),
        }
    }
}
