#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::time::Duration;

use freya::prelude::*;
use freya_sdk::timeout::use_timeout;
use tracing_subscriber::{
    EnvFilter,
    FmtSubscriber,
};

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new("freya_winit=trace"))
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(600., 600.)))
}

fn app() -> impl IntoElement {
    let mut count = use_state(|| 25usize);
    let mut increasing = use_state(|| true);
    let mut timeout = use_timeout(|| Duration::from_millis(10));

    if timeout.elapsed() {
        timeout.reset();
        if *increasing.read() {
            let next = (*count.read() + 25).min(500);
            count.set(next);
            if next >= 500 {
                increasing.set(false);
            }
        } else {
            let next = (*count.read()).saturating_sub(25).max(25);
            count.set(next);
            if next <= 25 {
                increasing.set(true);
            }
        }
    }

    let items = (0..*count.read()).map(|i| {
        rect()
            .width(Size::px(120.))
            .height(Size::px(28.))
            .margin((4., 4.))
            .center()
            .child(label().text(format!("Item {}", i + 1)))
            .into()
    });

    rect().padding(8.).vertical().child(
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .children_iter(items),
    )
}
