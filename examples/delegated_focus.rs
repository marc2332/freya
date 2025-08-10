#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    core::accessibility::AccessibilityFocusStrategy,
    prelude::*,
};

fn main() {
    launch_with_params(app, "Controlled Focus", (400.0, 350.0));
}

fn app() -> Element {
    let nodes = use_hook(|| {
        [
            UseFocus::new_id(),
            UseFocus::new_id(),
            UseFocus::new_id(),
            UseFocus::new_id(),
        ]
    });
    let platform = use_platform();
    let mut current = use_signal(|| 0);

    let onwheel = move |_| {
        current += 1;
        if current() == 4 {
            current.set(0);
        }
    };

    use_effect(move || {
        platform.request_focus(AccessibilityFocusStrategy::Node(nodes[current()]));
    });

    rsx!(
        rect {
            height: "100%",
            width: "100%",
            background: "black",
            color: "white",
            direction: "horizontal",
            main_align: "center",
            cross_align: "center",
            onwheel,
            for (i, id) in nodes.iter().enumerate() {
                Card {
                    key: "{i}",
                    id: *id,
                    index: i
                }
            }
        }
    )
}

#[component]
fn Card(index: usize, id: AccessibilityId) -> Element {
    let focus = use_focus_for_id(id);
    let background = if focus.is_focused() {
        "rgb(0, 119, 182)"
    } else {
        "black"
    };

    rsx!(
        rect {
            height: "100",
            width: "100",
            a11y_id: focus.attribute(),
            background,
            label {
                "Card {index}"
            }
        }
    )
}
