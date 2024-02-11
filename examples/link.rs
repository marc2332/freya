#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::{Outlet, Routable, Router};
use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Cats,
    #[route("/dogs")]
    Dogs,
    #[route("/bears")]
    Bears,
    #[route("/..routes")]
    NotFound
}

#[allow(non_snake_case)]
#[component]
fn Layout() -> Element {
    rsx! {
        rect {
            direction: "horizontal",
            Link {
                to: Route::Cats,
                label { "Cats 🐱" }
            }
            Link {
                to: Route::Dogs,
                label { "Dogs 🐶" }
            }
            Link {
                to: Route::Bears,
                label { "Bears 🐻" }
            }
        }
        rect {
            Outlet::<Route> {}
        }
    }
}

#[allow(non_snake_case)]
#[component]
fn Cats() -> Element {
    rsx! {
        label {
            "Search for cats with DuckDuckGo: "
        }
        Link {
            to: "https://duckduckgo.com/?q=cat",
            tooltip: LinkTooltip::Custom("Cats!".to_string()),
            label { "DuckDuckGo search (custom tooltip)" }
        }
    }
}

#[allow(non_snake_case)]
#[component]
fn Dogs() -> Element {
    rsx! {
        label {
            "Search for dogs with DuckDuckGo: "
        }
        Link {
            to: "https://duckduckgo.com/?q=dog",
            label { "DuckDuckGo search (default tooltip)" }
        }
    }
}

#[component]
fn Bears() -> Element {
    rsx! {
        label {
            "Search for bears with DuckDuckGo: "
        }
        Link {
            to: "https://duckduckgo.com/?q=bear",
            tooltip: LinkTooltip::None,
            label { "DuckDuckGo search (no tooltip)" }
        }
    }
}

#[component]
fn NotFound() -> Element {
    rsx! {
        label {
            "404!! 😵"
        }
    }
}
