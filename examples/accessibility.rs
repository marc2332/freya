#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    use_init_focus(cx);
    let focus_a = use_focus(cx);
    let focus_b = use_focus(cx);
    let focus_c = use_focus(cx);
    let focus_d = use_focus(cx);
    render!(
        AccessibilityFocusProvider {},
        rect {
            focus_id: focus_a.attribute(cx),
            background: "rgb(233, 196, 106)",
            padding: "25",
            width: "50%",
            height: "50%",
            onclick: move |_| {
                focus_a.focus();
            },
            label {
                focus_id: focus_c.attribute(cx),
                onclick: move |_| {
                    focus_c.focus();
                },
                "test"
            }
            Button {
                label {
                    "Button"
                }
            }
        }
        rect {
            focus_id: focus_b.attribute(cx),
            background: "rgb(150, 100, 231)",
            padding: "25",
            width: "100%",
            height: "50%",
            onclick: move |_| {
                focus_b.focus();
            },
            label {
                role: "staticText",
                focus_id: focus_d.attribute(cx),
                onclick: move |_| {
                    focus_d.focus();
                },
                color: "white",
                "Hello, World! This is an example."
            }
        }
    )
}
