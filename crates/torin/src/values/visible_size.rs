use crate::prelude::Length;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, PartialEq, Clone, Debug)]
pub enum VisibleSize {
    #[default]
    Full,
    InnerPercentage(Length),
}

impl VisibleSize {
    pub fn pretty(&self) -> String {
        match self {
            Self::Full => "full".to_string(),
            Self::InnerPercentage(p) => format!("{}%", p.get()),
        }
    }
}
