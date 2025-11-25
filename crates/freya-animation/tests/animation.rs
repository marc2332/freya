use std::time::Duration;

use freya::prelude::*;
use freya_animation::prelude::*;
use freya_testing::*;

#[test]
pub fn track_progress() {
    fn use_animation_app() -> impl IntoElement {
        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);

            AnimNum::new(0., 100.).time(50)
        });

        let progress = animation.get().value();

        rect().width(Size::px(progress))
    }

    let mut test = launch_test(use_animation_app);

    let rect = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    assert_eq!(rect.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(15));
    let rect = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    assert!(rect.area.width() > 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(85));
    let rect = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    assert_eq!(rect.area.width(), 100.0);
}

#[test]
pub fn reverse_progress() {
    fn use_animation_app() -> impl IntoElement {
        let mut animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);

            AnimNum::new(0., 100.).time(50)
        });

        let progress = animation.get().value();

        rect()
            .on_press(move |_| animation.reverse())
            .width(Size::px(progress))
            .height(Size::fill())
            .background(Color::WHITE)
    }

    let mut test = launch_test(use_animation_app);

    let rect = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    assert_eq!(rect.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(100));
    let rect = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    assert_eq!(rect.area.width(), 100.0);

    test.click_cursor((5., 5.));
    test.sync_and_update();

    test.poll(Duration::from_millis(1), Duration::from_millis(50));
    let rect = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    assert!(rect.area.width() < 100.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(50));
    let rect = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    assert_eq!(rect.area.width(), 0.0);
}

#[test]
pub fn animate_color() {
    fn use_animation_app() -> impl IntoElement {
        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);

            AnimColor::new(Color::RED, (50, 100, 200)).time(50)
        });

        let background = animation.get().value();

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .background(background)
    }

    let mut test = launch_test(use_animation_app);

    let background = &test.find_many(|_, e| Rect::try_downcast(e).map(|e| e.style.background))[2];
    assert_eq!(background, &Fill::Color(Color::RED));

    test.poll(Duration::from_millis(1), Duration::from_millis(100));
    let background = &test.find_many(|_, e| Rect::try_downcast(e).map(|e| e.style.background))[2];
    assert_eq!(background, &Fill::Color((50, 100, 200).into()));
}

#[test]
pub fn sequential() {
    fn use_animation_app() -> impl IntoElement {
        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);

            AnimSequential::new([
                AnimNum::new(0., 100.).time(50),
                AnimNum::new(0., 100.).time(50),
            ])
        });

        let progress_a = animation.get()[0].value();
        let progress_b = animation.get()[1].value();

        rect()
            .background(Color::WHITE)
            .height(Size::fill())
            .width(Size::px(progress_a))
            .child(
                rect()
                    .background(Color::WHITE)
                    .height(Size::fill())
                    .width(Size::px(progress_b)),
            )
    }

    let mut test = launch_test(use_animation_app);

    let rect_a = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    let rect_b = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[3];
    assert_eq!(rect_a.area.width(), 0.0);
    assert_eq!(rect_b.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(15));
    let rect_a = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    let rect_b = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[3];
    assert!(rect_a.area.width() > 0.0);
    assert_eq!(rect_b.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(85));
    let rect_a = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    let rect_b = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[3];
    assert_eq!(rect_a.area.width(), 100.0);
    assert!(rect_b.area.width() > 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(15));
    let rect_a = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    let rect_b = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[3];
    assert_eq!(rect_a.area.width(), 100.0);
    assert!(rect_b.area.width() > 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(85));
    let rect_a = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[2];
    let rect_b = &test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()))[3];
    assert_eq!(rect_a.area.width(), 100.0);
    assert_eq!(rect_b.area.width(), 100.0);
}
