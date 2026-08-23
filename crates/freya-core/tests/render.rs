use freya::prelude::*;
use freya_engine::prelude::{
    SkColor,
    SkImage,
};
use freya_testing::prelude::*;

#[test]
pub fn basic_render() {
    fn app() -> impl IntoElement {
        let mut show_popup = use_state(|| true);

        rect()
            .child(
                Popup::new()
                    .on_close_request(move |_| show_popup.set(false))
                    .maybe(show_popup(), |popup| {
                        popup
                            .child(PopupTitle::new("Title".to_string()))
                            .child(PopupContent::new().child("Hello, World!"))
                    }),
            )
            .child(
                Button::new()
                    .child("Open")
                    .on_press(move |_| show_popup.toggle()),
            )
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    let data = test.render();

    assert!(!data.is_empty());
}

fn render_filled_text(fill: Fill, left_padding: f32) -> SkImage {
    let mut test = launch_test(move || {
        rect()
            .width(Size::px(400.))
            .height(Size::px(200.))
            .background(Color::WHITE)
            .padding((0., 0., 0., left_padding))
            .child(
                label()
                    .width(Size::px(300.))
                    .text_align(TextAlign::Center)
                    .font_size(28.)
                    .color(fill.clone())
                    .text("A somewhat long line that wraps around"),
            )
    });
    test.sync_and_update();

    SkImage::from_encoded(test.render())
        .and_then(|image| image.make_raster_image(None, None))
        .unwrap()
}

#[test]
pub fn gradient_text_is_not_clipped() {
    fn dark_column_bounds(fill: Fill) -> (i32, i32) {
        let image = render_filled_text(fill, 0.);
        let pixels = image.peek_pixels().unwrap();
        let dark: Vec<i32> = (0..pixels.width())
            .filter(|&x| (0..pixels.height()).any(|y| pixels.get_color((x, y)).r() < 128))
            .collect();

        (dark[0], dark[dark.len() - 1])
    }

    let solid = dark_column_bounds(Color::BLACK.into());
    let gradient = dark_column_bounds(
        LinearGradient::new()
            .stop((Color::BLACK, 0.))
            .stop((Color::BLACK, 100.))
            .into(),
    );

    assert_eq!(
        solid, gradient,
        "gradient text does not cover the same columns as solid text"
    );
}

#[test]
pub fn gradient_text_follows_the_paragraph() {
    let offset = 50;
    let fill: Fill = LinearGradient::new()
        .angle(90.)
        .stop((Color::BLACK, 0.))
        .stop((Color::WHITE, 100.))
        .into();

    let origin_image = render_filled_text(fill.clone(), 0.);
    let moved_image = render_filled_text(fill, offset as f32);
    let origin = origin_image.peek_pixels().unwrap();
    let moved = moved_image.peek_pixels().unwrap();

    let biggest_difference = (0..origin.height())
        .flat_map(|y| (0..origin.width() - offset).map(move |x| (x, y)))
        .filter_map(|(x, y)| {
            let color = origin.get_color((x, y));
            if color == SkColor::WHITE {
                return None;
            }

            let shifted = moved.get_color((x + offset, y));
            Some(
                color
                    .r()
                    .abs_diff(shifted.r())
                    .max(color.g().abs_diff(shifted.g()))
                    .max(color.b().abs_diff(shifted.b())),
            )
        })
        .max()
        .expect("no text pixels rendered");

    assert!(
        biggest_difference <= 4,
        "the gradient did not follow the text, off by {biggest_difference} levels"
    );
}
