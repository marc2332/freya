use dioxus::prelude::*;
use freya_elements as dioxus_elements;
use freya_elements::MouseEvent;
use freya_hooks::use_get_theme;

use crate::Tooltip;

#[derive(Props)]
pub struct ExternalLinkProps<'a> {
    children: Element<'a>,
    #[props(optional)]
    onerror: Option<EventHandler<'a, ()>>,
    #[props(optional)]
    show_tooltip: Option<bool>,
    url: &'a str,
}

#[allow(non_snake_case)]
pub fn ExternalLink<'a>(cx: Scope<'a, ExternalLinkProps<'a>>) -> Element {
    let theme = use_get_theme(cx);
    let theme = &theme.external_link;
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
        "inherit"
    };

    render!(
        rect {
            direction: "both",
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            onclick: onclick,
            color: "{color}",
            &cx.props.children
        }
        rect {
            height: "0",
            width: "0",
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
