#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::{
    helpers::from_fn_standalone,
    prelude::*,
};
use freya_performance_plugin::PerformanceOverlayPlugin;

#[cfg_attr(feature = "hotpath", hotpath::main(percentiles = [90, 95, 99]))]
fn main() {
    launch(
        LaunchConfig::new()
            .with_window(WindowConfig::new(app))
            .with_plugin(PerformanceOverlayPlugin::default()),
    )
}

fn app() -> impl IntoElement {
    let cols = 30;
    let rows = 30;

    rect().children((0..rows).map(|row| {
        rect()
            .height(Size::percent(100. / rows as f32))
            .horizontal()
            .children((0..cols).map(|col| {
                rect()
                    .key((row, col))
                    .width(Size::percent(100. / cols as f32))
                    .child(from_fn_standalone(stateful_switch))
                    .into()
            }))
            .into()
    }))
}

fn stateful_switch() -> Element {
    let mut toggled = use_state(|| false);

    Switch::new()
        .toggled(toggled())
        .on_toggle(move |_| {
            toggled.toggle();
        })
        .into()
}
