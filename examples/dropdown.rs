use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let values = use_hook(|| {
        vec![
            "First Option".to_string(),
            "Second Option".to_string(),
            "Rust".to_string(),
        ]
    });
    let mut selected_dropdown = use_signal(|| "First Option".to_string());
    rsx!(
        Dropdown {
            value: selected_dropdown.read().clone(),
            for ch in values {
                DropdownItem {
                    value: ch.clone(),
                    onpress: {
                        to_owned![ch];
                        move |_| selected_dropdown.set(ch.clone())
                    },
                    label { "{ch}" }
                }
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
    )
}

#[component]
pub fn MyButton(children: Element, onpress: Option<EventHandler<PressEvent>>) -> Element {
    rsx!(rect {
        Button {
            onpress: onpress,
            rect {
                // Comment this out and suddenly the button can't be clicked when behind a dropdown.
                // background: "red",

                {children}
            }
        }
    })
}
