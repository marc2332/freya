use crate::{LinkTooltip, Tooltip};
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_applied_theme, LinkThemeWith};

/// **⚠️ If you use `dioxus-router`, you should use [`Link`](crate::Link).**]
///
/// This is an alternative to [`Link`](crate::Link), but it only works for external URLs, not internal routes.
///
/// # Styling
/// Inherits the [`LinkTheme`](freya_hooks::LinkTheme) theme.
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
#[component]
pub fn ExternalLink<'a>(
    cx: Scope<'a>,
    /// Theme override.
    #[props(optional)]
    theme: Option<LinkThemeWith>,
    /// Inner children for the ExternalLink.
    children: Element<'a>,
    #[props(optional)]
    /// Handler for the `onerror` event.
    onerror: Option<EventHandler<'a, ()>>,
    #[props(optional)]
    /// A little text hint to show when hovering over the anchor.
    ///
    /// Setting this to [`None`] is the same as [`LinkTooltip::Default`].
    /// To remove the tooltip, set this to [`LinkTooltip::None`].
    #[props(optional)]
    tooltip: Option<LinkTooltip<'a>>,
    /// The destination URL.
    url: &'a str,
) -> Element {
    let theme = use_applied_theme!(cx, theme, link);
    let is_hovering = use_state(cx, || false);

    let onmouseover = |_: MouseEvent| {
        is_hovering.with_mut(|v| *v = true);
    };

    let onmouseleave = |_: MouseEvent| {
        is_hovering.with_mut(|v| *v = false);
    };

    let onclick = move |_: MouseEvent| {
        let res = open::that(url);
        if let (Err(_), Some(onerror)) = (res, onerror.as_ref()) {
            onerror.call(());
        }
        // TODO(marc2332): Log unhandled errors
    };

    let color = if *is_hovering.get() {
        theme.highlight_color.as_ref()
    } else {
        "inherit"
    };

    let tooltip = match tooltip {
        None | Some(LinkTooltip::Default) => Some(url),
        Some(LinkTooltip::None) => None,
        Some(LinkTooltip::Custom(str)) => Some(str),
    };

    let Some(tooltip) = tooltip else {
        return render! {
            rect { onclick: onclick, children }
        };
    };

    render!(
        rect {
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            onclick: onclick,
            color: "{color}",
            children
        }
        rect {
            height: "0",
            width: "0",
            layer: "-999",
            rect {
                width: "100v",
                if *is_hovering.get() {
                    rsx! {
                        Tooltip {
                            url: tooltip
                        }
                    }
                }
            }
        }
    )
}
