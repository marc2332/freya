#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_router::prelude::{Outlet, Routable, Router};
use freya::elements;
use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
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

#[component]
fn Layout(cx: Scope) -> Element {
    render! {
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

#[component]
fn Cats(cx: Scope) -> Element {
    render! {
        label {
            "Search for cats with DuckDuckGo: "
        }
        Link {
            to: "https://duckduckgo.com/?q=cat",
            tooltip: LinkTooltip::Custom("Cats!"),
            label { "DuckDuckGo search (custom tooltip)" }
        }
    }
}

#[component]
fn Dogs(cx: Scope) -> Element {
    render! {
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
fn Bears(cx: Scope) -> Element {
    render! {
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
fn NotFound(cx: Scope) -> Element {
    render! {
        label {
            "404!! 😵"
        }
    }
}
