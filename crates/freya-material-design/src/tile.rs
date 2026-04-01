use std::time::Duration;

use freya_components::tile::Tile;
use freya_core::prelude::*;
use torin::size::Size;

use crate::ripple::Ripple;

/// Extension trait that adds ripple effect support to [Tile].
///
/// This trait provides the [TileRippleExt::ripple] method that wraps the tile's children
/// in a [Ripple] component, creating a Material Design-style ripple effect on click.
///
/// # Example
///
/// ```rust
/// # use freya::{material_design::*, prelude::*};
/// fn app() -> impl IntoElement {
///     let mut checked = use_state(|| false);
///     Tile::new()
///         .on_select(move |_| checked.toggle())
///         .ripple()
///         .leading(Checkbox::new().selected(checked()))
///         .child("Enable notifications")
/// }
/// ```
pub trait TileRippleExt {
    /// Enable ripple effect on this tile.
    /// Returns a [RippleTile] that allows adding children and configuring the ripple.
    fn ripple(self) -> RippleTile;
}

impl TileRippleExt for Tile {
    fn ripple(self) -> RippleTile {
        RippleTile {
            tile: self,
            ripple: Ripple::new(),
        }
    }
}

/// A Tile with a Ripple effect wrapper.
///
/// Created by calling [TileRippleExt::ripple] on a Tile.
/// Allows adding children to the ripple and configuring its color/duration.
#[derive(Clone, PartialEq)]
pub struct RippleTile {
    tile: Tile,
    ripple: Ripple,
}

impl ChildrenExt for RippleTile {
    fn get_children(&mut self) -> &mut Vec<Element> {
        self.tile.get_children()
    }
}

impl KeyExt for RippleTile {
    fn write_key(&mut self) -> &mut DiffKey {
        self.tile.write_key()
    }
}

impl RippleTile {
    /// Set the color of the ripple effect.
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.ripple = self.ripple.color(color);
        self
    }

    /// Set the duration of the ripple animation.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.ripple = self.ripple.duration(duration);
        self
    }

    /// Set the leading element of the tile.
    pub fn leading(mut self, leading: impl Into<Element>) -> Self {
        self.tile = self.tile.leading(leading);
        self
    }
}

impl Component for RippleTile {
    fn render(&self) -> impl IntoElement {
        let tile = self.tile.clone();

        let mut ripple = self.ripple.clone();
        ripple.get_children().clear();
        ripple.get_children().push(tile.into());
        ripple.width(Size::fill())
    }

    fn render_key(&self) -> DiffKey {
        self.tile.render_key()
    }
}
