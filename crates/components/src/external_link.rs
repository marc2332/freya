use crate::{LinkTooltip, Tooltip};
use dioxus::prelude::*;
use winit::event::MouseButton;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_applied_theme, LinkThemeWith};

/// <div class="warning">ℹ️ For linking to app routes, use [`Link`](crate::Link).</div>
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
/// fn app() -> Element {
///     rsx!(
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
#[component]
pub fn ExternalLink(
    /// Theme override.
    #[props(optional)]
    theme: Option<LinkThemeWith>,
    /// Inner children for the ExternalLink.
    children: Element,
    #[props(optional)]
    /// Handler for the `onerror` event.
    onerror: Option<EventHandler<()>>,
    #[props(optional)]
    /// A little text hint to show when hovering over the anchor.
    ///
    /// Setting this to [`None`] is the same as [`LinkTooltip::Default`].
    /// To remove the tooltip, set this to [`LinkTooltip::None`].
    #[props(optional)]
    tooltip: Option<LinkTooltip>,
    /// The destination URL.
    url: String,
) -> Element {
    let theme = use_applied_theme!(&theme, link);
    let mut is_hovering = use_signal(|| false);

    let onmouseover = move |_: MouseEvent| {
        is_hovering.set(true);
    };

    let onmouseleave = move |_: MouseEvent| {
        is_hovering.set(false);
    };

    let onclick = {
        to_owned![url];
        move |event: MouseEvent| {
            if !matches!(event.trigger_button, Some(MouseButton::Left)) {
                return;
            }

            let res = open::that(&url);
            if let (Err(_), Some(onerror)) = (res, onerror.as_ref()) {
                onerror.call(());
            }
        }
    };

    let color = if *is_hovering.read() {
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
        return rsx! {
            rect { onclick: onclick, color: "{color}", {children} }
        };
    };

    rsx!(
        rect {
            onmouseover,
            onmouseleave,
            onclick,
            color: "{color}",
            {children}
        }
        rect {
            height: "0",
            width: "0",
            layer: "-999",
            rect {
                width: "100v",
                if *is_hovering.read() {
                    Tooltip {
                        url: tooltip
                    }
                }
            }
        }
    )
}
