#[derive(PartialEq, Clone, Debug, Default)]
pub enum DirectionMode {
    #[default]
    Vertical,
    Horizontal,
}

impl DirectionMode {
    pub fn pretty(&self) -> String {
        match self {
            DirectionMode::Horizontal => "horizontal".to_string(),
            DirectionMode::Vertical => "vertical".to_string(),
        }
    }
}
