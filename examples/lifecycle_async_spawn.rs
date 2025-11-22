use std::time::Duration;

use async_io::Timer;
use freya::prelude::*;
use futures_lite::stream::StreamExt;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    use_hook(|| {
        spawn(async move {
            let mut interval = Timer::interval(Duration::from_secs(1));
            loop {
                interval.next().await;
                println!("Elapsed 1s");
            }
        });
    });

    let on_press = |_| {
        spawn(async move {
            let mut interval = Timer::interval(Duration::from_secs(2));
            interval.next().await;
            println!("Elapsed 2s");
        });
    };

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .main_align(Alignment::center())
        .cross_align(Alignment::center())
        .child(Button::new().on_press(on_press).child("Spawn"))
}
