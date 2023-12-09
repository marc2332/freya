#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use rand::Rng;
use std::borrow::Cow;

fn main() {
    launch_with_props(app, "Counter", (400.0, 350.0));
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    let base_padding = 10;
    let mut padding = String::with_capacity(10);
    padding.push_str("10 ");
    padding.push_str(&(base_padding + (count.get() * 5)).to_string());
    padding.push(' ');
    padding.push_str("10 ");
    padding.push_str(&(base_padding - (count.get() * 5)).to_string());

    render!(
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            background: "rgb(0, 119, 182)",
            color: "white",
            shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
            label { font_size: "75", font_weight: "bold", "{count}" }
        }
        rect { height: "50%", width: "100%", main_align: "center", cross_align: "center", direction: "horizontal",
            Button {
                theme: ButtonThemeWith {
                    background: Some(Cow::Borrowed("red")),
                    ..Default::default()
                },
                onclick: move |_| count += 1, label { "Increase" }
            }
            Button {
                theme: theme_with!(ButtonTheme {
                    padding: padding.into(),
                    background: "blue".into(),
                    font_theme: theme_with!(FontTheme {
                        color: "white".into(),
                    }),
                }),
                onclick: move |_| count -= 1, label { "Decrease" }
            }
        }
    )
}
