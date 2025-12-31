#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .center()
        .horizontal()
        .spacing(16.)
        .child(
            rect()
                .height(Size::px(220.))
                .width(Size::px(220.))
                .background_linear_gradient(
                    LinearGradient::new()
                        .angle(250.)
                        .stop(((255, 100, 50), 15.))
                        .stop(((255, 0, 0), 50.))
                        .stop(((255, 192, 203), 80.)),
                ),
        )
        .child(
            rect()
                .height(Size::px(220.))
                .width(Size::px(220.))
                .background_radial_gradient(
                    RadialGradient::new()
                        .stop(((255, 100, 50), 15.))
                        .stop(((255, 0, 0), 50.))
                        .stop(((255, 192, 203), 80.)),
                ),
        )
        .child(
            rect()
                .height(Size::px(220.))
                .width(Size::px(220.))
                .background_conic_gradient(
                    ConicGradient::new()
                        .angle(250.)
                        .stop(((255, 100, 50), 15.))
                        .stop(((255, 0, 0), 50.))
                        .stop(((255, 192, 203), 80.)),
                ),
        )
}
