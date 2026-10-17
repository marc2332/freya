#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::time::Duration;

use freya::{
    prelude::*,
    query::*,
};
fn main() {
    launch(
        LaunchConfig::new()
            .with_window(WindowConfig::new(app))
            .with_window(WindowConfig::new(second_app)),
    )
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct FancyClient;

impl FancyClient {
    pub fn name(&self) -> &'static str {
        "Marc"
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct GetUserName(Captured<FancyClient>);

impl QueryCapability for GetUserName {
    type Ok = String;
    type Err = ();
    type Keys = usize;

    async fn run(&self, user_id: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        async_io::Timer::after(Duration::from_secs(2)).await;
        match user_id {
            0 => Ok(self.0.name().to_string()),
            _ => Err(()),
        }
    }
}

fn app() -> impl IntoElement {
    let user = use_query(Query::new(0, GetUserName(Captured(FancyClient))));

    rect()
        .expanded()
        .center()
        .child(format!("{:?}", user.read().state()))
}

fn second_app() -> impl IntoElement {
    let on_press = |_| {
        spawn(async move {
            QueriesStorage::<GetUserName>::invalidate_all().await;
        });
    };

    rect()
        .expanded()
        .center()
        .child(Button::new().child("Invalidate").on_press(on_press))
}
