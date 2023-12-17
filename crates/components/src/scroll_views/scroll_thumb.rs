use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;

use freya_hooks::{use_applied_theme, ScrollBarThemeWith};

#[derive(Props)]
pub struct ScrollThumbProps<'a> {
    /// Theme override.
    #[props(optional)]
    pub theme: Option<ScrollBarThemeWith>,
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
    let theme = use_applied_theme!(cx, &cx.props.theme, scroll_bar);
    let state = use_state(cx, ScrollThumbState::default);
    let thumb_background = match state.get() {
        _ if cx.props.clicking_scrollbar => theme.active_thumb_background,
        ScrollThumbState::Idle => theme.thumb_background,
        ScrollThumbState::Hovering => theme.hover_thumb_background,
    };

    render!(
        rect {
            onmouseenter: |_| { state.set(ScrollThumbState::Hovering) },
            onmouseleave: |_| { state.set(ScrollThumbState::Idle) },
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
