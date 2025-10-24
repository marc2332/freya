#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

#[component]
fn app() -> Element {
    let mut hovered = use_signal(|| false);

    let animation = use_animation(move |conf| {
        conf.on_creation(OnCreation::Finish);
        conf.on_deps_change(OnDepsChange::Rerun);

        let anim = AnimColor::new("rgb(234, 235, 208)", "rgb(218, 108, 108)").time(150);

        if hovered() {
            anim.into_reversed()
        } else {
            anim
        }
    });

    let background = animation.read().value();

    rsx!(
        rect {
            width: "fill",
            height: "fill",
            background,
            main_align: "center",
            cross_align: "center",
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            label {
                font_size: "24",
                "Hello, World!"
            }
        }
    )
}
