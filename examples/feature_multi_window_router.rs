#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    prelude::*,
    router::*,
};

fn main() {
    let router = RouterContext::create_global::<Route>(RouterConfig::default());

    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_root_context(router)))
}

fn app() -> impl IntoElement {
    Outlet::<Route>::new()
}

#[derive(PartialEq)]
struct Home {}
impl Component for Home {
    fn render(&self) -> impl IntoElement {
        let on_open = move |_| {
            let router = RouterContext::get();
            spawn(async move {
                let _ = Platform::get()
                    .launch_window(WindowConfig::new(app).with_root_context(router))
                    .await;
            });
        };

        let on_change_route = |_| {
            let _ = RouterContext::get().push(Route::SecondPage);
        };

        rect()
            .expanded()
            .center()
            .spacing(6.)
            .child(Button::new().on_press(on_open).child("Open another window"))
            .child(
                Button::new()
                    .on_press(on_change_route)
                    .child("Go to Second Page in all windows"),
            )
    }
}

#[derive(PartialEq)]
struct SecondPage {}
impl Component for SecondPage {
    fn render(&self) -> impl IntoElement {
        rect().expanded().center().child(
            Button::new()
                .on_press(|_| {
                    let _ = RouterContext::get().replace(Route::Home);
                })
                .child("Go Home"),
        )
    }
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home,
    #[route("/second-page")]
    SecondPage,
}
