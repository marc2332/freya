#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use freya::prelude::*;
use freya_i18n::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut i18n = use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("./i18n/en-US.ftl")))
            .with_locale((langid!("es-ES"), PathBuf::from("./examples/i18n/es-ES.ftl")))
    });

    let change_to_english = move |_| i18n.set_language(langid!("en-US"));
    let change_to_spanish = move |_| i18n.set_language(langid!("es-ES"));

    rect()
        .expanded()
        .center()
        .spacing(6.)
        .child(
            rect()
                .spacing(6.)
                .horizontal()
                .child(Button::new().on_press(change_to_english).child("English"))
                .child(Button::new().on_press(change_to_spanish).child("Spanish")),
        )
        .child(t!("hello_world"))
        .child(t!("hello", name: "Freya!"))
}
