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
    FirstChoice,
    SecondChoice,
    ThirdChoice,
}

fn app() -> Element {
    let mut selected = use_signal::<HashSet<Choice>>(HashSet::default);

    rsx!(
        Tile {
            onselect: move |_| {
                if selected.read().contains(&Choice::FirstChoice) {
                    selected.write().remove(&Choice::FirstChoice);
                } else {
                    selected.write().insert(Choice::FirstChoice);
                }
            },
            leading: rsx!(
                Checkbox {
                    selected: selected.read().contains(&Choice::FirstChoice),
                },
            ),
            label { "First choice" }
        }
        Tile {
            onselect: move |_| {
                if selected.read().contains(&Choice::SecondChoice) {
                    selected.write().remove(&Choice::SecondChoice);
                } else {
                    selected.write().insert(Choice::SecondChoice);
                }
            },
            leading: rsx!(
                Checkbox {
                    selected: selected.read().contains(&Choice::SecondChoice),
                },
            ),
            label { "Second choice" }
        }
        Tile {
            onselect: move |_| {
                if selected.read().contains(&Choice::ThirdChoice) {
                    selected.write().remove(&Choice::ThirdChoice);
                } else {
                    selected.write().insert(Choice::ThirdChoice);
                }
            },
            leading: rsx!(
                Checkbox {
                    selected: selected.read().contains(&Choice::ThirdChoice),
                },
            ),
            label { "Third choice" }
        }
    )
}
