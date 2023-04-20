#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use dioxus_utils::use_channel::{use_channel, use_listen_channel};
use freya::{common::EventMessage, prelude::*};
use skia_safe::{Color, Data, Paint, Rect, RuntimeEffect};
use tokio::time::sleep;
use winit::event_loop::EventLoopProxy;

fn main() {
    launch(app);
}

const SHADER: &str = "
 uniform vec2 u_resolution;
 uniform float u_time;

 vec4 main(vec2 cords) {
     vec2 U = cords / 55.;
    
     float t = .8* u_time;
     float r = ceil(U.x + t) + ceil(U.y + t);
     float v = mod(r, 4.) > 1. ? U.x : U.y;
     float b = step(fract(v+.2), .5);
    
     vec4 C = vec4(.9*b, 0. + abs(sin(t) * 0.5), .6-b, 1.);
     return C;
 }
 ";

#[derive(Default)]
struct UniformsBuilder {
    uniforms: HashMap<String, UniformValue>,
}

enum UniformValue {
    Float(f32),
    #[allow(dead_code)]
    FloatVec(Vec<f32>),
}

impl UniformsBuilder {
    pub fn set(&mut self, name: &str, value: UniformValue) {
        self.uniforms.insert(name.to_string(), value);
    }

    pub fn build(&self, shader: &RuntimeEffect) -> Vec<u8> {
        let mut val = Vec::new();

        for uniform in shader.uniforms().iter() {
            let value = self.uniforms.get(uniform.name()).unwrap();
            match &value {
                UniformValue::Float(f) => {
                    val.extend(f.to_le_bytes());
                }
                UniformValue::FloatVec(f) => {
                    for n in f {
                        val.extend(n.to_le_bytes());
                    }
                }
            }
        }
        val
    }
}

fn app(cx: Scope) -> Element {
    let event_loop_proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
    let render_channel = use_channel::<()>(cx, 5);

    use_listen_channel(cx, &render_channel, move |_| {
        to_owned![event_loop_proxy];
        async move {
            sleep(Duration::from_millis(25)).await;
            if let Some(event_loop_proxy) = &event_loop_proxy {
                event_loop_proxy
                    .send_event(EventMessage::RequestRerender)
                    .unwrap();
            }
        }
    });

    let canvas = use_canvas(cx, || {
        let shader = RuntimeEffect::make_for_shader(SHADER, None).unwrap();
        let shader = Arc::new(Mutex::new(shader));
        let instant = Instant::now();
        to_owned![render_channel];
        Box::new(move |canvas, region| {
            let mut builder = UniformsBuilder::default();
            builder.set(
                "u_resolution",
                UniformValue::FloatVec(vec![region.max_x(), region.max_y()]),
            );
            builder.set(
                "u_time",
                UniformValue::Float(instant.elapsed().as_secs_f32()),
            );

            let uniforms = Data::new_copy(&builder.build(&shader.lock().unwrap()));

            let shader = shader
                .lock()
                .unwrap()
                .make_shader(uniforms, &[], None)
                .unwrap();

            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_color(Color::WHITE);
            paint.set_shader(shader);

            canvas.draw_rect(
                Rect::new(
                    region.min_x(),
                    region.min_y(),
                    region.width(),
                    region.height(),
                ),
                &paint,
            );

            render_channel.send(()).unwrap();
        })
    });

    render!(
        rect {
            Canvas {
                canvas: canvas,
                background: "black",
                width: "100%",
                height: "100%"
            }
        }
    )
}
