#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Animation", (400.0, 350.0));
}

fn app() -> Element {
    let mut toggle = use_signal(|| true);
    let animator = use_animation(|ctx| {
        (
            ctx.with(
                AnimNum::new(100., 200.)
                    .time(500)
                    .ease(Ease::Out)
                    .function(Function::Expo),
            ),
            ctx.with(
                AnimColor::new("rgb(131, 111, 255)", "rgb(255, 167, 50)")
                    .time(170)
                    .ease(Ease::InOut),
            ),
            ctx.with(
                AnimNum::new(0., 360.)
                    .time(1000)
                    .ease(Ease::Out)
                    .function(Function::Bounce),
            ),
            ctx.with(
                AnimNum::new(50., 0.)
                    .time(550)
                    .ease(Ease::InOut)
                    .function(Function::Bounce),
            ),
        )
    });

    let animations = animator.read();
    let (size, color, rotate, radius) = animations.get();

    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            height: "100%",
            width: "100%",
            onclick: move |_| {
                if *toggle.peek() {
                    animator.read().start();
                } else {
                    animator.read().reverse();
                }
                toggle.toggle();
            },
            rect {
                width: "{size.read().as_f32()}",
                rotate: "{rotate.read().as_f32()}deg",
                height: "50%",
                background: "{color.read().as_string()}",
                corner_radius: "{radius.read().as_f32()}"
            }
        }
    )
}
