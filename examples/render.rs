#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use dioxus_core::AttributeValue;
use freya::{
    common::{Area, EventMessage},
    prelude::*,
};
use freya_node_state::CanvasReference;
use skia_safe::{Canvas, Color, Font, FontStyle, Paint, Typeface};
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
        })
    });

    render!(rect {
        background: "black",
        onclick: move |_| {
            state += 1;
        },
        canvas: canvas.attribute(cx),
        width: "100%",
        height: "100%",
    })
}
