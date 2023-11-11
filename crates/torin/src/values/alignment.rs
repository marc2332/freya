#[derive(PartialEq, Clone, Debug, Default)]
pub enum Alignment {
    #[default]
    Start,
    Center,
    End,
}

impl Alignment {
    pub fn is_not_start(&self) -> bool {
        *self != Self::Start
    }

    pub fn pretty(&self) -> String {
        match self {
            Alignment::Start => "start".to_string(),
            Alignment::Center => "center".to_string(),
            Alignment::End => "end".to_string(),
        }
    }
}
