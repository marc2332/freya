use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;

use freya_hooks::{use_applied_theme, BodyTheme, BodyThemeWith};

/// [`Body`] component properties.
#[derive(Props)]
pub struct BodyProps<'a> {
    /// Theme override.
    pub theme: Option<BodyThemeWith>,
    /// Inner children for the Body.
    pub children: Element<'a>,
}

/// `Body` component.
///
/// Usually just used one time and as a root component for all the app.
///
/// # Props
/// See [`BodyProps`].
///
/// # Styling
/// Inherits the [`BodyTheme`](freya_hooks::BodyTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     render!(
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
pub fn Body<'a>(cx: Scope<'a, BodyProps<'a>>) -> Element {
    let theme = use_applied_theme!(cx, &cx.props.theme, body);
    let BodyTheme {
        background,
        color,
        padding,
    } = theme;

    render!(
        rect {
            width: "fill",
            height: "fill",
            color: "{color}",
            background: "{background}",
            padding: "{padding}",
            &cx.props.children
        }
    )
}
