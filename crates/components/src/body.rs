use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_hooks::{
    use_applied_theme,
    BodyTheme,
    BodyThemeWith,
};

/// Properties for the [`Body`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BodyProps {
    /// Theme override.
    pub theme: Option<BodyThemeWith>,
    /// Inner children for the Body.
    pub children: Element,
    /// Spacing for the inner children of the Body.
    pub spacing: Option<String>,
    /// Padding for the inner children of the Body.
    pub padding: Option<String>,
    /// Width of the Body.
    #[props(default = "fill".to_string())]
    pub width: String,
    /// Height of the Body.
    #[props(default = "fill".to_string())]
    pub height: String,
    /// Direction of the Body.
    #[props(default = "vertical".to_string())]
    pub direction: String,
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
#[allow(non_snake_case)]
pub fn Body(
    BodyProps {
        children,
        theme,
        spacing,
        padding,
        width,
        height,
        direction,
    }: BodyProps,
) -> Element {
    let theme = use_applied_theme!(&theme, body);
    let BodyTheme { background, color } = theme;

    rsx!(
        rect {
            width,
            height,
            color: "{color}",
            background: "{background}",
            spacing,
            padding,
            direction,
            {&children}
        }
    )
}
