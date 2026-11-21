use alloc::{
    borrow::ToOwned,
    string::String,
};

use crate::scaled::Scaled;

/// Controls how a parent's children are laid out.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Clone, Debug, Default)]
pub enum Content {
    /// Stack children along the main axis. **This is the default.**
    ///
    /// Horizontal layouts place children left to right, vertical layouts top to bottom.
    ///
    /// For example, a horizontal parent with 400 px of usable width and two 100 px children:
    ///
    /// ```text
    /// | Child: 100 px | Child: 100 px |           200 px unused           |
    /// ```
    #[default]
    Normal,
    /// Matches child widths to the widest sibling in a vertical layout, or heights
    /// to the tallest sibling in a horizontal layout, when the parent is auto-sized.
    ///
    /// Set [`Size::fill_minimum`](crate::size::Size::fill_minimum) on each child width
    /// or height that should match.
    ///
    /// For example, a vertical auto-width parent with a 100 px wide child:
    ///
    /// ```text
    /// | Fixed width: 100 px    |
    /// | fill_minimum(): 100 px |
    /// | fill_minimum(): 100 px |
    /// ```
    Fit,
    /// Share the remaining space between children using [`Size::Flex`](crate::size::Size::Flex).
    ///
    /// Set this on the parent, then use [`Size::flex`](crate::size::Size::flex) on each
    /// child's width for a horizontal layout, or height for a vertical layout.
    /// Flex children share the space left after non-flex children, padding, margins
    /// and spacing, according to their grow factors (weights).
    ///
    /// For example, a horizontal parent with 400 px of usable width, no gaps and
    /// three children sized `Size::px(100.0)`, `Size::flex(1.0)` and `Size::flex(2.0)`:
    ///
    /// ```text
    /// | Fixed: 100 px | Flex(1): 100 px |          Flex(2): 200 px          |
    ///                 <----------- 300 px remaining -------------------->
    /// ```
    ///
    /// The total weight is 3, so the flex children receive 1/3 and 2/3 of the
    /// remaining 300 px. A weight of 2 gets twice as much space as a weight of 1.
    /// Weights are relative, not pixel sizes or percentages. Weights of 2 and 4
    /// produce the same split as 1 and 2.
    ///
    /// If the total weight is less than 1, each child receives its weight times
    /// the remaining space instead, leaving some space unused. For example, a
    /// single child with `Size::flex(0.5)` receives half of the remaining space.
    Flex,
    /// Start a new row or column when a child no longer fits along the main axis.
    ///
    /// Horizontal layouts wrap into rows below, vertical layouts into columns to the
    /// right.
    ///
    /// Use [`Content::wrap_spacing`] for gaps between rows or columns, or [`Content::wrap`]
    /// for no gap between them.
    ///
    /// For example, a horizontal parent with 250 px of usable width, no gaps and three
    /// 100 px wide children:
    ///
    /// ```text
    /// | Child: 100 px | Child: 100 px | 50 px unused |
    /// | Child: 100 px |       150 px unused         |
    /// ```
    Wrap {
        /// Gap between wrapped rows or columns, with `None` equivalent to zero.
        wrap_spacing: Option<f32>,
    },
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
