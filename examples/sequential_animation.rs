#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Counter", (600.0, 350.0));
}


fn app() -> Element {
    let mut toggle = use_signal(|| true);
    let mut animations = use_animation(|ctx| {
        ctx.with(
            AnimSequential::new()
                .with(
                    AnimNum::new(0., 360.)
                        .time(500)
                        .ease(Ease::InOut)
                        .function(Function::Expo),
                )
                .with(
                    AnimNum::new(0., 180.)
                        .time(2000)
                        .ease(Ease::Out)
                        .function(Function::Elastic),
                ),
        )
    });

    let sequential = animations.get();

    let rotate_a = sequential.read().sub(0).as_f32();
    let rotate_b = sequential.read().sub(1).as_f32();

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "center",
            cross_align: "center",
            spacing: "50",
            direction: "horizontal",
            onclick: move |_| {
                if *toggle.peek() {
                    animations.start();
                } else {
                    animations.reverse();
                }
                toggle.toggle();
            },
            rect {
                width: "100",
                height: "100",
                rotate: "{rotate_a}deg",
                background: "rgb(0, 119, 182)"
            },
            rect {
                width: "100",
                height: "100",
                rotate: "{rotate_b}deg",
                background: "rgb(0, 119, 182)"
            }
        }
    )
}
