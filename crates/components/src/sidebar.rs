use dioxus::prelude::*;
use dioxus_router::{hooks::use_navigator, routable::Routable};
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_get_theme, use_theme, ProgressBarTheme};

use crate::{ButtonStatus, ScrollView};

#[allow(non_snake_case)]
#[inline_props]
pub fn Sidebar<'a>(cx: Scope<'a>, children: Element<'a>, sidebar: Element<'a>) -> Element<'a> {
    let theme = use_theme(cx);
    let background = theme.read().body.background;
    let color = theme.read().body.color;

    render!(
        rect {
            width: "100%",
            height: "100%",
            direction: "horizontal",
            background: "{background}",
            rect {
                overflow: "clip",
                width: "170",
                height: "100%",
                background: "rgb(20, 20, 20)",
                color: "{color}",
                ScrollView {
                    padding: "16",
                    sidebar
                }
            }
            rect {
                overflow: "clip",
                width: "fill",
                height: "100%",
                color: "{color}",
                children,
            }
        }
    )
}

#[allow(non_snake_case)]
#[inline_props]
pub fn SidebarItem<'a, T: Routable>(
    cx: Scope<'a>,
    children: Element<'a>,
    onclick: Option<EventHandler<'a, ()>>,
    to: Option<T>,
) -> Element<'a> {
    let theme = use_get_theme(cx);
    let status = use_state(cx, ButtonStatus::default);
    let navigator = use_navigator(cx);

    let onclick = move |_| {
        if let Some(to) = to {
            navigator.replace(to.clone());
        }
        if let Some(onclick) = onclick {
            onclick.call(());
        }
    };

    let onmouseenter = move |_| {
        status.set(ButtonStatus::Hovering);
    };

    let onmouseleave = move |_| {
        status.set(ButtonStatus::default());
    };

    let background = match *status.get() {
        ButtonStatus::Hovering => theme.button.hover_background,
        ButtonStatus::Idle => theme.button.background,
    };
    let color = theme.button.font_theme.color;

    render!(
        rect {
            overflow: "clip",
            margin: "5 0",
            onclick: onclick,
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,
            width: "100%",
            height: "auto",
            color: "{color}",
            shadow: "0 4 5 0 rgb(0, 0, 0, 30)",
            corner_radius: "8",
            padding: "8",
            background: "{background}",
            label {
                children
            }
        }
    )
}
