use dioxus::prelude::*;
use dioxus_router::{hooks::use_navigator, routable::Routable};
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    theme_with, use_applied_theme, FontTheme, ScrollViewThemeWith, SidebarItemThemeWith,
    SidebarTheme, SidebarThemeWith,
};

use crate::{ButtonStatus, ScrollView};

#[allow(non_snake_case)]
#[component]
pub fn Sidebar<'a>(
    cx: Scope<'a>,
    /// Theme override.
    theme: Option<SidebarThemeWith>,
    sidebar: Element<'a>,
    children: Element<'a>,
) -> Element<'a> {
    //let SidebarProps { theme, sidebar, children } = &cx.props;
    let theme = use_applied_theme!(cx, &theme, sidebar);
    let SidebarTheme {
        background,
        font_theme: FontTheme { color },
    } = theme;

    render!(
        rect {
            width: "100%",
            height: "100%",
            direction: "horizontal",
            rect {
                overflow: "clip",
                width: "170",
                height: "100%",
                background: "{background}",
                color: "{color}",
                shadow: "2 0 5 0 rgb(0, 0, 0, 30)",
                ScrollView {
                    theme: theme_with!(ScrollViewTheme {
                        padding: "16".into(),
                    }),
                    sidebar
                }
            }
            rect {
                overflow: "clip",
                width: "fill",
                height: "100%",
                children,
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
pub fn SidebarItem<'a, T: Routable>(
    cx: Scope<'a>,
    /// Theme override.
    theme: Option<SidebarItemThemeWith>,
    children: Element<'a>,
    onclick: Option<EventHandler<'a, ()>>,
    to: Option<T>,
) -> Element<'a> {
    let theme = use_applied_theme!(cx, theme, sidebar_item);
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
        ButtonStatus::Hovering => theme.hover_background,
        ButtonStatus::Idle => theme.background,
    };
    let color = theme.font_theme.color;
    let border_fill = theme.border_fill;

    render!(
        rect {
            onclick: onclick,
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,
            overflow: "clip",
            margin: "5 0",
            width: "100%",
            height: "auto",
            color: "{color}",
            border: "1 solid {border_fill}",
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
