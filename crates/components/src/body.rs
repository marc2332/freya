use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_get_theme, BodyTheme};

/// [`Body`] component properties.
#[derive(Props)]
pub struct BodyProps<'a> {
    /// Inner children for the Body.
    children: Element<'a>,
    /// Padding for the Body.
    #[props(default = "none".to_string(), into)]
    padding: String,
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
    let theme = use_get_theme(cx);
    let BodyTheme { background, color } = theme.body;
    let BodyProps { children, padding } = cx.props;

    render!(rect {
        width: "fill",
        height: "fill",
        color: "{color}",
        background: "{background}",
        padding: "{padding}",
        children
    })
}
