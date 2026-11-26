#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    animation::*,
    prelude::*,
};
use freya_metrics_plugin::{
    MetricsContext,
    MetricsPlugin,
};
use tracing_subscriber::{
    filter::EnvFilter,
    fmt,
};

fn main() {
    fmt()
        .with_env_filter(EnvFilter::new("freya::metrics=debug"))
        .init();

    launch(
        LaunchConfig::new()
            .with_plugin(MetricsPlugin::default().with_visible_performance(true))
            .with_window(WindowConfig::new(app).with_size(1191., 911.)),
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

    rect()
        .expanded()
        .center()
        .map(MetricsContext::try_get(), |el, metrics| {
            let performance = metrics.clone();
            el.child(
                rect()
                    .horizontal()
                    .spacing(8.)
                    .child(
                        Button::new()
                            .on_press(move |_| {
                                performance
                                    .set_performance_visible(!performance.is_performance_visible());
                            })
                            .child("Toggle performance"),
                    )
                    .child(
                        Button::new()
                            .on_press(move |_| {
                                metrics.set_stats_visible(!metrics.is_stats_visible());
                            })
                            .child("Toggle stats"),
                    ),
            )
        })
        .children((0..32).map(|i| {
            rect()
                .key(i)
                .offset_x(progress - i as f32 * 10.)
                .horizontal()
                .children(
                    [
                        (7, 102, 173),
                        (166, 207, 152),
                        (179, 19, 18),
                        (255, 108, 34),
                    ]
                    .into_iter()
                    .map(|color| {
                        rect()
                            .width(Size::px(45.))
                            .height(Size::px(25.))
                            .background(color)
                            .corner_radius(100.)
                    }),
                )
        }))
}
