use std::time::Duration;

use freya::prelude::*;
use freya_animation::prelude::*;
use freya_testing::{
    TestingNode,
    prelude::*,
};

fn animated_rect(test: &TestingRunner, name: &str) -> TestingNode {
    test.find(|node, element| {
        Rect::try_downcast(element)
            .filter(|rect| rect.accessibility.builder.label() == Some(name))
            .map(|_| node)
    })
    .unwrap()
}

#[test]
pub fn track_progress() {
    fn use_animation_app() -> impl IntoElement {
        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);

            AnimNum::new(0., 100.).time(50)
        });

        let progress = animation.get().value();

        rect().a11y_alt("animation").width(Size::px(progress))
    }

    let mut test = launch_test(use_animation_app);

    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(15));
    let rect = animated_rect(&test, "animation").layout();
    assert!(rect.area.width() > 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(85));
    let rect = animated_rect(&test, "animation").layout();
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
            .a11y_alt("animation")
            .on_press(move |_| animation.reverse())
            .width(Size::px(progress))
            .height(Size::fill())
            .background(Color::WHITE)
    }

    let mut test = launch_test(use_animation_app);

    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(100));
    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 100.0);

    test.click_cursor((5., 5.));
    test.sync_and_update();

    test.poll(Duration::from_millis(1), Duration::from_millis(50));
    let rect = animated_rect(&test, "animation").layout();
    assert!(rect.area.width() < 100.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(50));
    let rect = animated_rect(&test, "animation").layout();
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
            .a11y_alt("animation")
            .width(Size::fill())
            .height(Size::fill())
            .background(background)
    }

    let mut test = launch_test(use_animation_app);

    let background = |test: &TestingRunner| {
        Rect::try_downcast(&*animated_rect(test, "animation").element())
            .unwrap()
            .style
            .background
    };
    assert_eq!(background(&test), Fill::Color(Color::RED));

    test.poll(Duration::from_millis(1), Duration::from_millis(100));
    assert_eq!(background(&test), Fill::Color((50, 100, 200).into()));
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
            .a11y_alt("animation")
            .background(Color::WHITE)
            .height(Size::fill())
            .width(Size::px(progress_a))
            .child(
                rect()
                    .a11y_alt("second animation")
                    .background(Color::WHITE)
                    .height(Size::fill())
                    .width(Size::px(progress_b)),
            )
    }

    let mut test = launch_test(use_animation_app);

    let rect_a = animated_rect(&test, "animation").layout();
    let rect_b = animated_rect(&test, "second animation").layout();
    assert_eq!(rect_a.area.width(), 0.0);
    assert_eq!(rect_b.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(15));
    let rect_a = animated_rect(&test, "animation").layout();
    let rect_b = animated_rect(&test, "second animation").layout();
    assert!(rect_a.area.width() > 0.0);
    assert_eq!(rect_b.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(85));
    let rect_a = animated_rect(&test, "animation").layout();
    let rect_b = animated_rect(&test, "second animation").layout();
    assert_eq!(rect_a.area.width(), 100.0);
    assert!(rect_b.area.width() > 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(15));
    let rect_a = animated_rect(&test, "animation").layout();
    let rect_b = animated_rect(&test, "second animation").layout();
    assert_eq!(rect_a.area.width(), 100.0);
    assert!(rect_b.area.width() > 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(85));
    let rect_a = animated_rect(&test, "animation").layout();
    let rect_b = animated_rect(&test, "second animation").layout();
    assert_eq!(rect_a.area.width(), 100.0);
    assert_eq!(rect_b.area.width(), 100.0);
}

