use std::{
    fs,
    io,
    path::{
        Path,
        PathBuf,
    },
    time::Duration,
};

use freya_components::button::Button;
use freya_core::elements::{
    image::{
        Image,
        ImageHolder,
        image,
    },
    label::Label,
};
use freya_testing::prelude::*;
use freya_video::{
    ensure_ffmpeg,
    use_video,
};
use torin::prelude::Size;

const SAMPLE_URL: &str = "https://media.w3.org/2010/05/sintel/trailer.mp4";

fn sample_video_path() -> PathBuf {
    std::env::temp_dir().join("freya-video-test-sintel-trailer.mp4")
}

fn download_sample(path: &Path) {
    if fs::metadata(path).is_ok_and(|meta| meta.len() > 0) {
        return;
    }
    let mut response = ureq::get(SAMPLE_URL)
        .call()
        .expect("failed to download the sample video");
    let mut file = fs::File::create(path).expect("failed to create the temp video file");
    io::copy(&mut response.body_mut().as_reader(), &mut file)
        .expect("failed to write the temp video file");
}

fn app() -> impl IntoElement {
    let mut player = use_video(sample_video_path);
    rect()
        .child(
            rect()
                .width(Size::fill())
                .height(Size::px(200.))
                .maybe_child(player.frame().map(image)),
        )
        .child(
            Button::new()
                .on_press(move |_| player.pause())
                .child("pause"),
        )
        .child(Button::new().on_press(move |_| player.play()).child("play"))
        .child(
            Button::new()
                .on_press(move |_| player.seek(Duration::from_secs(2), Duration::ZERO))
                .child("seek"),
        )
        .child(Button::new().on_press(move |_| player.stop()).child("stop"))
}

fn current_frame(test: &mut TestingRunner) -> Option<ImageHolder> {
    test.poll(Duration::from_millis(16), Duration::from_millis(250));
    test.sync_and_update();
    test.find(|_, element| Image::try_downcast(element).map(|image| image.image_holder))
}

fn wait_for_new_frame(test: &mut TestingRunner, previous: &ImageHolder) -> bool {
    (0..100).any(|_| current_frame(test).is_some_and(|frame| &frame != previous))
}

fn press_text(test: &mut TestingRunner, text: &str) {
    let area = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|label| label.text.as_ref() == text)
                .map(|_| node.layout().area)
        })
        .unwrap();
    let center = (
        (area.min_x() + area.width() / 2.0) as f64,
        (area.min_y() + area.height() / 2.0) as f64,
    );
    test.click_cursor(center);
    test.sync_and_update();
}

#[test]
fn playback_controls() {
    ensure_ffmpeg().expect("failed to prepare ffmpeg");
    download_sample(&sample_video_path());

    let mut test = launch_test(app);

    let first = (0..100)
        .find_map(|_| current_frame(&mut test))
        .expect("the first frame should decode and render");
    assert!(
        wait_for_new_frame(&mut test, &first),
        "playback should advance to a new frame",
    );

    press_text(&mut test, "pause");
    for _ in 0..4 {
        current_frame(&mut test);
    }
    let paused = current_frame(&mut test).expect("a frame should stay visible while paused");
    assert!(
        (0..6).all(|_| current_frame(&mut test).as_ref() == Some(&paused)),
        "the frame should not advance while paused",
    );

    press_text(&mut test, "play");
    assert!(
        wait_for_new_frame(&mut test, &paused),
        "playback should advance again after resuming",
    );

    let before_seek = current_frame(&mut test).expect("a frame before seeking");
    press_text(&mut test, "seek");
    assert!(
        wait_for_new_frame(&mut test, &before_seek),
        "seeking should resume rendering frames",
    );

    press_text(&mut test, "stop");
    assert!(
        (0..40).any(|_| current_frame(&mut test).is_none()),
        "stopping should clear the displayed frame",
    );
}
