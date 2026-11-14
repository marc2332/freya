use freya::prelude::*;
use freya_engine::prelude::{
    Paint,
    SkColor,
    SkImage,
    SkRect,
};
use freya_testing::prelude::*;

fn snapshot(runner: &mut TestingRunner) -> SkImage {
    runner.sync_and_update();
    SkImage::from_encoded(runner.render())
        .and_then(|image| image.make_raster_image(None, None))
        .unwrap()
}

fn assert_channel(actual: u8, expected: u8) {
    assert!(
        actual.abs_diff(expected) <= 3,
        "expected {expected}, got {actual}"
    );
}

fn horizontal_mask() -> LinearGradient {
    LinearGradient::new()
        .angle(270.)
        .stop((Color::TRANSPARENT, 0.))
        .stop((Color::BLACK, 100.))
}

#[test]
fn mask_uses_alpha_not_rgb_and_preserves_backdrop() {
    let mut runner = launch_test(|| {
        rect()
            .background(Color::BLUE)
            .width(Size::px(200.))
            .height(Size::px(100.))
            .child(
                rect()
                    .width(Size::px(100.))
                    .height(Size::px(100.))
                    .background(Color::RED)
                    .mask(Color::from_argb(128, 0, 255, 0)),
            )
    });
    let image = snapshot(&mut runner);
    let pixels = image.peek_pixels().unwrap();
    let color = pixels.get_color((50, 50));
    assert_channel(color.r(), 128);
    assert_channel(color.g(), 0);
    assert_channel(color.b(), 127);
    assert_eq!(pixels.get_color((150, 50)), SkColor::BLUE);
}

#[test]
fn inherited_gradient_uses_owner_bounds() {
    let mut runner = launch_test(|| {
        rect().padding((0., 0., 0., 40.)).child(
            rect()
                .horizontal()
                .width(Size::px(100.))
                .height(Size::px(100.))
                .mask(horizontal_mask())
                .children((0..2).map(|_| {
                    rect()
                        .width(Size::px(50.))
                        .height(Size::px(100.))
                        .background(Color::BLACK)
                })),
        )
    });
    let image = snapshot(&mut runner);
    let pixels = image.peek_pixels().unwrap();
    assert_channel(pixels.get_color((65, 50)).r(), 190);
    assert_channel(pixels.get_color((115, 50)).r(), 62);
}

#[test]
fn nested_masks_multiply_with_opacity() {
    let mut runner = launch_test(|| {
        rect().mask(Color::from_argb(128, 255, 0, 0)).child(
            rect()
                .width(Size::px(100.))
                .height(Size::px(100.))
                .background(Color::BLACK)
                .opacity(0.5)
                .mask(Color::from_argb(128, 0, 255, 0)),
        )
    });
    let image = snapshot(&mut runner);
    assert_channel(image.peek_pixels().unwrap().get_color((50, 50)).r(), 223);
}

#[test]
fn bounded_mask_hides_overflowing_content() {
    let mut runner = launch_test(|| {
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .mask(Color::BLACK)
            .child(
                rect()
                    .width(Size::px(200.))
                    .height(Size::px(100.))
                    .background(Color::BLACK),
            )
    });
    let image = snapshot(&mut runner);
    let pixels = image.peek_pixels().unwrap();
    assert_eq!(pixels.get_color((50, 50)), SkColor::BLACK);
    assert_eq!(pixels.get_color((150, 50)), SkColor::WHITE);
}

#[test]
fn callback_mask_clears_unpainted_content() {
    let mut runner = launch_test(|| {
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .background(Color::BLACK)
            .mask(RenderCallback::new(|context| {
                let RenderTarget::Background { canvas, .. } = &context.target else {
                    return;
                };

                let mut paint = Paint::default();
                paint.set_color(SkColor::WHITE);
                canvas.draw_rect(
                    SkRect::from_xywh(
                        context.origin.x,
                        context.origin.y,
                        context.size.width / 2.,
                        context.size.height,
                    ),
                    &paint,
                );
            }))
    });
    let image = snapshot(&mut runner);
    let pixels = image.peek_pixels().unwrap();
    assert_eq!(pixels.get_color((25, 50)), SkColor::BLACK);
    assert_eq!(pixels.get_color((75, 50)), SkColor::WHITE);
}

#[test]
fn omitting_mask_updates_descendants() {
    let (mut runner, mut masked) = TestingRunner::new(
        || {
            let masked = use_consume::<State<bool>>();
            rect()
                .maybe(masked(), |el| el.mask(Color::TRANSPARENT))
                .child(rect().expanded().background(Color::BLACK))
        },
        (100., 100.).into(),
        |root| root.provide_root_context(|| State::create(true)),
        1.,
    );
    let hidden = snapshot(&mut runner);
    assert_eq!(
        hidden.peek_pixels().unwrap().get_color((50, 50)),
        SkColor::WHITE
    );
    masked.set(false);
    let visible = snapshot(&mut runner);
    assert_eq!(
        visible.peek_pixels().unwrap().get_color((50, 50)),
        SkColor::BLACK
    );
}

#[test]
fn mask_tracks_owner_scale_and_pixel_density() {
    for scale_factor in [1., 2.] {
        let (mut runner, ()) = TestingRunner::new(
            || {
                rect().padding(50.).child(
                    rect()
                        .width(Size::px(100.))
                        .height(Size::px(100.))
                        .scale(0.5)
                        .mask(horizontal_mask())
                        .child(rect().expanded().background(Color::BLACK)),
                )
            },
            (400., 400.).into(),
            |_| (),
            scale_factor,
        );
        let image = snapshot(&mut runner);
        let pixels = image.peek_pixels().unwrap();
        let left = (87. * scale_factor) as i32;
        let right = (112. * scale_factor) as i32;
        let middle = (100. * scale_factor) as i32;
        assert_channel(pixels.get_color((left, middle)).r(), 191);
        assert_channel(pixels.get_color((right, middle)).r(), 64);
    }
}

#[test]
fn mask_does_not_follow_descendant_transform() {
    let mut runner = launch_test(|| {
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .mask(horizontal_mask())
            .child(rect().expanded().scale(0.5).background(Color::BLACK))
    });
    let image = snapshot(&mut runner);
    let pixels = image.peek_pixels().unwrap();
    assert_channel(pixels.get_color((30, 50)).r(), 177);
    assert_channel(pixels.get_color((70, 50)).r(), 75);
}
