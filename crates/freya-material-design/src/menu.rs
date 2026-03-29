use std::time::Duration;

use freya_components::menu::MenuItem;
use freya_core::prelude::*;
use torin::size::Size;

use crate::ripple::Ripple;

/// Extension trait that adds ripple effect support to [MenuItem].
pub trait MenuItemRippleExt {
    /// Enable ripple effect on this menu item.
    fn ripple(self) -> RippleMenuItem;
}

impl MenuItemRippleExt for MenuItem {
    fn ripple(self) -> RippleMenuItem {
        RippleMenuItem {
            item: self,
            ripple: Ripple::new(),
        }
    }
}

/// A MenuItem with a Ripple effect wrapper.
///
/// Created by calling [MenuItemRippleExt::ripple] on a MenuItem.
#[derive(Clone, PartialEq)]
pub struct RippleMenuItem {
    item: MenuItem,
    ripple: Ripple,
}

impl ChildrenExt for RippleMenuItem {
    fn get_children(&mut self) -> &mut Vec<Element> {
        self.ripple.get_children()
    }
}

impl KeyExt for RippleMenuItem {
    fn write_key(&mut self) -> &mut DiffKey {
        self.item.write_key()
    }
}

impl RippleMenuItem {
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
}

impl Component for RippleMenuItem {
    fn render(&self) -> impl IntoElement {
        let mut item = self.item.clone();

        let padding = item.get_padding();

        let ripple = self
            .ripple
            .clone()
            .padding(padding)
            .width(Size::fill_minimum());

        item.get_children().clear();
        item.get_children().push(ripple.into());
        item.padding(0.)
    }

    fn render_key(&self) -> DiffKey {
        ComponentOwned::render_key(&self.item)
    }
}
