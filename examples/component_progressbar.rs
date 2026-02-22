#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use rand::Rng;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut progress = use_state(|| 50.);

    rect()
        .on_press(move |_| progress.set(random_percent().round()))
        .center()
        .expanded()
        .child(ProgressBar::new(progress()).width(Size::px(200.)))
}

fn random_percent() -> f32 {
    let mut rng = rand::rng();
    rng.random_range(0.0..=100.0)
}
