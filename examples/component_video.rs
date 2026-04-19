#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(1200., 720.)))
}

fn app() -> impl IntoElement {
    use_init_theme(dark_theme);
    let mut player = use_video(|| {
        std::env::args()
            .nth(1)
            .expect("pass a video path as the first argument")
            .into()
    });

    let state = player.state();
    let progress = player.progress();
    let position = player.position();
    let duration = player.duration();

    let toggle_label = match state {
        PlaybackState::Playing => freya::icons::lucide::pause(),
        PlaybackState::Errored => freya::icons::lucide::bug(),
        PlaybackState::Loading => freya::icons::lucide::loader(),
        _ => freya::icons::lucide::play(),
    };

    let time_label = format!(
        "{} / {}",
        format_duration(position),
        duration
            .map(format_duration)
            .unwrap_or_else(|| "--:--".into()),
    );

    rect()
        .theme_background()
        .theme_color()
        .expanded()
        .cross_align(Alignment::Center)
        .main_align(Alignment::Center)
        .spacing(8.)
        .content(Content::Flex)
        .child(
            VideoViewer::new(player)
                .width(Size::fill())
                .height(Size::flex(1.))
                .aspect_ratio(AspectRatio::Min)
                .image_cover(ImageCover::Center),
        )
        .child(
            rect()
                .horizontal()
                .content(Content::Flex)
                .cross_align(Alignment::Center)
                .spacing(12.)
                .padding(12.)
                .width(Size::percent(70.))
                .child(
                    Button::new()
                        .rounded_full()
                        .expanded()
                        .flat()
                        .on_press(move |_| player.toggle())
                        .child(
                            svg(toggle_label)
                                .color((200, 200, 200))
                                .width(Size::px(16.))
                                .height(Size::px(16.)),
                        ),
                )
                .child(
                    Slider::new(move |per: f64| {
                        if let Some(duration) = player.duration() {
                            let target =
                                Duration::from_secs_f64(duration.as_secs_f64() * per / 100.0);
                            player.seek(target);
                        }
                    })
                    .value(progress)
                    .size(Size::flex(1.)),
                )
                .child(label().text(time_label).max_lines(1)),
        )
}

fn format_duration(d: Duration) -> String {
    let total = d.as_secs();
    let minutes = total / 60;
    let seconds = total % 60;
    format!("{minutes}:{seconds:02}")
}
