use crate::theme::get_theme;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::ExternalLinkThemeWith;
use std::borrow::Cow;

use crate::Tooltip;

/// [`ExternalLink`] component properties.
#[derive(Props)]
pub struct ExternalLinkProps<'a> {
    /// Theme override.
    #[props(optional)]
    pub theme: Option<ExternalLinkThemeWith>,
    /// Inner children for the ExternalLink.
    pub children: Element<'a>,
    #[props(optional)]
    /// Handler for the `onerror` event.
    pub onerror: Option<EventHandler<'a, ()>>,
    #[props(optional)]
    /// Whether  to show a tooltip with the URL or not.
    pub show_tooltip: Option<bool>,
    /// The ExternalLink destination URL.
    pub url: &'a str,
}

/// `Link` for external locations, e.g websites.
///
/// # Props
/// See [`ExternalLinkProps`].
///
/// # Styling
/// Inherits the [`ExternalLinkTheme`](freya_hooks::ExternalLinkTheme) theme.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     render!(
///         ExternalLink {
///             url: "https://github.com",
///             label {
///                 "GitHub"
///             }
///         }
///     )
/// }
/// ```
///
#[allow(non_snake_case)]
pub fn ExternalLink<'a>(cx: Scope<'a, ExternalLinkProps<'a>>) -> Element {
    let theme = get_theme!(cx, &cx.props.theme, external_link);
    let is_hovering = use_state(cx, || false);
    let show_tooltip = cx.props.show_tooltip.unwrap_or(true);

    let onmouseover = |_: MouseEvent| {
        is_hovering.with_mut(|v| *v = true);
    };

    let onmouseleave = |_: MouseEvent| {
        is_hovering.with_mut(|v| *v = false);
    };

    let onclick = |_: MouseEvent| {
        let res = open::that(cx.props.url);
        if let (Err(_), Some(onerror)) = (res, cx.props.onerror.as_ref()) {
            onerror.call(());
        }
        // TODO(marc2332): Log unhandled errors
    };

    let color = if *is_hovering.get() {
        theme.highlight_color
    } else {
        Cow::Borrowed("inherit")
    };

    render!(
        rect { onmouseover: onmouseover, onmouseleave: onmouseleave, onclick: onclick, color: "{color}", &cx.props.children }
        rect { height: "0", layer: "-999",
            (*is_hovering.get() && show_tooltip).then_some({
                rsx!(
                    Tooltip {
                        url: cx.props.url
                    }
                )
            })
        }
    )
}
