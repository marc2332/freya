#[derive(PartialEq, Clone, Debug, Copy, Default)]
pub enum DisplayMode {
    #[default]
    Normal,
    Center,
}

impl DisplayMode {
    pub fn pretty(&self) -> String {
        match self {
            DisplayMode::Normal => "Normal".to_string(),
            DisplayMode::Center => "Center".to_string(),
        }
    }
}
