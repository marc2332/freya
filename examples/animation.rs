#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Animation", (400.0, 350.0));
}

fn app() -> Element {
    let animation = use_animation(|ctx| {
        ctx.auto_start(true);
        ctx.on_finish(OnFinish::Reverse);
        (
            ctx.with(AnimNum::new(0., 100.).time(200)),
            ctx.with(AnimColor::new("red", "blue").time(200))
        )
    });
    
    let (width, color) = animation.get();
    let width = width.read().as_f32();
    let background = color.read().as_string();

    rsx!(
        rect {
            height: "100%",
            width: "{width}",
            background: "{background}"
        }
    )
}
