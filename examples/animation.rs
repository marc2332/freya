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
        // The animation will start automatically when the component is mounted
        ctx.auto_start(true); 
        
        // Run the animation in revese direction once an iteration is done
        ctx.on_finish(OnFinish::Reverse);

        // You can register as many animations you want
        (
            // Create an animable numeric value that starts from 0 and goes to 100 in 50ms
            ctx.with(AnimNum::new(0., 100.).time(500).ease(Ease::InOut)), 
            // Create an animable color value that starts from one color and transitions to another in 200ms and has a Bounce function
            ctx.with(
                AnimColor::new("rgb(131, 111, 255)", "rgb(255, 167, 50)")
                    .time(400)
                    .function(Function::Bounce)
            )
        )
    });

    let (width, color) = animation.get();

    rsx!(rect {
        width: "{width.read().as_f32()}%",
        height: "100%",
        background: "{color.read().as_string()}",
    })
}