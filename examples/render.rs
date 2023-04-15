#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::Arc, time::Duration};

use dioxus_core::AttributeValue;
use dioxus_utils::use_channel::{use_channel, use_listen_channel};
use freya::{
    common::{Area, EventMessage},
    prelude::*,
};
use freya_node_state::CanvasReference;
use skia_safe::{Canvas, Color, Font, FontStyle, Paint, PaintStyle, Rect, Typeface};
use tokio::time::sleep;
use winit::event_loop::EventLoopProxy;

fn main() {
    launch(app);
}

struct UseCanvas {
    renderer: Arc<Box<dyn Fn(&mut Canvas, Area) -> ()>>,
}

impl UseCanvas {
    pub fn attribute<'a>(&self, cx: Scope<'a>) -> AttributeValue<'a> {
        cx.any_value(CustomAttributeValues::Canvas(CanvasReference {
            runner: self.renderer.clone(),
        }))
    }
}

fn use_canvas(
    cx: &ScopeState,
    renderer: impl FnOnce() -> Box<dyn Fn(&mut Canvas, Area) -> ()>,
) -> UseCanvas {
    let renderer = cx.use_hook(|| Arc::new(renderer()));

    UseCanvas {
        renderer: renderer.clone(),
    }
}

fn app(cx: Scope) -> Element {
    let event_loop_proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
    let mut state = use_state(cx, || 0);

    let channel = use_channel(cx, 1);

    use_listen_channel(cx, &channel, move |_: ()| {
        to_owned![event_loop_proxy];
        async move {
            sleep(Duration::from_millis(16)).await;
            if let Some(event_loop_proxy) = &event_loop_proxy {
                event_loop_proxy
                    .send_event(EventMessage::RequestRerender)
                    .unwrap();
            }
        }
    });

    let canvas = use_canvas(cx, || {
        to_owned![state, channel];
        Box::new(move |canvas, region| {
            canvas.translate((region.min_x(), region.min_y()));
            let mut bg_paint = Paint::default();
            bg_paint.set_style(PaintStyle::Fill);
            bg_paint.set_color(Color::RED);
            canvas.draw_rect(Rect::new(0.0, 0.0, 200.0, 200.0), &bg_paint);

            let mut text_paint = Paint::default();
            text_paint.set_anti_alias(true);
            text_paint.set_color(Color::WHITE);
            let font = Font::new(
                Typeface::from_name("Inter", FontStyle::default()).unwrap(),
                25.0,
            );
            canvas.draw_str(
                format!("value is {}", state.current()),
                (25.0, 25.0),
                &font,
                &text_paint,
            );

            channel.send(()).unwrap();
        })
    });

    render!(
        rect {
            padding: "100",
            container {
                background: "black",
                onclick: move |_| {
                    state += 1;
                },
                rect {
                    canvas: canvas.attribute(cx),
                    width: "100%",
                    height: "100%",
                }
            }
        }
    )
}
