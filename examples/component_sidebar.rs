#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    prelude::*,
    router::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    Router::<Route>::new(RouterConfig::default)
}

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppSideBar)]
        #[route("/")]
        Home,
        #[route("/settings")]
        Settings,
}

#[derive(PartialEq)]
struct AppSideBar;
impl Component for AppSideBar {
    fn render(&self) -> impl IntoElement {
        NativeRouter::new().child(
            rect()
                .horizontal()
                .child(
                    ScrollView::new().width(Size::px(200.)).child(
                        rect()
                            .theme_background()
                            .padding(8.)
                            .height(Size::fill())
                            .child(
                                ActivableRoute::new(
                                    Route::Home,
                                    Link::new(Route::Home).child(SideBarItem::new().child("Home")),
                                )
                                .exact(true),
                            )
                            .child(ActivableRoute::new(
                                Route::Settings,
                                Link::new(Route::Settings)
                                    .child(SideBarItem::new().child("Settings")),
                            ))
                            .child(
                                SideBarItem::new()
                                    .on_press(|_| {
                                        println!("Pressed 🦀");
                                    })
                                    .child("Crab 🦀"),
                            ),
                    ),
                )
                .child(rect().expanded().center().child(Outlet::<Route>::new())),
        )
    }
}

#[derive(PartialEq)]
struct Home;
impl Component for Home {
    fn render(&self) -> impl IntoElement {
        "Home Page!"
    }
}

#[derive(PartialEq)]
struct Settings;
impl Component for Settings {
    fn render(&self) -> impl IntoElement {
        "Settings Page!"
    }
}
