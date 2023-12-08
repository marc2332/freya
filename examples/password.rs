#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let password = use_state(cx, || String::new());
    let is_hidden = use_state(cx, || true);
    render!(
        rect { overflow: "clip", padding: "7", width: "100%", height: "100%",
            label { color: "black", "Password:" }
            rect { direction: "horizontal",
                Input {
                    hidden: if !is_hidden { InputMode::Shown } else { InputMode::new_password() },
                    value: password.get().clone(),
                    onchange: |e| { password.set(e) }
                }
                Button { onclick: |_| is_hidden.modify(|v| !v),
                    label {
                        if *is_hidden.get() {
                            "Show password"
                        } else {
                            "Hide password"
                        }
                    }
                }
            }
        }
    )
}
