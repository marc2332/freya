#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_std::{
    i18n::{use_i18, use_init_i18n, Language},
    translate,
};
use freya::prelude::*;
use std::str::FromStr;

fn main() {
    launch_with_props(app, "freya + i18n", (300.0, 200.0));
}

static EN_US: &str = include_str!("./en-US.json");
static ES_ES: &str = include_str!("./es-ES.json");

#[allow(non_snake_case)]
fn Body() -> Element {
    let mut i18 = use_i18();

    let change_to_english = move |_| i18.set_language("en-US".parse().unwrap());
    let change_to_spanish = move |_| i18.set_language("es-ES".parse().unwrap());

    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            width: "100%",
            height: "100%",
            rect {
                label { {translate!(i18, "messages.hello_world")} }
                label { {translate!(i18, "messages.hello", name: "Dioxus")} }
                rect {
                    direction: "horizontal",
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
            }
        }
    )
}

fn app() -> Element {
    use_init_i18n("en-US".parse().unwrap(), "en-US".parse().unwrap(), || {
        let en_us = Language::from_str(EN_US).unwrap();
        let es_es = Language::from_str(ES_ES).unwrap();
        vec![en_us, es_es]
    });

    rsx!(Body {})
}
