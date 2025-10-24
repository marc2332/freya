#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_params(app, "Sequential Animation", (600.0, 350.0));
}

fn app() -> Element {
    let mut toggle = use_signal(|| true);
    let animations = use_animation(|_conf| {
        AnimSequential::new([
            AnimNum::new(0., 360.)
                .time(500)
                .ease(Ease::InOut)
                .function(Function::Expo),
            AnimNum::new(0., 180.)
                .time(2000)
                .ease(Ease::Out)
                .function(Function::Elastic),
        ])
    });

    let rotations = animations();
    let rotate_a = rotations[0].value();
    let rotate_b = rotations[1].value();

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
