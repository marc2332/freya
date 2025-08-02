#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashSet;

use freya::prelude::*;

fn main() {
    launch_with_params(app, "Segmented Button", (600.0, 200.0));
}

fn app() -> Element {
    let mut selected = use_signal(HashSet::new);
    rsx!(
        Body {
            padding: "8",
            spacing: "8",
            main_align: "center",
            cross_align: "center",
            SegmentedButton {
                for i in 0..5 {
                    ButtonSegment {
                        key: "{i}",
                        selected: selected.read().contains(&i),
                        onpress: move |_| {
                            if selected.read().contains(&i) {
                                selected.write().remove(&i);
                            } else {
                                selected.write().insert(i);
                            }
                        },
                        label {
                            "Option {i}"
                        }
                    }
                }
            }
        }
    )
}
