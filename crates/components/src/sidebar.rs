use crate::{ButtonStatus, ScrollView};
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    theme_with, use_activable_route, use_applied_theme, use_platform, ScrollViewThemeWith,
    SidebarItemTheme, SidebarItemThemeWith, SidebarTheme, SidebarThemeWith,
};
use winit::window::CursorIcon;

#[allow(non_snake_case)]
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
            rect {
                overflow: "clip",
                width: "170",
                height: "100%",
                background: "{background}",
                color: "{font_theme.color}",
                shadow: "2 0 5 0 rgb(0, 0, 0, 30)",
                ScrollView {
                    theme: theme_with!(ScrollViewTheme {
                        padding: "8".into(),
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

#[allow(non_snake_case)]
#[component]
pub fn SidebarItem(
    /// Theme override.
    theme: Option<SidebarItemThemeWith>,
    /// Inner content for the SidebarItem.
    children: Element,
    /// Optionally handle the `onclick` event in the SidebarItem.
    onclick: Option<EventHandler<()>>,
) -> Element {
    let SidebarItemTheme {
        hover_background,
        background,
        font_theme,
    } = use_applied_theme!(&theme, sidebar_item);
    let mut status = use_signal(ButtonStatus::default);
    let platform = use_platform();
    let is_active = use_activable_route();

    use_drop(move || {
        if *status.read() == ButtonStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onclick = move |_| {
        if let Some(onclick) = &onclick {
            onclick.call(());
        }
    };

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(ButtonStatus::Hovering);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(ButtonStatus::default());
    };

    let background = match *status.read() {
        _ if is_active => hover_background,
        ButtonStatus::Hovering => hover_background,
        ButtonStatus::Idle => background,
    };

    rsx!(
        rect {
            overflow: "clip",
            margin: "4 0",
            onclick,
            onmouseenter,
            onmouseleave,
            width: "100%",
            height: "auto",
            color: "{font_theme.color}",
            corner_radius: "99",
            padding: "8 10",
            background: "{background}",
            {children}
        }
    )
}
