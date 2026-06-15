use freya_engine::prelude::*;

/// Controls whether the extra leading height of the first and last lines is kept or trimmed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Default)]
pub enum TextHeightBehavior {
    /// Keep the leading on both the first ascent and the last descent.
    All = 0,
    /// Trim the leading above the first line.
    DisableFirstAscent = 1,
    /// Trim the leading below the last line.
    DisableLastDescent = 2,
    /// Trim the leading on both the first and last lines. This is the default.
    #[default]
    DisableAll = 3,
}

impl TextHeightBehavior {
    pub fn needs_custom_height(&self) -> bool {
        matches!(
            self,
            Self::All | Self::DisableFirstAscent | Self::DisableLastDescent
        )
    }

    pub fn pretty(&self) -> String {
        match self {
            Self::All => "All".to_string(),
            Self::DisableFirstAscent => "DisableFirstAscent".to_string(),
            Self::DisableLastDescent => "DisableLastDescent".to_string(),
            Self::DisableAll => "DisableAll".to_string(),
        }
    }
}

impl From<TextHeightBehavior> for SkTextHeightBehavior {
    fn from(value: TextHeightBehavior) -> Self {
        match value {
            TextHeightBehavior::All => SkTextHeightBehavior::All,
            TextHeightBehavior::DisableAll => SkTextHeightBehavior::DisableAll,
            TextHeightBehavior::DisableFirstAscent => SkTextHeightBehavior::DisableFirstAscent,
            TextHeightBehavior::DisableLastDescent => SkTextHeightBehavior::DisableLastDescent,
        }
    }
}
