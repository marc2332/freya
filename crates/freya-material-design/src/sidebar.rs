use std::time::Duration;

use freya_components::{
    get_theme,
    sidebar::SideBarItem,
};
use freya_core::prelude::*;
use torin::size::Size;

use crate::ripple::Ripple;

/// Extension trait that adds ripple effect support to [SideBarItem].
///
/// This trait provides the [SideBarItemRippleExt::ripple] method that wraps the item's children
/// in a [Ripple] component, creating a Material Design-style ripple effect on click.
///
/// # Example
///
/// ```rust
/// # use freya::{material_design::*, prelude::*};
/// fn app() -> impl IntoElement {
///     SideBarItem::new()
///         .on_press(|_| println!("Pressed"))
///         .ripple()
///         .child("Settings")
/// }
/// ```
pub trait SideBarItemRippleExt {
    /// Enable ripple effect on this sidebar item.
    /// Returns a [RippleSideBarItem] that allows adding children and configuring the ripple.
    fn ripple(self) -> RippleSideBarItem;
}

impl SideBarItemRippleExt for SideBarItem {
    fn ripple(self) -> RippleSideBarItem {
        RippleSideBarItem {
            item: self,
            ripple: Ripple::new(),
        }
    }
}

/// A SideBarItem with a Ripple effect wrapper.
///
/// Created by calling [SideBarItemRippleExt::ripple] on a SideBarItem.
/// Allows adding children to the ripple and configuring its color/duration.
#[derive(Clone, PartialEq)]
pub struct RippleSideBarItem {
    item: SideBarItem,
    ripple: Ripple,
}

impl ChildrenExt for RippleSideBarItem {
    fn get_children(&mut self) -> &mut Vec<Element> {
        self.ripple.get_children()
    }
}

impl KeyExt for RippleSideBarItem {
    fn write_key(&mut self) -> &mut DiffKey {
        self.item.write_key()
    }
}

impl RippleSideBarItem {
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

impl Component for RippleSideBarItem {
    fn render(&self) -> impl IntoElement {
        let mut item = self.item.clone();

        let theme = get_theme!(&item.get_theme(), sidebar_item);

        let ripple = self
            .ripple
            .clone()
            .padding(theme.padding)
            .width(Size::fill());

        let theme_override = item.get_theme().cloned().unwrap_or_default().padding(0.);

        item.get_children().clear();
        item.get_children().push(ripple.into());
        item.theme(theme_override)
    }

    fn render_key(&self) -> DiffKey {
        self.item.render_key()
    }
}
