#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::path::PathBuf;

use freya::{
    prelude::*,
    radio::*,
    router::*,
};
use freya_i18n::{
    i18n::{
        I18n,
        I18nConfig,
        use_share_i18n,
    },
    prelude::langid,
    t,
};

fn main() {
    let i18n = I18n::create_global(
        I18nConfig::new(langid!("en-US"))
            .with_locale((langid!("en-US"), include_str!("./i18n/en-US.ftl")))
            .with_locale((langid!("es-ES"), PathBuf::from("./examples/i18n/es-ES.ftl"))),
    )
    .unwrap();
    let radio_station = RadioStation::create_global(Data::default());
    let router = RouterContext::create_global::<Route>(RouterConfig::default());

    launch(
        LaunchConfig::new().with_window(WindowConfig::new(AppComponent::new(App {
            radio_station,
            i18n,
            router,
        }))),
    )
}

#[derive(Default)]
struct Data {
    pub count: i32,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
pub enum DataChannel {
    Count,
}

impl RadioChannel<Data> for DataChannel {}

struct App {
    radio_station: RadioStation<Data, DataChannel>,
    i18n: I18n,
    router: RouterContext,
}

impl Component for App {
    fn render(&self) -> impl IntoElement {
        use_share_radio(move || self.radio_station);
        use_share_i18n(move || self.i18n);
        use_share_router(move || self.router);

        Outlet::<Route>::new()
    }
}

struct SubApp {
    radio_station: RadioStation<Data, DataChannel>,
    i18n: I18n,
    router: RouterContext,
}

impl Component for SubApp {
    fn render(&self) -> impl IntoElement {
        use_share_radio(move || self.radio_station);
        use_share_i18n(move || self.i18n);
        let mut radio = use_radio(DataChannel::Count);

        let router = self.router;

        let on_increase = move |_| {
            radio.write().count += 1;
        };

        let on_change_route = move |_| {
            router.push(Route::SecondPage);
        };

        rect()
            .expanded()
            .center()
            .spacing(6.)
            .child(t!("value_is", count: radio.read().count))
            .child(Button::new().on_press(on_increase).child("Increase"))
            .child(
                Button::new()
                    .on_press(on_change_route)
                    .child("Go to Second Page in other windows"),
            )
    }
}

#[derive(PartialEq)]
struct Home {}
impl Component for Home {
    fn render(&self) -> impl IntoElement {
        let radio_station = use_radio_station();
        let on_press = move |_| {
            spawn(async move {
                let _ = Platform::get()
                    .launch_window(WindowConfig::new(AppComponent::new(SubApp {
                        radio_station,
                        i18n: I18n::get(),
                        router: RouterContext::get(),
                    })))
                    .await;
            });
        };

        rect()
            .expanded()
            .center()
            .child(Button::new().on_press(on_press).child("Open"))
    }
}

#[derive(PartialEq)]
struct SecondPage {}
impl Component for SecondPage {
    fn render(&self) -> impl IntoElement {
        rect().expanded().center().child(
            Button::new()
                .on_press(|_| {
                    RouterContext::get().replace(Route::Home);
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
