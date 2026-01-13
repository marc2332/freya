#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    animation::*,
    prelude::*,
};
use freya_performance_plugin::PerformanceOverlayPlugin;

fn main() {
    launch(
        LaunchConfig::new()
            .with_plugin(PerformanceOverlayPlugin::default())
            .with_window(WindowConfig::new(app)),
    )
}

fn app() -> impl IntoElement {
    let animation = use_animation(|conf| {
        conf.on_creation(OnCreation::Run);
        conf.on_finish(OnFinish::reverse());
        AnimNum::new(0., 650.)
            .time(400)
            .ease(Ease::InOut)
            .function(Function::Sine)
    });

    let progress = animation.get().value();

    rect().expanded().center().children((0..32).map(|i| {
        rect()
            .key(i)
            .offset_x(progress - i as f32 * 10.)
            .horizontal()
            .child(
                rect()
                    .width(Size::px(45.))
                    .height(Size::px(25.))
                    .background((7, 102, 173))
                    .corner_radius(100.),
            )
            .child(
                rect()
                    .width(Size::px(45.))
                    .height(Size::px(25.))
                    .background((166, 207, 152))
                    .corner_radius(100.),
            )
            .child(
                rect()
                    .width(Size::px(45.))
                    .height(Size::px(25.))
                    .background((179, 19, 18))
                    .corner_radius(100.),
            )
            .child(
                rect()
                    .width(Size::px(45.))
                    .height(Size::px(25.))
                    .background((255, 108, 34))
                    .corner_radius(100.),
            )
            .into()
    }))
}
