#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_i18n::{
    prelude::*,
    t,
    unic_langid::langid,
};
use freya::prelude::*;

fn main() {
    launch_with_props(app, "freya + i18n", (300.0, 200.0));
}

#[allow(non_snake_case)]
fn Body() -> Element {
    let mut i18n = i18n();

    let change_to_english = move |_| i18n.set_language(langid!("en-US"));
    let change_to_spanish = move |_| i18n.set_language(langid!("es-ES"));

    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            spacing: "10",
            width: "fill",
            height: "fill",
            rect {
                direction: "horizontal",
                spacing: "10",
                Button {
                    onclick: change_to_english,
                    label {
                        "English"
                    }
                }
                Button {
                    onclick: change_to_spanish,
                    label {
                        "Spanish"
                    }
                }
            }

            label { {t!("hello", name: "Dioxus")} }
        }
    )
}

fn app() -> Element {
    use_init_i18n(|| {
        I18nConfig::new(langid!("en-US"))
            .with_locale(Locale::new_static(
                langid!("en-US"),
                include_str!("./en-US.ftl"),
            ))
            .with_locale(Locale::new_static(
                langid!("es-ES"),
                include_str!("./es-ES.ftl"),
            ))
    });

    rsx!(Body {})
}
