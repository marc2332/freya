use crate::ScrollView;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    theme_with, use_applied_theme, use_focus, use_platform, ScrollViewThemeWith, TabTheme,
    TabThemeWith,
};
use winit::window::CursorIcon;

#[allow(non_snake_case)]
#[component]
pub fn TabsBar(children: Element) -> Element {
    rsx!(
        ScrollView {
            direction: "horizontal",
            theme: theme_with!(ScrollViewTheme {
                height : "auto".into(),
            }),
            {children}
        }
    )
}

/// Current status of the Tab.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TabStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the button.
    Hovering,
}

#[allow(non_snake_case)]
#[component]
pub fn Tab(children: Element, theme: Option<TabThemeWith>) -> Element {
    let focus = use_focus();
    let mut status = use_signal(TabStatus::default);
    let platform = use_platform();

    let focus_id = focus.attribute();

    let TabTheme {
        background,
        hover_background,
        border_fill,
        focus_border_fill,
        padding,
        margin,
        corner_radius,
        width,
        height,
        font_theme,
        shadow,
    } = use_applied_theme!(&theme, tab);

    use_drop(move || {
        if *status.read() == TabStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Pointer);
        status.set(TabStatus::Hovering);
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        status.set(TabStatus::default());
    };

    let background = match *status.read() {
        TabStatus::Hovering => hover_background,
        TabStatus::Idle => background,
    };
    let border = if focus.is_selected() {
        format!("2 solid {focus_border_fill}")
    } else {
        format!("1 solid {border_fill}")
    };

    rsx!(
        rect {
            onmouseenter,
            onmouseleave,
            focus_id,
            width: "{width}",
            height: "{height}",
            padding: "{padding}",
            margin: "{margin}",
            focusable: "true",
            overflow: "clip",
            role: "tab",
            color: "{font_theme.color}",
            shadow: "{shadow}",
            border: "{border}",
            corner_radius: "{corner_radius}",
            background: "{background}",
            text_align: "center",
            main_align: "center",
            cross_align: "center",
            {&children}
        }
    )
}
