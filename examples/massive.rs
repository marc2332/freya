use freya::{
    helpers::from_fn,
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

fn app() -> Element {
    let cols = 30;
    let rows = 30;

    rect()
        .children_iter((0..rows).map(|row| {
            rect()
                .height(Size::percent(100. / rows as f32))
                .horizontal()
                .children_iter((0..cols).map(|col| {
                    rect()
                        .width(Size::percent(100. / cols as f32))
                        .children([from_fn((row, col), (), stateful_switch)])
                        .into()
                }))
                .into()
        }))
        .into()
}

fn stateful_switch(_: &()) -> Element {
    let mut toggled = use_state(|| false);

    Switch::new()
        .toggled(toggled())
        .on_toggle(move |_| {
            toggled.toggle();
        })
        .into()
}
