use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;
use freya_hooks::use_get_theme;

#[derive(Props)]
pub struct ScrollThumbProps<'a> {
    clicking_scrollbar: bool,
    onmousedown: EventHandler<'a, MouseEvent>,
    #[props(into)]
    width: String,
    #[props(into)]
    height: String,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ScrollThumbState {
    #[default]
    Idle,
    // Thumb is being hovered
    Hovering,
}

#[allow(non_snake_case)]
pub fn ScrollThumb<'a>(cx: Scope<'a, ScrollThumbProps<'a>>) -> Element<'a> {
    let theme = use_get_theme(cx);
    let state = use_state(cx, ScrollThumbState::default);
    let thumb_background = match state.get() {
        _ if cx.props.clicking_scrollbar => theme.scrollbar.active_thumb_background,
        ScrollThumbState::Idle => theme.scrollbar.thumb_background,
        ScrollThumbState::Hovering => theme.scrollbar.hover_thumb_background,
    };

    render!(
        rect {
            onmouseenter: |_| {
                state.set(ScrollThumbState::Hovering)
            },
            onmouseleave: |_| {
                state.set(ScrollThumbState::Idle)
            },
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
                background: "{thumb_background}",
            }
        }
    )
}
