#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    sync::Arc,
    time::Instant,
};

use freya::prelude::*;
use skia_safe::{
    Color,
    Data,
    Paint,
    Rect,
    RuntimeEffect,
};

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

fn app() -> Element {
    let platform = use_platform();
    let (reference, size) = use_node_signal();

    use_hook(|| {
        let mut ticker = platform.new_ticker();

        spawn(async move {
            loop {
                ticker.tick().await;
                platform.invalidate_drawing_area(size.peek().area);
                platform.request_animation_frame();
            }
        });
    });

    let canvas = use_canvas(|| {
        let shader = RuntimeEffect::make_for_shader(SHADER, None).unwrap();
        let shader_wrapper = Arc::new(ShaderWrapper(shader));
        let instant = Instant::now();

        Box::new(move |canvas, _, region, _| {
            let mut builder = UniformsBuilder::default();
            builder.set(
                "u_resolution",
                UniformValue::FloatVec(vec![region.width(), region.height()]),
            );
            builder.set(
                "u_time",
                UniformValue::Float(instant.elapsed().as_secs_f32()),
            );

            let uniforms = Data::new_copy(&builder.build(&shader_wrapper.0));

            let shader = shader_wrapper.0.make_shader(uniforms, &[], None).unwrap();

            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_color(Color::WHITE);
            paint.set_shader(shader);

            canvas.draw_rect(
                Rect::new(
                    region.min_x(),
                    region.min_y(),
                    region.max_x(),
                    region.max_y(),
                ),
                &paint,
            );
        })
    });

    rsx!(rect {
        canvas_reference: canvas.attribute(),
        reference,
        background: "black",
        width: "100%",
        height: "100%",
    })
}

struct ShaderWrapper(RuntimeEffect);

unsafe impl Sync for ShaderWrapper {}
unsafe impl Send for ShaderWrapper {}
