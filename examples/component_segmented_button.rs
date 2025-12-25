use std::collections::HashSet;

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut selected = use_state::<HashSet<i32>>(HashSet::new);

    rect()
        .center()
        .expanded()
        .spacing(16.)
        .child(SegmentedButton::new().children_iter((0..4).map(|i| {
            ButtonSegment::new()
                .key(i)
                .selected(selected.read().contains(&i))
                .on_press(move |_| {
                    if selected.read().contains(&i) {
                        selected.write().remove(&i);
                    } else {
                        selected.write().insert(i);
                    }
                })
                .child(format!("Option {i}"))
                .into()
        })))
        .child(format!(
            "Selected: {:?}",
            selected.read().iter().collect::<Vec<_>>()
        ))
}
