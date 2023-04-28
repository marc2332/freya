use dioxus::prelude::*;
use freya_common::EventMessage;
use freya_elements::elements as dioxus_elements;
use winit::{event_loop::EventLoopProxy, window::CursorIcon};

#[derive(Props)]
pub struct CursorAreaProps<'a> {
    icon: CursorIcon,

    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn CursorArea<'a>(cx: Scope<'a, CursorAreaProps<'a>>) -> Element<'a> {
    let event_loop_proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
    let icon = cx.props.icon;

    let onmouseover = {
        to_owned![event_loop_proxy];
        move |_| {
            if let Some(event_loop_proxy) = &event_loop_proxy {
                event_loop_proxy
                    .send_event(EventMessage::SetCursorIcon(icon))
                    .unwrap();
            }
        }
    };

    let onmouseleave = move |_| {
        if let Some(event_loop_proxy) = &event_loop_proxy {
            event_loop_proxy
                .send_event(EventMessage::SetCursorIcon(CursorIcon::default()))
                .unwrap();
        }
    };

    render!(
        rect {
            onmouseover: onmouseover,
            onmouseleave: onmouseleave,
            &cx.props.children
        }
    )
}
