#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{sync::{Arc, Mutex}, collections::HashMap, time::Duration};

use dioxus_core::AttributeValue;
use dioxus_utils::use_channel::{use_channel, use_listen_channel};
use freya::{
    common::{Area, EventMessage},
    prelude::*,
};
use freya_node_state::CanvasReference;
use skia_safe::{
    runtime_effect::{Uniform, ChildPtr}, Canvas, Color, Data, DataTable, Font, FontStyle, Paint, Rect,
    RuntimeEffect, Shader, Typeface, V3, V4, V2, Image, SamplingOptions,
};
use tokio::time::{Interval, interval, sleep};
use uuid::Uuid;
use winit::event_loop::EventLoopProxy;

fn main() {
    launch(app);
}

struct UseCanvas {
    id: Uuid,
    renderer: Arc<Box<dyn Fn(&mut Canvas, Area) -> ()>>,
}

impl PartialEq for UseCanvas {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl UseCanvas {
    pub fn attribute<'a, T>(&self, cx: Scope<'a, T>) -> AttributeValue<'a> {
        cx.any_value(CustomAttributeValues::Canvas(CanvasReference {
            runner: self.renderer.clone(),
        }))
    }
}

fn use_canvas(
    cx: &ScopeState,
    renderer: impl FnOnce() -> Box<dyn Fn(&mut Canvas, Area) -> ()>,
) -> UseCanvas {
    let id = cx.use_hook(Uuid::new_v4);
    let renderer = cx.use_hook(|| Arc::new(renderer()));

    UseCanvas {
        id: id.clone(),
        renderer: renderer.clone(),
    }
}

#[derive(Props, PartialEq)]
struct CanvasProps {
    #[props(default = "300".to_string(), into)]
    width: String,

    #[props(default = "150".to_string(), into)]
    height: String,

    #[props(default = "white".to_string(), into)]
    background: String,

    canvas: UseCanvas,
}

#[allow(non_snake_case)]
fn Canvas(cx: Scope<CanvasProps>) -> Element {
    render!(container {
        canvas_reference: cx.props.canvas.attribute(cx),
        background: "{cx.props.background}",
        width: "{cx.props.width}",
        height: "{cx.props.height}",
    })
}

const SHADER: &str = "
uniform float u_time;

vec4 main(vec2 test) {
	return vec4(abs(sin(u_time)),0.0,0.0,1.0);
}
";

#[derive(Default)]
struct UniformsBuilder {
    uniforms: HashMap<String, UniformValue>
}

enum UniformValue {
    FloatVec(Vec<f32>),
    Float3(V3),
    Float(f32),
    Float4(V4),
}

impl UniformsBuilder {
    pub fn set(&mut self, name: &str, value: UniformValue){
        self.uniforms.insert(name.to_string(), value);
    }

    pub fn build(&self, shader: &RuntimeEffect) -> Vec<u8> {
        let mut val = Vec::new();

        for uniform in shader.uniforms().iter() {
            let value = self.uniforms.get(uniform.name()).unwrap();
            match &value {
                UniformValue::Float(f) => {
                    val.extend(f.to_be_bytes());
                }
                UniformValue::FloatVec(f) => {
                    for n in f {
                        val.extend(n.to_be_bytes());
                    }
                   
                }
                UniformValue::Float3(f) => {
                    let data = f.as_array();
                    for n in data {
                        val.extend(n.to_be_bytes());
                    }
                   
                }
                UniformValue::Float4(f) => {
                    let data = f.as_array();
                    for n in data {
                        val.extend(n.to_be_bytes());
                    }
                }
            }
            
           
            let offset_end = uniform.offset() + uniform.size_in_bytes();
            println!(">>> {:?} to {:?} of type {:?}", uniform.offset(), offset_end, uniform.ty());
            /* if val.len() < offset_end {
                val.extend(vec![0; offset_end - val.len() - 1])
            } */

        }
        println!("{:?}", val);
        val
    }
}

fn app(cx: Scope) -> Element {
    let event_loop_proxy = cx.consume_context::<EventLoopProxy<EventMessage>>();
    let channel = use_channel::<()>(cx, 5);

    use_listen_channel(cx, &channel , move |_| {
        to_owned![event_loop_proxy];
        async move {
            sleep(Duration::from_millis(100)).await;
            if let Some(event_loop_proxy) = &event_loop_proxy {
                event_loop_proxy
                    .send_event(EventMessage::RequestRerender)
                    .unwrap();
            }
        }
    });

    let canvas = use_canvas(cx, || {
        let channel = channel.clone();
        let shader = RuntimeEffect::make_for_shader(SHADER, None).unwrap();
        let shader = Arc::new(Mutex::new(shader));
        let counter = Arc::new(Mutex::new(0.0));
        Box::new(move |canvas, region| {

            let mut builder = UniformsBuilder::default();
            builder.set("u_time", UniformValue::Float(*counter.lock().unwrap()));

            *counter.lock().unwrap() += 0.1;

            let uniforms = Data::new_copy(&builder.build(&shader.lock().unwrap()));

            let shader = shader.lock().unwrap().make_shader(uniforms, &[], None).unwrap();

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

            channel.send(()).unwrap();
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
