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
    pub fn normal() -> Content {
        Content::Normal
    }

    /// Use a [`Fit`](Content::Fit) content.
    pub fn fit() -> Content {
        Content::Fit
    }

    /// Use a [`Flex`](Content::Flex) content.
    pub fn flex() -> Content {
        Content::Flex
    }

    /// Use a [`Wrap`](Content::Wrap) content with no spacing.
    pub fn wrap() -> Content {
        Content::Wrap { wrap_spacing: None }
    }

    /// Use a [`Wrap`](Content::Wrap) content with the given spacing.
    pub fn wrap_spacing(spacing: f32) -> Content {
        Content::Wrap {
            wrap_spacing: Some(spacing),
        }
    }

    pub fn is_fit(&self) -> bool {
        self == &Self::Fit
    }

    pub fn is_flex(&self) -> bool {
        self == &Self::Flex
    }

    pub fn is_wrap(&self) -> bool {
        matches!(self, Self::Wrap { .. })
    }

    pub fn allows_alignments(&self) -> bool {
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
