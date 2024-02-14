#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "SnackBar", (400.0, 350.0));
}

fn app() -> Element {
    let animation = use_animation(move |ctx| {
        ctx.with(
            AnimNum::new(0., 100.)
                .time(1650)
                .ease(Ease::Out)
                .function(Function::Sine),
        )
    });
    let progress = animation.read().get().read().as_f32();
    let mut show = use_signal(|| false);

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            Button {
                onclick: move |_| {
                    animation.read().start();
                    show.toggle();
                },
                label { "Install" }
            }
            SnackBarContainer {
                show,
                SnackBar {
                    ProgressBar {
                        show_progress: true,
                        progress: progress
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn SnackBarContainer(children: Element, show: Signal<bool>) -> Element {
    let animation = use_animation(|ctx| {
        ctx.with(
            AnimNum::new(50., 0.)
                .time(200)
                .ease(Ease::Out)
                .function(Function::Expo),
        )
    });

    use_effect(move || {
        if *show.read() {
            animation.read().start();
        } else if animation.read().peek_has_run_yet() {
            animation.read().reverse();
        }
    });

    let offset_y = animation.read().get().read().as_f32();
    println!("{offset_y:?}");

    rsx!(
        rect {
            width: "100%",
            height: "40",
            position: "absolute",
            position_bottom: "0",
            offset_y: "{offset_y}",
            {children}
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn SnackBar(children: Element) -> Element {
    rsx!(
        rect {
            width: "fill",
            height: "40",
            background: "rgb(103, 80, 164)",
            overflow: "clip",
            shadow: "0 -2 7 0 rgb(0, 0, 0, 0.2)",
            padding: "10",
            color: "white",
            direction: "horizontal",
            {children}
        }
    )
}
