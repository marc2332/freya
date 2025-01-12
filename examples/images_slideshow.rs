#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use reqwest::Url;

fn main() {
    launch_with_props(app, "Animation", (400.0, 350.0));
}

#[component]
fn Card(selected: ReadOnlySignal<bool>, children: Element) -> Element {
    let animations = use_animation(move |conf| {
        conf.on_deps_change(OnDepsChange::Rerun);
        conf.auto_start(true);
        let (from, to) = if selected() {
            (1.0, 3.0)
        } else {
            (3.0, 1.0)
        };
        AnimNum::new(from, to).time(250).ease(Ease::Out).function(Function::Expo)
    });

    let width = animations.get().read().read();

    rsx!(
        rect {
            corner_radius: "16",
            height: "100%",
            width: "flex({width})",
            overflow: "clip",
            {children}
        }
    )
}

fn app() -> Element {
    let mut selected = use_signal(|| 0);

    let onwheel = move |_| {
        *selected.write() += 1;
        if selected() == 3 {
            selected.set(0)
        }
    };

    rsx!(
        rect {
            onwheel,
            content: "flex",
            direction: "horizontal",
            spacing: "5",
            width: "100%",
            padding: "5",
            
            for (i, url) in [
                "https://images.dog.ceo/breeds/dachshund/dachshund-2033796_640.jpg",
                "https://images.dog.ceo/breeds/cavapoo/doggo4.jpg",
                "https://images.dog.ceo/breeds/wolfhound-irish/n02090721_3109.jpg"
            ].iter().enumerate() {
                Card {
                    key: "{i}",
                    selected: i == selected(),
                    NetworkImage {
                        url: url.parse::<Url>().unwrap(),
                        aspect_ratio: "max",
                        theme: theme_with!(NetworkImageTheme {
                            height: "100%".into(),
                        })
                    }
                }
            }
        }
    )
}
