use crate::theme::get_theme;
use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{ScrollBarTheme, ScrollBarThemeWith};
use std::borrow::Cow;

#[derive(Props)]
pub struct ScrollBarProps<'a> {
    /// Theme override.
    pub theme: Option<ScrollBarThemeWith>,
    pub children: Element<'a>,
    #[props(into)]
    pub width: String,
    #[props(into)]
    pub height: String,
    pub clicking_scrollbar: bool,
}

enum ScrollBarStatus {
    Idle,
    Hovering,
}

#[allow(non_snake_case)]
pub fn ScrollBar<'a>(cx: Scope<'a, ScrollBarProps<'a>>) -> Element<'a> {
    let status = use_state(cx, || ScrollBarStatus::Idle);
    let ScrollBarTheme {
        background,
        offset_x,
        offset_y,
        ..
    } = get_theme!( cx, &cx.props.theme, scroll_bar );

    let onmouseenter = |_| status.set(ScrollBarStatus::Hovering);
    let onmouseleave = |_| status.set(ScrollBarStatus::Idle);

    let background = match status.get() {
        _ if cx.props.clicking_scrollbar => background,
        ScrollBarStatus::Hovering => background,
        ScrollBarStatus::Idle => Cow::Borrowed("transparent"),
    };

    render!(
        rect {
            overflow: "clip",
            role: "scrollBar",
            width: "{cx.props.width}",
            height: "{cx.props.height}",
            offset_x: "{offset_x}",
            offset_y: "{offset_y}",
            background: "{background}",
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,
            &cx.props.children
        }
    )
}
