#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub enum Alignment {
    /// Align children to the start of the axis. This is the default.
    #[default]
    Start,
    /// Align children to the center of the axis.
    Center,
    /// Align children to the end of the axis.
    End,
    /// Distribute children with equal space between them, no space at the edges.
    SpaceBetween,
    /// Distribute children with equal space between and around them, including the edges.
    SpaceEvenly,
    /// Distribute children with equal space around them, half-size space at the edges.
    SpaceAround,
}

impl Alignment {
    /// Use a [`Start`](Alignment::Start) alignment.
    pub fn start() -> Alignment {
        Alignment::Start
    }

    /// Use a [`Center`](Alignment::Center) alignment.
    pub fn center() -> Alignment {
        Alignment::Center
    }

    /// Use an [`End`](Alignment::End) alignment.
    pub fn end() -> Alignment {
        Alignment::End
    }

    /// Use a [`SpaceBetween`](Alignment::SpaceBetween) alignment.
    pub fn space_between() -> Alignment {
        Alignment::SpaceBetween
    }

    /// Use a [`SpaceEvenly`](Alignment::SpaceEvenly) alignment.
    pub fn space_evenly() -> Alignment {
        Alignment::SpaceEvenly
    }

    /// Use a [`SpaceAround`](Alignment::SpaceAround) alignment.
    pub fn space_around() -> Alignment {
        Alignment::SpaceAround
    }

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
