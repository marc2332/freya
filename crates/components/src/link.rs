use crate::Tooltip;
use dioxus::hooks::use_state;
use dioxus::prelude::*;
use dioxus_router::prelude::{use_navigator, IntoRoutable};
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::{use_applied_theme, LinkThemeWith};
use std::borrow::Cow;
use winit::event::MouseButton;

pub enum LinkTooltip<'a> {
    /// No tooltip at all.
    None,
    /// Default tooltip.
    ///
    /// - For a route, this is the same as [`None`](AnchorTooltip::None).
    /// - For a URL, this is the value of that URL.
    Default,
    /// Custom tooltip to always show.
    Custom(&'a str),
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
pub fn Link<'a>(
    cx: Scope<'a>,
    /// Theme override.
    #[props(optional)]
    theme: Option<LinkThemeWith>,
    /// The route or external URL string to navigate to.
    #[props(into)]
    to: IntoRoutable,
    children: Element<'a>,
    /// This event will be fired if opening an external link fails.
    #[props(optional)]
    onerror: Option<EventHandler<'a, ()>>,
    /// A little text hint to show when hovering over the anchor.
    ///
    /// Setting this to [`None`] is the same as [`LinkTooltip::Default`].
    /// To remove the tooltip, set this to [`LinkTooltip::None`].
    #[props(optional)]
    tooltip: Option<LinkTooltip<'a>>,
) -> Element<'a> {
    let theme = use_applied_theme!(cx, theme, link);
    let nav = use_navigator(cx);
    let is_hovering = use_state(cx, || false);

    let url = if let IntoRoutable::FromStr(url) = to {
        Some(url)
    } else {
        None
    };

    let onmouseover = |_: MouseEvent| {
        is_hovering.with_mut(|v| *v = true);
    };

    let onmouseleave = |_: MouseEvent| {
        is_hovering.with_mut(|v| *v = false);
    };

    let onclick = move |event: MouseEvent| {
        let Some(MouseButton::Left) = event.trigger_button else {
            return;
        };

        url.map_or_else(
            || {
                nav.push(to.clone());
            },
            |url| {
                let res = open::that(url);

                if let (Err(_), Some(onerror)) = (res, onerror.as_ref()) {
                    onerror.call(());
                }

                // TODO(marc2332): Log unhandled errors
            },
        );
    };

    let color = if *is_hovering.get() {
        theme.highlight_color
    } else {
        Cow::Borrowed("inherit")
    };

    let tooltip = match tooltip {
        None | Some(LinkTooltip::Default) => url.map(String::as_str),
        Some(LinkTooltip::None) => None,
        Some(LinkTooltip::Custom(str)) => Some(*str),
    };

    let main_rect = rsx! {
        rect { onmouseover: onmouseover, onmouseleave: onmouseleave, onclick: onclick, color: "{color}", children }
    };

    let Some(tooltip) = tooltip else {
        return render!(main_rect);
    };

    render! {
        main_rect

        rect {
            height: "0",
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
    }
}