#[test]
pub fn animation_with_dependencies() {
    fn use_animation_app() -> impl IntoElement {
        let mut target = use_state(|| 100.0);

        let animation = use_animation_with_dependencies(&target(), |conf, target| {
            conf.on_creation(OnCreation::Run);

            AnimNum::new(0., *target).time(50)
        });

        let progress = animation.get().value();

        rect()
            .a11y_alt("animation")
            .width(Size::px(progress))
            .height(Size::fill())
            .background(Color::WHITE)
            .child(
                Button::new()
                    .on_press(move |_| target.set(200.0))
                    .child("Change Target"),
            )
    }

    let mut test = launch_test(use_animation_app);

    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(100));
    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 100.0);

    // Click button to change target to 200
    test.click_cursor((15.0, 15.0));
    test.sync_and_update();
    test.sync_and_update();
    test.sync_and_update();
    test.sync_and_update();
    test.sync_and_update();

    // Animation should reset to 0 when dependencies change (default OnChange::Reset)
    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 0.0);
}

#[test]
pub fn animation_with_dependencies_on_change_finish() {
    fn use_animation_app() -> impl IntoElement {
        let mut target = use_state(|| 100.0);

        let animation = use_animation_with_dependencies(&target(), |conf, target| {
            conf.on_creation(OnCreation::Run);
            conf.on_change(OnChange::Finish);

            AnimNum::new(0., *target).time(50)
        });

        let progress = animation.get().value();

        rect()
            .a11y_alt("animation")
            .width(Size::px(progress))
            .height(Size::fill())
            .background(Color::WHITE)
            .child(
                Button::new()
                    .on_press(move |_| target.set(200.0))
                    .child("Change Target"),
            )
    }

    let mut test = launch_test(use_animation_app);

    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(25));
    let rect = animated_rect(&test, "animation").layout();
    let width_mid = rect.area.width();
    assert!(width_mid > 0.0 && width_mid < 100.0);

    // Click button to change target to 200
    // With OnChange::Finish, should immediately jump to finish value
    test.click_cursor((7.0, 7.0));
    test.sync_and_update();
    test.sync_and_update();
    test.sync_and_update();
    test.sync_and_update();
    test.sync_and_update();

    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 200.0);
}

#[test]
pub fn animation_with_dependencies_on_change_rerun() {
    fn use_animation_app() -> impl IntoElement {
        let mut target = use_state(|| 100.0);

        let animation = use_animation_with_dependencies(&target(), |conf, target| {
            conf.on_creation(OnCreation::Run);
            conf.on_change(OnChange::Rerun);

            AnimNum::new(0., *target).time(50)
        });

        let progress = animation.get().value();

        rect()
            .a11y_alt("animation")
            .width(Size::px(progress))
            .height(Size::fill())
            .background(Color::WHITE)
            .child(
                Button::new()
                    .on_press(move |_| target.set(200.0))
                    .child("Change Target"),
            )
    }

    let mut test = launch_test(use_animation_app);

    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(100));
    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 100.0);

    // Click button to change target to 200
    // With OnChange::Rerun, should restart from 0 and animate to 200
    test.click_cursor((15.0, 15.0));
    test.sync_and_update();
    test.sync_and_update();
    test.sync_and_update();
    test.sync_and_update();
    test.sync_and_update();

    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(25));
    let rect = animated_rect(&test, "animation").layout();
    assert!(rect.area.width() > 0.0 && rect.area.width() < 200.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(75));
    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 200.0);
}

#[test]
pub fn animation_with_dependencies_reruns_in_same_render() {
    fn use_animation_app() -> impl IntoElement {
        let mut target = use_state(|| 100.0);

        let animation = use_animation_with_dependencies(&target(), |conf, target| {
            conf.on_creation(OnCreation::Finish);
            conf.on_change(OnChange::Rerun);

            AnimNum::new(0., *target).time(50)
        });

        let progress = animation.get().value();

        rect()
            .a11y_alt("animation")
            .width(Size::px(progress))
            .height(Size::fill())
            .background(Color::WHITE)
            .child(
                Button::new()
                    .on_press(move |_| target.set(200.0))
                    .child("Change Target"),
            )
    }

    let mut test = launch_test(use_animation_app);

    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 100.0);

    // The rerun must be reflected in the first render after the change
    test.click_cursor((15.0, 15.0));
    test.sync_and_update();

    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 0.0);

    test.poll(Duration::from_millis(1), Duration::from_millis(100));
    let rect = animated_rect(&test, "animation").layout();
    assert_eq!(rect.area.width(), 200.0);
}
