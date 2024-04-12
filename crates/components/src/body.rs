use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

use freya_hooks::{use_applied_theme, BodyTheme, BodyThemeWith};

/// Properties for the [`Body`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BodyProps {
    /// Theme override.
    pub theme: Option<BodyThemeWith>,
    /// Inner children for the Body.
    pub children: Element,
}

/// Usually used to wrap the application root component.
///
/// # Styling
/// Inherits the [`BodyTheme`](freya_hooks::BodyTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         Body {
///             label {
///                 "Click this"
///             }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn Body(props: BodyProps) -> Element {
    let theme = use_applied_theme!(&props.theme, body);
    let BodyTheme {
        background,
        color,
        padding,
    } = theme;

    rsx!(
        rect {
            width: "fill",
            height: "fill",
            color: "{color}",
            background: "{background}",
            padding: "{padding}",
            {&props.children}
        }
    )
}
