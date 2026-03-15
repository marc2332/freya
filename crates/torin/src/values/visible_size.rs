use crate::prelude::Length;

/// Controls the percentage of the measured size that will actually be used in layout,
/// regardless of the element's own size.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, PartialEq, Clone, Debug)]
pub enum VisibleSize {
    /// Use the full measured size. This is the default.
    #[default]
    Full,
    /// Only use a percentage of the measured size in layout.
    InnerPercentage(Length),
}

impl VisibleSize {
    /// Use a [`Full`](VisibleSize::Full) visible size.
    pub fn full() -> VisibleSize {
        VisibleSize::Full
    }

    /// Use an [`InnerPercentage`](VisibleSize::InnerPercentage) visible size.
    pub fn inner_percent(value: impl Into<f32>) -> VisibleSize {
        VisibleSize::InnerPercentage(Length::new(value.into()))
    }

    pub fn pretty(&self) -> String {
        match self {
            Self::Full => "full".to_string(),
            Self::InnerPercentage(p) => format!("{}%", p.get()),
        }
    }
}
