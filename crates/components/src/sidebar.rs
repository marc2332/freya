#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::{ButtonStatus, ScrollView};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    theme_with, use_applied_theme, ScrollViewThemeWith, SidebarItemTheme, SidebarItemThemeWith,
    SidebarTheme, SidebarThemeWith,
};

#[component]
pub fn Sidebar(
    /// Theme override.
    theme: Option<SidebarThemeWith>,
    /// This is what is rendered next to the sidebar.
    children: Element,
    /// This is what is rendered in the sidebar.
    sidebar: Element,
) -> Element {
    let SidebarTheme {
        font_theme,
        background,
    } = use_applied_theme!(&theme, sidebar);

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            direction: "horizontal",
            background: "{background}",
            rect {
                overflow: "clip",
                width: "170",
                height: "100%",
                background: "{background}",
                color: "{font_theme.color}",
                shadow: "2 0 5 0 rgb(0, 0, 0, 30)",
                ScrollView {
                    theme: theme_with!(ScrollViewTheme {
                        padding: "16".into(),
                    }),
                    {sidebar}
                }
            }
            rect {
                overflow: "clip",
                width: "fill",
                height: "100%",
                color: "{font_theme.color}",
                {children}
            }
        }
    )
}

#[component]
pub fn SidebarItem(
    /// Theme override.
    theme: Option<SidebarItemThemeWith>,
    children: Element,
    onclick: Option<EventHandler<()>>,
) -> Element {
    let SidebarItemTheme {
        hover_background,
        background,
        font_theme,
        border_fill,
    } = use_applied_theme!(&theme, sidebar_item);
    let mut status = use_signal(ButtonStatus::default);

    let onclick = move |_| {
        if let Some(onclick) = &onclick {
            onclick.call(());
        }
    };

    let onmouseenter = move |_| {
        status.set(ButtonStatus::Hovering);
    };

    let onmouseleave = move |_| {
        status.set(ButtonStatus::default());
    };

    let background = match *status.read() {
        ButtonStatus::Hovering => hover_background,
        ButtonStatus::Idle => background,
    };

    rsx!(
        rect {
            overflow: "clip",
            margin: "5 0",
            onclick,
            onmouseenter,
            onmouseleave,
            width: "100%",
            height: "auto",
            color: "{font_theme.color}",
            border: "1 solid {border_fill}",
            shadow: "0 4 5 0 rgb(0, 0, 0, 30)",
            corner_radius: "8",
            padding: "8",
            background: "{background}",
            label {
                {children}
            }
        }
    )
}
