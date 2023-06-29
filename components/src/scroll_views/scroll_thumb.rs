use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::use_get_theme;

#[derive(Props)]
pub struct ScrollThumbProps<'a> {
    onmousedown: EventHandler<'a, MouseEvent>,
    #[props(into)]
    width: String,
    #[props(into)]
    height: String,
}

#[allow(non_snake_case)]
pub fn ScrollThumb<'a>(cx: Scope<'a, ScrollThumbProps<'a>>) -> Element<'a> {
    let theme = use_get_theme(cx);
    let scrollbar_theme = &theme.scrollbar;
    render!(
        rect {
            onmousedown: |e| {
                cx.props.onmousedown.call(e);
            },
            width: "{cx.props.width}",
            height: "{cx.props.height}",
            padding: "2",
            rect {
                width: "100%",
                height: "100%",
                corner_radius: "8",
                background: "{scrollbar_theme.thumb_background}",
            }
        }
    )
}
