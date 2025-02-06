#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut values = use_signal(|| (String::new(), String::new()));
    use_init_theme(|| DARK_THEME);

    rsx!(
        Body {
            padding: "10",
            spacing: "10",
            label {
                "Your name:"
            }
            Input {
                value: values().0,
                placeholder: "Enter your name...",
                width: "fill",
                onchange: move |txt| {
                    values.write().0 = txt;
                }
            }
            label {
                "Your age:"
            }
            Input {
                value: values().1,
                placeholder: "Enter your age...",
                width: "fill",
                onvalidate: |validator: InputValidator| {
                    validator.set_valid(validator.text().parse::<u8>().is_ok())
                },
                onchange: move |txt| {
                    values.write().1 = txt;
                }
            },
            rect {
                width: "fill",
                content: "flex",
                direction: "horizontal",
                spacing: "10",
                Button {
                    theme: theme_with!(ButtonTheme {
                        width: "flex(1)".into(),
                    }),
                    onpress: move |_| {
                        *values.write() = (String::new(), String::new());
                    },
                    label {
                        "Clear"
                    }
                }
                FilledButton {
                    theme: theme_with!(ButtonTheme {
                        width: "flex(1)".into(),
                    }),
                    label {
                        "Submit"
                    }
                }
            }
        }
    )
}
