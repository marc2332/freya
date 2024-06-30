use std::collections::HashSet;

use freya::prelude::*;

#[derive(PartialEq, Eq, Hash)]
enum Choice {
    First,
    Second,
    Third,
}

#[allow(non_snake_case)]
pub fn DsCheckbox() -> Element {
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
                },
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
                },
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
                },
            ),
            label { "Third choice" }
        }
    )
}

#[allow(non_snake_case)]
pub fn DsRadio() -> Element {
    let mut selected = use_signal(|| Choice::First);

    rsx!(
        Tile {
            onselect: move |_| {
                selected.set(Choice::First);
            },
            leading: rsx!(
                Radio {
                    selected: *selected.read() == Choice::First,
                },
            ),
            label { "First choice" }
        }
        Tile {
            onselect: move |_| {
                selected.set(Choice::Second);
            },
            leading: rsx!(
                Radio {
                    selected: *selected.read() == Choice::Second,
                },
            ),
            label { "Second choice" }
        }
        Tile {
            onselect: move |_| {
                selected.set(Choice::Third);
            },
            leading: rsx!(
                Radio {
                    selected: *selected.read() == Choice::Third,
                },
            ),
            label { "Third choice" }
        }
    )
}
