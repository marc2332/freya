#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let text = use_state(String::new);
    rect()
        .center()
        .expanded()
        .horizontal()
        .spacing(6.)
        .child(rect().spacing(6.).children(inputs(text)))
        .child(rect().spacing(6.).children(disabled_inputs(text)))
}

fn inputs(mut text: State<String>) -> [Element; 9] {
    [
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Normal")
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Filled")
            .filled()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Flat")
            .flat()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Expanded")
            .expanded()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Expanded Filled")
            .expanded()
            .filled()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Expanded Flat")
            .expanded()
            .flat()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Compact")
            .compact()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Compact Filled")
            .compact()
            .filled()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Compact Flat")
            .compact()
            .flat()
            .into(),
    ]
}

fn disabled_inputs(mut text: State<String>) -> [Element; 9] {
    [
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Normal")
            .enabled(false)
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Filled")
            .enabled(false)
            .filled()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Flat")
            .enabled(false)
            .flat()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Expanded")
            .enabled(false)
            .expanded()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Expanded Filled")
            .enabled(false)
            .expanded()
            .filled()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Expanded Flat")
            .enabled(false)
            .expanded()
            .flat()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Compact")
            .enabled(false)
            .compact()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Compact Filled")
            .enabled(false)
            .compact()
            .filled()
            .into(),
        Input::new()
            .value(text)
            .on_change(move |v| text.set(v))
            .placeholder("Compact Flat")
            .enabled(false)
            .compact()
            .flat()
            .into(),
    ]
}
