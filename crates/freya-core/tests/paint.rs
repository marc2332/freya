use std::rc::Rc;

use freya::prelude::*;
use freya_engine::prelude::{
    FontCollection,
    FontMgr,
    Paint,
    PaintStyle,
    ParagraphBuilder,
    ParagraphStyle,
    Recording,
    RenderRecorder,
    SkColor,
    SkImage,
    TextStyle as SkTextStyle,
    raster_n32_premul,
};
use freya_testing::prelude::*;

#[test]
pub fn basic_painting() {
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

fn paint_filled_text(fill: Fill, left_padding: f32) -> SkImage {
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
        let image = paint_filled_text(fill, 0.);
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
pub fn gradient_text_fits_aligned_text() {
    for alignment in [TextAlign::Left, TextAlign::Center, TextAlign::Right] {
        let mut test = launch_test(move || {
            rect()
                .width(Size::px(400.))
                .height(Size::px(100.))
                .background(Color::WHITE)
                .child(
                    label()
                        .width(Size::px(300.))
                        .font_size(28.)
                        .text_align(alignment)
                        .color(
                            LinearGradient::new()
                                .angle(90.)
                                .stop((Color::RED, 0.))
                                .stop((Color::BLACK, 100.)),
                        )
                        .text("Gradient text"),
                )
        });
        test.sync_and_update();

        let image = SkImage::from_encoded(test.render())
            .and_then(|image| image.make_raster_image(None, None))
            .unwrap();
        let pixels = image.peek_pixels().unwrap();
        let text_pixels: Vec<_> = (0..pixels.width())
            .flat_map(|x| (0..pixels.height()).map(move |y| (x, y)))
            .filter_map(|(x, y)| {
                let color = pixels.get_color((x, y));
                (color.g() < 100).then_some((x, color.r()))
            })
            .collect();

        let left = text_pixels.iter().map(|(x, _)| *x).min().unwrap();
        let right = text_pixels.iter().map(|(x, _)| *x).max().unwrap();

        let start_red = text_pixels
            .iter()
            .filter(|(x, _)| *x < left + 10)
            .map(|(_, red)| *red)
            .min()
            .unwrap();
        let end_red = text_pixels
            .iter()
            .filter(|(x, _)| *x > right - 10)
            .map(|(_, red)| *red)
            .max()
            .unwrap();

        assert!(
            start_red < 50,
            "gradient starts too late for {alignment:?}: {start_red} ({left}..{right})"
        );
        assert!(
            end_red > 180,
            "gradient ends too early for {alignment:?}: {end_red} ({left}..{right})"
        );
    }
}

#[test]
pub fn gradient_text_follows_the_paragraph() {
    let offset = 50;
    let fill: Fill = LinearGradient::new()
        .angle(90.)
        .stop((Color::BLACK, 0.))
        .stop((Color::WHITE, 100.))
        .into();

    let origin_image = paint_filled_text(fill.clone(), 0.);
    let moved_image = paint_filled_text(fill, offset as f32);
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
        .expect("no text pixels painted");

    assert!(
        biggest_difference <= 4,
        "the gradient did not follow the text, off by {biggest_difference} levels"
    );
}

#[test]
pub fn overlapping_same_layer_siblings_paint_deterministically() {
    fn app() -> impl IntoElement {
        rect()
            .child(
                rect()
                    .position(Position::new_absolute().top(10.).left(10.))
                    .width(Size::px(100.))
                    .height(Size::px(100.))
                    .background((255, 0, 0)),
            )
            .child(
                rect()
                    .position(Position::new_absolute().top(50.).left(50.))
                    .width(Size::px(100.))
                    .height(Size::px(100.))
                    .background((0, 0, 255)),
            )
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    let first_render = test.render_pixels();
    let second_render = test.render_pixels();
    assert_eq!(first_render, second_render);

    let pixel = |pixels: &[u8], x: usize, y: usize| -> [u8; 4] {
        let offset = (y * 500 + x) * 4;
        pixels[offset..offset + 4].try_into().unwrap()
    };

    let red_only = pixel(&first_render, 20, 20);
    let blue_only = pixel(&first_render, 140, 140);
    let overlap = pixel(&first_render, 75, 75);

    assert_ne!(red_only, blue_only);
    assert_eq!(overlap, blue_only);
}

#[cfg(feature = "incremental-painting")]
#[test]
pub fn recorded_bounds_cover_painted_pixels() {
    fn app() -> impl IntoElement {
        rect()
            .padding(20.)
            .spacing(20.)
            .child(
                rect()
                    .width(Size::px(100.))
                    .height(Size::px(50.))
                    .corner_radius(8.)
                    .background((30, 120, 200))
                    .shadow(Shadow::new().x(10.).y(10.).blur(15.).color((0, 0, 0)))
                    .border(Border::new().fill((200, 30, 30)).width(2.)),
            )
            .child(
                rect()
                    .width(Size::px(80.))
                    .height(Size::px(40.))
                    .rotate(30.)
                    .background((30, 200, 120)),
            )
            .child(
                rect()
                    .width(Size::px(80.))
                    .height(Size::px(40.))
                    .opacity(0.5)
                    .background((200, 200, 30)),
            )
            .child(label().text("Some styled text"))
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    test.render_to_surface();

    let tree = test.tree().borrow();
    assert!(!tree.render_state.cache.is_empty());
    let mut fonts = FontCollection::new();
    let painted: usize = tree
        .render_state
        .cache
        .values()
        .map(|recording| assert_recording_bounds(recording, &mut fonts))
        .sum();
    assert!(painted > 0);
}

fn assert_recording_bounds(recording: &Recording, fonts: &mut FontCollection) -> usize {
    let bounds = recording
        .bounds
        .expect("Recording should have finite bounds");
    let mut surface = raster_n32_premul((500, 500)).unwrap();
    surface.canvas().clear(SkColor::WHITE);
    recording.replay(surface.canvas(), fonts);
    let image = surface.image_snapshot();
    let pixels = image.peek_pixels().unwrap();
    let mut painted = 0;

    for vertical in 0..pixels.height() {
        for horizontal in 0..pixels.width() {
            if pixels.get_color((horizontal, vertical)) != SkColor::WHITE {
                painted += 1;
                let point = (horizontal as f32 + 0.5, vertical as f32 + 0.5).into();
                assert!(
                    bounds.contains(point),
                    "Painted pixel at ({horizontal}, {vertical}) is outside recording bounds {bounds:?}"
                );
            }
        }
    }
    painted
}

#[test]
fn text_recording_bounds_cover_native_shadows_and_foreground_strokes() {
    let mut fonts = FontCollection::new();
    fonts.set_default_font_manager(FontMgr::default(), None);
    let mut style = SkTextStyle::new();
    style.set_font_size(32.);
    style.add_shadow(TextShadow::new(Color::BLACK, (120., 60.), 10.).into());
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_color(SkColor::BLACK);
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_width(20.);
    style.set_foreground_paint(&paint);
    let mut builder = ParagraphBuilder::new(&ParagraphStyle::default(), fonts.clone());
    builder.push_style(&style);
    builder.add_text("Shadow");
    let mut paragraph = builder.build();
    paragraph.layout(200.);
    let mut recorder = RenderRecorder::default();
    recorder.draw_paragraph(
        Rc::new(paragraph),
        (50., 50.).into(),
        [],
        TextAlign::default().into(),
    );

    assert!(assert_recording_bounds(&recorder.finish(), &mut fonts) > 0);
}
