#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{common::EventMessage, prelude::*};
use skia_safe::{Color, Font, FontStyle, Paint, Typeface};
use winit::event_loop::EventLoopProxy;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let event_loop_proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
    let mut state = use_state(cx, || 0);

    use_effect(cx, (state,), move |_| async move {
        if let Some(event_loop_proxy) = &event_loop_proxy {
            event_loop_proxy
                .send_event(EventMessage::RequestRerender)
                .unwrap();
        }
    });

    let canvas = use_canvas(cx, || {
        to_owned![state];
        Box::new(move |canvas, region| {
            canvas.translate((region.min_x(), region.min_y()));

            let mut text_paint = Paint::default();
            text_paint.set_anti_alias(true);
            text_paint.set_color(Color::WHITE);
            let font = Font::new(
                Typeface::from_name("Inter", FontStyle::default()).unwrap(),
                50.0,
            );

            canvas.draw_str(
                format!("value is {}", state.current()),
                ((region.max_x() / 2.0 - 120.0), region.max_y() / 2.0),
                &font,
                &text_paint,
            );

            canvas.restore();
        })
    });

    render!(
        rect {
            onclick: move |_| {
                state += 1;
            },
            Canvas {
                canvas: canvas,
                background: "black",
                width: "100%",
                height: "100%"
            }
        }
    )
}
