use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_hooks::{use_get_theme, ScrollbarTheme};

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
}

#[allow(non_snake_case)]
pub fn ScrollBar<'a>(cx: Scope<'a, ScrollBarProps<'a>>) -> Element<'a> {
    let theme = use_get_theme(cx);
    let ScrollbarTheme { background, .. } = &theme.scrollbar;

    render!(
        rect {
            overflow: "clip",
            role: "scrollBar",
            width: "{cx.props.width}",
            height: "{cx.props.height}",
            offset_x: "{cx.props.offset_x}",
            offset_y: "{cx.props.offset_y}",
            background: "{background}",
            &cx.props.children
        }
    )
}
