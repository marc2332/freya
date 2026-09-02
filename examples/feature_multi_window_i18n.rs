#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::path::PathBuf;

use freya::{
    i18n::*,
    prelude::*,
};

fn main() {
    let i18n = I18n::create_global(
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("./i18n/en-US.ftl")))
            .with_locale((langid!("es-ES"), PathBuf::from("./examples/i18n/es-ES.ftl"))),
    )
    .expect("Failed to create the i18n instance.");

    launch(
        LaunchConfig::new()
            .with_global(i18n)
            .with_window(WindowConfig::new(app))
            .with_window(WindowConfig::new(app)),
    )
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .spacing(6.)
        .child(t!("hello_world"))
        .child(
            Button::new()
                .on_press(|_| I18n::get().set_language(langid!("en-US")))
                .child("English"),
        )
        .child(
            Button::new()
                .on_press(|_| I18n::get().set_language(langid!("es-ES")))
                .child("Spanish"),
        )
}
