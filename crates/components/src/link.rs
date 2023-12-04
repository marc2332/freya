use dioxus::hooks::use_state;
use dioxus::prelude::{Element, EventHandler, Props, render, Scope, rsx, fc_to_builder};
use dioxus_router::prelude::{IntoRoutable, use_navigator};
use winit::event::MouseButton;
use freya_elements::events::MouseEvent;
use freya_elements::elements as dioxus_elements;
use crate::Tooltip;

pub enum LinkTooltip<'a> {
    /// No tooltip at all.
    None,
    /// Default tooltip.
    ///
    /// - For a route, this is the same as [`None`](AnchorTooltip::None).
    /// - For a URL, this is the value of that URL.
    Default,
    /// Custom tooltip to always show.
    Custom(&'a str)
}

#[derive(Props)]
pub struct LinkProps<'a> {
    /// The route or external URL string to navigate to.
    #[props(into)]
    pub to: IntoRoutable,
    pub children: Element<'a>,
    /// This event will be fired if opening an external link fails.
    pub onerror: Option<EventHandler<'a, ()>>,
    /// A little text hint to show when hovering over the anchor.
    ///
    /// Setting this to [`None`] is the same as [`LinkTooltip::Default`].
    /// To remove the tooltip, set this to [`LinkTooltip::None`].
    pub tooltip: Option<LinkTooltip<'a>>,
}

/// **Warning: Do not pass in a plain string as the children.
/// The children get put inside a [`rect`](freya_elements::elements::rect),
/// so wrap your text in a [`label`](freya_elements::elements::label).**
///
/// **Warning: Do not register the `onclick` event in the children.**
///
/// Similar to [`Link`](dioxus_router::components::Link), but you can use it in Freya.
/// Both internal routes and external links are supported.
///
/// # Examples
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
#[allow(non_snake_case)]
pub fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element<'a> {
    let nav = use_navigator(cx);
    let LinkProps { to, children, onerror, tooltip } = cx.props;
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

        url.map_or_else(|| { nav.push(to.clone()); }, |url| {
            let res = open::that(url);

            if let (Err(_), Some(onerror)) = (res, onerror.as_ref()) {
                onerror.call(());
            }
        });
    };

    let tooltip = match tooltip {
        None | Some(LinkTooltip::Default) => url.map(String::as_str),
        Some(LinkTooltip::None) => None,
        Some(LinkTooltip::Custom(str)) => Some(*str),
    };

    let Some(tooltip) = tooltip else {
        return render! {
            rect { onclick: onclick, children }
        }
    };

    render! {
        rect { onmouseover: onmouseover, onmouseleave: onmouseleave, onclick: onclick, children }

        rect {
            height: "0",
            layer: "-999",
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