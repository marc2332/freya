use std::time::Duration;

use freya_components::{
    button::{
        Button,
        ButtonLayoutVariant,
    },
    get_theme,
    theming::component_themes::ButtonLayoutThemePartialExt,
};
use freya_core::prelude::*;

use crate::ripple::Ripple;

/// Extension trait that adds ripple effect support to [Button].
///
/// This trait provides the [ButtonRippleExt::ripple] method that wraps the button's children
/// in a [Ripple] component, creating a Material Design-style ripple effect on click.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::{material_design::*, *};
/// fn app() -> impl IntoElement {
///     Button::new()
///         .on_press(|_| println!("Pressed!"))
///         .ripple()
///         .color((200, 200, 255))
///         .child("Click me!")
///         .child("More text")
/// }
/// ```
pub trait ButtonRippleExt {
    /// Enable ripple effect on this button.
    /// Returns a [RippleButton] that allows adding children and configuring the ripple.
    fn ripple(self) -> RippleButton;
}

impl ButtonRippleExt for Button {
    fn ripple(self) -> RippleButton {
        RippleButton {
            button: self,
            ripple: Ripple::new(),
        }
    }
}

/// A Button with a Ripple effect wrapper.
///
/// Created by calling [ButtonRippleExt::ripple] on a Button.
/// Allows adding children to the ripple and configuring its color/duration.
#[derive(Clone, PartialEq)]
pub struct RippleButton {
    button: Button,
    ripple: Ripple,
}

impl ChildrenExt for RippleButton {
    fn get_children(&mut self) -> &mut Vec<Element> {
        self.ripple.get_children()
    }
}

impl RippleButton {
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

impl Render for RippleButton {
    fn render(&self) -> impl IntoElement {
        let mut button = self.button.clone();

        let theme_layout = match button.get_layout_variant() {
            ButtonLayoutVariant::Normal => get_theme!(&button.get_theme_layout(), button_layout),
            ButtonLayoutVariant::Compact => {
                get_theme!(&button.get_theme_layout(), compact_button_layout)
            }
            ButtonLayoutVariant::Expanded => {
                get_theme!(&button.get_theme_layout(), expanded_button_layout)
            }
        };

        let ripple = self.ripple.clone().padding(theme_layout.padding);

        button.get_children().clear();
        button.get_children().push(ripple.into());
        button.padding(0.)
    }

    fn render_key(&self) -> DiffKey {
        self.button.render_key()
    }
}
