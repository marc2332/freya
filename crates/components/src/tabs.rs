use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{
    use_activable_route, use_applied_theme, use_focus, use_platform, TabTheme, TabThemeWith,
};
use winit::window::CursorIcon;

#[allow(non_snake_case)]
#[component]
pub fn TabsBar(children: Element) -> Element {
    rsx!(
        rect {
            direction: "horizontal",
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
    let is_active = use_activable_route();

    let focus_id = focus.attribute();

    let TabTheme {
        background,
        hover_background,
        border_fill,
        focus_border_fill,
        padding,
        width,
        height,
        font_theme,
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
    let border = if focus.is_selected() || is_active {
        focus_border_fill
    } else {
        border_fill
    };

    rsx!(
        rect {
            onmouseenter,
            onmouseleave,
            focus_id,
            width: "{width}",
            height: "{height}",
            focusable: "true",
            overflow: "clip",
            role: "tab",
            color: "{font_theme.color}",
            background: "{background}",
            text_align: "center",
            content: "fit",
            rect {
                padding: "{padding}",
                main_align: "center",
                cross_align: "center",
                {children},
            }
            rect {
                height: "2",
                width: "fill-min",
                background: "{border}"
            }
        }
    )
}
