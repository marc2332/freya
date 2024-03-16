#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_title(app, "Radio");
}

#[derive(PartialEq)]
enum Choice {
    FirstChoice,
    SecondChoice,
    ThirdChoice,
}

fn app() -> Element {
    let mut selected = use_signal(|| Choice::FirstChoice);

    rsx!(
        Tile {
            onselect: move |_| selected.set(Choice::FirstChoice),
            leading: rsx!(
                Radio {
                    selected: *selected.read() == Choice::FirstChoice,
                },
            ),
            label { "First choice" }
        }
        Tile {
            onselect: move |_| selected.set(Choice::SecondChoice),
            leading: rsx!(
                Radio {
                    selected: *selected.read() == Choice::SecondChoice,
                },
            ),
            label { "Second choice" }
        }
        Tile {
            onselect: move |_| selected.set(Choice::ThirdChoice),
            leading: rsx!(
                Radio {
                    selected: *selected.read() == Choice::ThirdChoice,
                },
            ),
            label { "Third choice" }
        }
    )
}
