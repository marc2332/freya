use crate::prelude::Length;

#[derive(PartialEq, Clone, Debug)]
pub enum VisibleSize {
    Full,
    InnerPercentage(Length),
}

impl Default for VisibleSize {
    fn default() -> Self {
        Self::Full
    }
}

impl VisibleSize {
    pub fn pretty(&self) -> String {
        match self {
            Self::Full => "full".to_string(),
            Self::InnerPercentage(p) => format!("{}%", p.get()),
        }
    }
}
