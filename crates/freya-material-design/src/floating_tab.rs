use std::time::Duration;

use freya_components::{
    floating_tab::FloatingTab,
    get_theme,
};
use freya_core::prelude::*;

use crate::ripple::Ripple;

/// Extension trait that adds ripple effect support to [FloatingTab].
///
/// This trait provides the [FloatingTabRippleExt::ripple] method that wraps the tab's children
/// in a [Ripple] component, creating a Material Design-style ripple effect on click.
///
/// # Example
///
/// ```rust
/// # use freya::{material_design::*, prelude::*};
/// fn app() -> impl IntoElement {
///     FloatingTab::new().ripple().child("Home")
/// }
/// ```
pub trait FloatingTabRippleExt {
    /// Enable ripple effect on this floating tab.
    /// Returns a [RippleFloatingTab] that allows adding children and configuring the ripple.
    fn ripple(self) -> RippleFloatingTab;
}

impl FloatingTabRippleExt for FloatingTab {
    fn ripple(self) -> RippleFloatingTab {
        RippleFloatingTab {
            tab: self,
            ripple: Ripple::new(),
        }
    }
}

/// A FloatingTab with a Ripple effect wrapper.
///
/// Created by calling [FloatingTabRippleExt::ripple] on a FloatingTab.
/// Allows adding children to the ripple and configuring its color/duration.
#[derive(Clone, PartialEq)]
pub struct RippleFloatingTab {
    tab: FloatingTab,
    ripple: Ripple,
}

impl ChildrenExt for RippleFloatingTab {
    fn get_children(&mut self) -> &mut Vec<Element> {
        self.ripple.get_children()
    }
}

impl KeyExt for RippleFloatingTab {
    fn write_key(&mut self) -> &mut DiffKey {
        self.tab.write_key()
    }
}

impl RippleFloatingTab {
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

impl Component for RippleFloatingTab {
    fn render(&self) -> impl IntoElement {
        let mut tab = self.tab.clone();

        let theme = get_theme!(&tab.get_theme(), floating_tab);

        let ripple = self.ripple.clone().padding(theme.padding);

        let theme_override = tab.get_theme().cloned().unwrap_or_default().padding(0.);

        tab.get_children().clear();
        tab.get_children().push(ripple.into());
        tab.theme(theme_override)
    }

    fn render_key(&self) -> DiffKey {
        self.tab.render_key()
    }
}
