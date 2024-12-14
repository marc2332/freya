#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app)
}

fn app() -> Element {
    let values = use_signal(|| ["Hello, World!"].repeat(150));

    rsx!(VirtualScrollView {
        length: values.read().len(),
        item_size: 70.0,
        direction: "vertical",
        builder: move |index, _: &Option<()>| {
            let value = values.read()[index];
            rsx! {
                AnimatedContainer {
                    key: "{index}",
                    height: 70.0,
                    rect {
                        width: "100%",
                        height: "100%",
                        background: "rgb(235, 235, 235)",
                        corner_radius: "16",
                        padding: "4 10",
                        label {
                            height: "25",
                            "{index} {value}"
                        }
                        label {
                            "Cool!"
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn AnimatedContainer(height: f32, children: Element) -> Element {
    let animations = use_animation(|ctx| {
        ctx.auto_start(true);
        ctx.with(
            AnimNum::new(350., 0.)
                .time(500)
                .ease(Ease::InOut)
                .function(Function::Expo),
        )
    });

    let pos = animations.get();

    rsx!(
        rect {
            offset_x: "{pos.read().as_f32()}",
            height: "{height}",
            width: "100%",
            padding: "4",
            {children}
        }
    )
}
