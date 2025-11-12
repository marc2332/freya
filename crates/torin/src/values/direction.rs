#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Clone, Debug, Default, Copy)]
pub enum Direction {
    #[default]
    Vertical,
    Horizontal,
}

impl Direction {
    pub fn pretty(&self) -> String {
        match self {
            Self::Horizontal => "horizontal".to_string(),
            Self::Vertical => "vertical".to_string(),
        }
    }
}
