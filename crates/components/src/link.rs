use crate::Tooltip;
use dioxus::prelude::*;
use dioxus_router::prelude::{use_navigator, IntoRoutable};
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_applied_theme, LinkThemeWith};
use std::borrow::Cow;
use winit::event::MouseButton;

#[derive(Clone, PartialEq)]
pub enum LinkTooltip {
    /// No tooltip at all.
    None,
    /// Default tooltip.
    ///
    /// - For a route, this is the same as [`None`](AnchorTooltip::None).
    /// - For a URL, this is the value of that URL.
    Default,
    /// Custom tooltip to always show.
    Custom(String),
}

/// **⚠️ Just like Dioxus's `Link`,
/// this must be a descendant of a
/// [`Router`](https://docs.rs/dioxus-router/latest/dioxus_router/components/fn.Router.html) component**
/// **⚠️ Do not register the `onclick` event in the children,
/// Dioxus event propagation doesn't work properly.**
///
/// Similar to [`Link`](dioxus_router::components::Link), but you can use it in Freya.
/// Both internal routes and external links are supported.
///
/// # Styling
///
/// Inherits the [`LinkTheme`](freya_hooks::LinkTheme) theme.
///
/// # Example
///
/// Bad (will not render the text):
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use freya_components::Link;
/// # fn link_example_bad(cx: Scope) -> Element {
/// render! {
///     Link {
///         to: "https://crates.io/crates/freya",
///         "Freya crates.io"
///     }
/// }
/// # }
/// ```
///
/// Good:
///
/// ```rust
/// # use dioxus::prelude::*;
/// # use freya_elements::elements as dioxus_elements;
/// # use freya_components::Link;
/// # fn link_example_good(cx: Scope) -> Element {
/// render! {
///     Link {
///         to: "https://crates.io/crates/freya",
///         label { "Freya crates.io" }
///     }
/// }
/// # }
/// ```
#[component]
pub fn Link(
    /// Theme override.
    #[props(optional)]
    theme: Option<LinkThemeWith>,
    /// The route or external URL string to navigate to.
    #[props(into)]
    to: IntoRoutable,
    children: Element,
    /// This event will be fired if opening an external link fails.
    #[props(optional)]
    onerror: Option<EventHandler<()>>,
    /// A little text hint to show when hovering over the anchor.
    ///
    /// Setting this to [`None`] is the same as [`LinkTooltip::Default`].
    /// To remove the tooltip, set this to [`LinkTooltip::None`].
    #[props(optional)]
    tooltip: Option<LinkTooltip>,
) -> Element {
    let theme = use_applied_theme!(&theme, link);
    let nav = use_navigator();
    let mut is_hovering = use_signal(|| false);

    let url = if let IntoRoutable::FromStr(ref url) = to {
        Some(url.clone())
    } else {
        None
    };

    let onmouseover = move |_: MouseEvent| {
        is_hovering.set(true);
    };

    let onmouseleave = move |_: MouseEvent| {
        is_hovering.set(false);
    };

    let onclick = {
        to_owned![url, to];
        move |event: MouseEvent| {
            let Some(MouseButton::Left) = event.trigger_button else {
                return;
            };

            url.as_ref().map_or_else(
                || {
                    nav.push(to.clone());
                },
                |url| {
                    let res = open::that(&url);

                    if let (Err(_), Some(onerror)) = (res, onerror.as_ref()) {
                        onerror.call(());
                    }

                    // TODO(marc2332): Log unhandled errors
                },
            );
        }
    };

    let color = if *is_hovering.read() {
        theme.highlight_color
    } else {
        Cow::Borrowed("inherit")
    };

    let tooltip = match tooltip {
        None | Some(LinkTooltip::Default) => url.clone(),
        Some(LinkTooltip::None) => None,
        Some(LinkTooltip::Custom(str)) => Some(str),
    };

    let main_rect = rsx! {
        rect {
            onmouseover,
            onmouseleave,
            onclick,
            color: "{color}",
            {children}
        }
    };

    let Some(tooltip) = tooltip else {
        return rsx!({ main_rect });
    };

    rsx! {
        {main_rect}
        rect {
            height: "0",
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
    }
}
