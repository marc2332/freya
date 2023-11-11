use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::use_get_theme;

#[derive(Props)]
pub struct ScrollBarProps<'a> {
    children: Element<'a>,

    #[props(into)]
    width: String,

    #[props(into)]
    height: String,

    #[props(default = "0".to_string(), into)]
    offset_x: String,

    #[props(default = "0".to_string(), into)]
    offset_y: String,

    clicking_scrollbar: bool,
}

enum ScrollBarStatus {
    Idle,
    Hovering,
}

#[allow(non_snake_case)]
pub fn ScrollBar<'a>(cx: Scope<'a, ScrollBarProps<'a>>) -> Element<'a> {
    let status = use_state(cx, || ScrollBarStatus::Idle);
    let theme = use_get_theme(cx);

    let onmouseenter = |_| status.set(ScrollBarStatus::Hovering);

    let onmouseleave = |_| status.set(ScrollBarStatus::Idle);

    let background = match status.get() {
        _ if cx.props.clicking_scrollbar => theme.scrollbar.background,
        ScrollBarStatus::Hovering => theme.scrollbar.background,
        ScrollBarStatus::Idle => "transparent",
    };

    render!(
        rect {
            overflow: "clip",
            role: "scrollBar",
            width: "{cx.props.width}",
            height: "{cx.props.height}",
            offset_x: "{cx.props.offset_x}",
            offset_y: "{cx.props.offset_y}",
            background: "{background}",
            onmouseenter: onmouseenter,
            onmouseleave: onmouseleave,
            &cx.props.children
        }
    )
}
