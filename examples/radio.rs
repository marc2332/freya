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
    First,
    Second,
    Third,
}

fn app() -> Element {
    let mut selected = use_signal(|| Choice::First);

    rsx!(
        Tile {
            onselect: move |_| selected.set(Choice::First),
            leading: rsx!(
                Radio {
                    selected: *selected.read() == Choice::First,
                },
            ),
            label { "First choice" }
        }
        Tile {
            onselect: move |_| selected.set(Choice::Second),
            leading: rsx!(
                Radio {
                    selected: *selected.read() == Choice::Second,
                },
            ),
            label { "Second choice" }
        }
        Tile {
            onselect: move |_| selected.set(Choice::Third),
            leading: rsx!(
                Radio {
                    selected: *selected.read() == Choice::Third,
                },
            ),
            label { "Third choice" }
        }
    )
}
