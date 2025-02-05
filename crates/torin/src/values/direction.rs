#[derive(PartialEq, Eq, Clone, Debug, Default)]
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
