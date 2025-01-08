#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Form", (300.0, 250.0));
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum FormEntry {
    Name,
    Description,
}

impl std::fmt::Display for FormEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Description => f.write_str("Description"),
            Self::Name => f.write_str("Name"),
        }
    }
}

fn app() -> Element {
    let form = use_form(|data| {
        println!("Submitting: {data:?}");
    });

    rsx!(
        Input {
            ..form.input(FormEntry::Name)
        }
        Input {
            ..form.input(FormEntry::Description)
        }
        Button {
            children: rsx!(
                label {
                    "Submit"
                }
            ),
            ..form.submit(),
        }
    )
}
