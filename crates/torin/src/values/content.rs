use crate::scaled::Scaled;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Clone, Debug, Default)]
pub enum Content {
    /// Default layout, children are stacked along the direction axis.
    #[default]
    Normal,
    /// Resize children to evenly fit the available space along the direction axis.
    Fit,
    /// Let children use [`Size::Flex`](crate::size::Size::Flex) to grow proportionally to fill the available space.
    Flex,
    /// Wrap children to the next line or column when they exceed the available space,
    /// with an optional gap between wrapped lines.
    Wrap { wrap_spacing: Option<f32> },
}

impl Content {
    /// Use a [`Normal`](Content::Normal) content.
    pub const fn normal() -> Content {
        Content::Normal
    }

    /// Use a [`Fit`](Content::Fit) content.
    pub const fn fit() -> Content {
        Content::Fit
    }

    /// Use a [`Flex`](Content::Flex) content.
    pub const fn flex() -> Content {
        Content::Flex
    }

    /// Use a [`Wrap`](Content::Wrap) content with no spacing.
    pub const fn wrap() -> Content {
        Content::Wrap { wrap_spacing: None }
    }

    /// Use a [`Wrap`](Content::Wrap) content with the given spacing.
    pub const fn wrap_spacing(spacing: f32) -> Content {
        Content::Wrap {
            wrap_spacing: Some(spacing),
        }
    }

    pub const fn is_fit(&self) -> bool {
        matches!(self, Self::Fit)
    }

    pub const fn is_flex(&self) -> bool {
        matches!(self, Self::Flex)
    }

    pub const fn is_wrap(&self) -> bool {
        matches!(self, Self::Wrap { .. })
    }

    pub const fn allows_alignments(&self) -> bool {
        matches!(self, Self::Normal | Self::Flex | Self::Fit)
    }
}

impl Content {
    pub fn pretty(&self) -> String {
        match self {
            Self::Normal => "normal".to_owned(),
            Self::Fit => "fit".to_owned(),
            Self::Flex => "flex".to_owned(),
            Self::Wrap { .. } => "wrap".to_owned(),
        }
    }
}

impl Scaled for Content {
    fn scale(&mut self, scale_factor: f32) {
        if let Self::Wrap {
            wrap_spacing: Some(wrap_spacing),
        } = self
        {
            *wrap_spacing *= scale_factor;
        }
    }
}
