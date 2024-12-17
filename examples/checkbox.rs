#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashSet;

use freya::prelude::*;

fn main() {
    launch_with_title(app, "Checkbox");
}

#[derive(PartialEq, Eq, Hash)]
enum Choice {
    First,
    Second,
    Third,
}

fn app() -> Element {
    let mut selected = use_signal::<HashSet<Choice>>(HashSet::default);

    rsx!(
        Tile {
            onselect: move |_| {
                if selected.read().contains(&Choice::First) {
                    selected.write().remove(&Choice::First);
                } else {
                    selected.write().insert(Choice::First);
                }
            },
            leading: rsx!(
                Checkbox {
                    selected: selected.read().contains(&Choice::First),
                }
            ),
            label { "First choice" }
        }
        Tile {
            onselect: move |_| {
                if selected.read().contains(&Choice::Second) {
                    selected.write().remove(&Choice::Second);
                } else {
                    selected.write().insert(Choice::Second);
                }
            },
            leading: rsx!(
                Checkbox {
                    selected: selected.read().contains(&Choice::Second),
                }
            ),
            label { "Second choice" }
        }
        Tile {
            onselect: move |_| {
                if selected.read().contains(&Choice::Third) {
                    selected.write().remove(&Choice::Third);
                } else {
                    selected.write().insert(Choice::Third);
                }
            },
            leading: rsx!(
                Checkbox {
                    selected: selected.read().contains(&Choice::Third),
                }
            ),
            label { "Third choice" }
        }
    )
}
