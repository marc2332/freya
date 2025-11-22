use std::collections::HashSet;

use freya::prelude::*;

#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [90, 95, 99]))]
fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut selected = use_state::<HashSet<i32>>(|| HashSet::from_iter([1, 3, 5]));

    rect()
        .center()
        .expanded()
        .spacing(8.)
        .child(
            rect()
                .spacing(8.)
                .horizontal()
                .children_iter((0..3).map(|i| {
                    Chip::new()
                        .key(i)
                        .selected(selected.read().contains(&i))
                        .on_press(move |_| {
                            if selected.read().contains(&i) {
                                selected.write().remove(&i);
                            } else {
                                selected.write().insert(i);
                            }
                        })
                        .child(format!("Value {i}"))
                        .into()
                })),
        )
        .child(
            rect()
                .spacing(8.)
                .horizontal()
                .children_iter((0..3).map(|i| {
                    Chip::new()
                        .key(i)
                        .enabled(selected.read().contains(&i))
                        .selected(!selected.read().contains(&i))
                        .on_press(move |_| {
                            if selected.read().contains(&i) {
                                selected.write().remove(&i);
                            } else {
                                selected.write().insert(i);
                            }
                        })
                        .child(format!("Value {i}"))
                        .into()
                })),
        )
}
