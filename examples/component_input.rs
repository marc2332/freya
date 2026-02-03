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

fn inputs(text: State<String>) -> [Element; 9] {
    [
        Input::new(text).placeholder("Normal").into(),
        Input::new(text).placeholder("Filled").filled().into(),
        Input::new(text).placeholder("Flat").flat().into(),
        Input::new(text).placeholder("Expanded").expanded().into(),
        Input::new(text)
            .placeholder("Expanded Filled")
            .expanded()
            .filled()
            .into(),
        Input::new(text)
            .placeholder("Expanded Flat")
            .expanded()
            .flat()
            .into(),
        Input::new(text).placeholder("Compact").compact().into(),
        Input::new(text)
            .placeholder("Compact Filled")
            .compact()
            .filled()
            .into(),
        Input::new(text)
            .placeholder("Compact Flat")
            .compact()
            .flat()
            .into(),
    ]
}

fn disabled_inputs(text: State<String>) -> [Element; 9] {
    [
        Input::new(text).placeholder("Normal").enabled(false).into(),
        Input::new(text)
            .placeholder("Filled")
            .enabled(false)
            .filled()
            .into(),
        Input::new(text)
            .placeholder("Flat")
            .enabled(false)
            .flat()
            .into(),
        Input::new(text)
            .placeholder("Expanded")
            .enabled(false)
            .expanded()
            .into(),
        Input::new(text)
            .placeholder("Expanded Filled")
            .enabled(false)
            .expanded()
            .filled()
            .into(),
        Input::new(text)
            .placeholder("Expanded Flat")
            .enabled(false)
            .expanded()
            .flat()
            .into(),
        Input::new(text)
            .placeholder("Compact")
            .enabled(false)
            .compact()
            .into(),
        Input::new(text)
            .placeholder("Compact Filled")
            .enabled(false)
            .compact()
            .filled()
            .into(),
        Input::new(text)
            .placeholder("Compact Flat")
            .enabled(false)
            .compact()
            .flat()
            .into(),
    ]
}
