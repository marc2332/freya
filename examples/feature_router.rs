#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_router::prelude::{
    Routable,
    RouterConfig,
    RouterContext,
    outlet,
    router,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    router::<Route>(|| RouterConfig::default().with_initial_path(Route::Settings))
}

#[derive(PartialEq)]
struct Layout;
impl Component for Layout {
    fn render(&self) -> impl IntoElement {
        rect().center().expanded().child(outlet::<Route>())
    }
}

#[derive(PartialEq)]
struct Home {}
impl Component for Home {
    fn render(&self) -> impl IntoElement {
        Button::new()
            .on_press(|_| {
                RouterContext::get().replace(Route::Settings);
            })
            .child("Go Settings")
    }
}

#[derive(PartialEq)]
struct Settings {}
impl Component for Settings {
    fn render(&self) -> impl IntoElement {
        Button::new()
            .on_press(|_| {
                RouterContext::get().replace(Route::Home);
            })
            .child("Go Home")
    }
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home,
        #[route("/settings")]
        Settings,
}
