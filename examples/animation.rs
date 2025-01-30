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
    let animation = use_animation(|_conf| {
        (
            AnimNum::new(100., 200.)
                .time(500)
                .ease(Ease::Out)
                .function(Function::Expo),
            AnimColor::new("rgb(131, 111, 255)", "rgb(255, 167, 50)")
                .time(170)
                .ease(Ease::InOut),
            AnimNum::new(0., 360.)
                .time(1000)
                .ease(Ease::Out)
                .function(Function::Bounce),
            AnimNum::new(50., 0.)
                .time(550)
                .ease(Ease::InOut)
                .function(Function::Bounce),
            AnimNum::new(2., 1.)
                .time(1300)
                .ease(Ease::Out)
                .function(Function::Bounce),
        )
    });

    let (size, background, rotate, radius, scale) = &*animation.get().read_unchecked();

    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            height: "100%",
            width: "100%",
            onclick: move |_| {
                if *toggle.peek() {
                    animation.start();
                } else {
                    animation.reverse();
                }
                toggle.toggle();
            },
            rect {
                scale: "{scale.read()}",
                width: "{size.read()}",
                rotate: "{rotate.read()}deg",
                height: "50%",
                background: "{background.read()}",
                corner_radius: "{radius.read()}"
            }
        }
    )
}
