#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::path::PathBuf;

use freya::prelude::*;
use freya_i18n::{
    i18n::{
        I18nConfig,
        use_init_i18n,
    },
    prelude::langid,
    t,
};

fn main() {
    launch(
        LaunchConfig::new()
            .with_window(WindowConfig::new(app))
            .with_window(WindowConfig::new(app)),
    )
}

fn app() -> impl IntoElement {
    // The first window creates the I18n instance, the rest reuse it
    let mut i18n = use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("./i18n/en-US.ftl")))
            .with_locale((langid!("es-ES"), PathBuf::from("./examples/i18n/es-ES.ftl")))
    });

    rect()
        .expanded()
        .center()
        .spacing(6.)
        .child(t!("hello_world"))
        .child(
            Button::new()
                .on_press(move |_| i18n.set_language(langid!("en-US")))
                .child("English"),
        )
        .child(
            Button::new()
                .on_press(move |_| i18n.set_language(langid!("es-ES")))
                .child("Spanish"),
        )
}
