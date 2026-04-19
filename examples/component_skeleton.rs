#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut loading = use_state(|| true);

    rect()
        .expanded()
        .padding(24.)
        .spacing(12.)
        .child(
            Skeleton::new(*loading.read())
                .width(Size::fill())
                .height(Size::px(20.))
                .animation(SkeletonAnimation::Shimmer)
                .child("This text appears once loaded"),
        )
        .child(
            Skeleton::new(*loading.read())
                .width(Size::px(200.))
                .height(Size::px(200.))
                .animation(SkeletonAnimation::Shimmer)
                .child(
                    rect()
                        .expanded()
                        .background((80, 120, 200))
                        .center()
                        .child("This box appears once loaded"),
                ),
        )
        .child(
            Button::new()
                .child(if *loading.read() {
                    "Mark as loaded"
                } else {
                    "Mark as loading"
                })
                .on_press(move |_| {
                    loading.toggle();
                }),
        )
}
