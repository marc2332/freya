use freya_core::prelude::*;
use freya_testing::TestingRunner;
use torin::{
    prelude::Area,
    size::Size,
};

#[test]
fn visible_fires_once_on_first_layout() {
    fn app() -> impl IntoElement {
        let mut count = use_consume::<State<i32>>();
        let mut area = use_consume::<State<Option<Area>>>();

        rect()
            .width(Size::px(50.))
            .height(Size::px(50.))
            .on_visible(move |e: Event<VisibleEventData>| {
                *count.write() += 1;
                area.set(Some(e.area));
            })
    }

    let (mut test, (count, area)) = TestingRunner::new(
        app,
        (100., 100.).into(),
        |runner| {
            (
                runner.provide_root_context(|| State::create(0i32)),
                runner.provide_root_context(|| State::create(Option::<Area>::None)),
            )
        },
        1.,
    );
    test.sync_and_update();
    assert_eq!(*count.peek(), 1);
    assert_eq!(
        *area.peek(),
        Some(Area::new((0., 0.).into(), (50., 50.).into()))
    );

    // Further updates must not fire it again
    test.sync_and_update();
    test.sync_and_update();
    assert_eq!(*count.peek(), 1);
}

#[test]
fn visible_fires_when_entering_clip_viewport() {
    fn app() -> impl IntoElement {
        let offset = use_consume::<State<f32>>();
        let mut count = use_consume::<State<i32>>();

        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .overflow(Overflow::Clip)
            .offset_y(*offset.read())
            .child(rect().width(Size::px(100.)).height(Size::px(200.)))
            .child(
                rect()
                    .width(Size::px(50.))
                    .height(Size::px(50.))
                    .on_visible(move |_| *count.write() += 1),
            )
    }

    let (mut test, (mut offset, count)) = TestingRunner::new(
        app,
        (100., 100.).into(),
        |runner| {
            (
                runner.provide_root_context(|| State::create(0.0f32)),
                runner.provide_root_context(|| State::create(0i32)),
            )
        },
        1.,
    );
    test.sync_and_update();

    // The target sits at 200..250, outside of the 100px tall clipped parent
    assert_eq!(*count.peek(), 0);

    // Scroll the target into view
    *offset.write() = -150.;
    test.sync_and_update();
    test.sync_and_update();
    assert_eq!(*count.peek(), 1);

    // Scrolling it out does not fire, entering again does
    *offset.write() = 0.;
    test.sync_and_update();
    test.sync_and_update();
    assert_eq!(*count.peek(), 1);
    *offset.write() = -150.;
    test.sync_and_update();
    test.sync_and_update();
    assert_eq!(*count.peek(), 2);
}

#[test]
fn visible_fires_even_when_slightly_visible() {
    fn app() -> impl IntoElement {
        let offset = use_consume::<State<f32>>();
        let mut count = use_consume::<State<i32>>();

        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .overflow(Overflow::Clip)
            .offset_y(*offset.read())
            .child(rect().width(Size::px(100.)).height(Size::px(200.)))
            .child(
                rect()
                    .width(Size::px(50.))
                    .height(Size::px(50.))
                    .on_visible(move |_| *count.write() += 1),
            )
    }

    let (mut test, (mut offset, count)) = TestingRunner::new(
        app,
        (100., 100.).into(),
        |runner| {
            (
                runner.provide_root_context(|| State::create(0.0f32)),
                runner.provide_root_context(|| State::create(0i32)),
            )
        },
        1.,
    );
    test.sync_and_update();

    // The target top edge exactly touches the viewport bottom, still not visible
    *offset.write() = -100.;
    test.sync_and_update();
    test.sync_and_update();
    assert_eq!(*count.peek(), 0);

    // One more pixel makes it partially visible
    *offset.write() = -101.;
    test.sync_and_update();
    test.sync_and_update();
    assert_eq!(*count.peek(), 1);
}

#[test]
fn visible_area_is_scaled_back_to_logical_pixels() {
    fn app() -> impl IntoElement {
        let mut area = use_consume::<State<Option<Area>>>();

        rect()
            .width(Size::px(50.))
            .height(Size::px(50.))
            .on_visible(move |e: Event<VisibleEventData>| area.set(Some(e.area)))
    }

    let (mut test, area) = TestingRunner::new(
        app,
        (200., 200.).into(),
        |runner| runner.provide_root_context(|| State::create(Option::<Area>::None)),
        2.,
    );
    test.sync_and_update();
    assert_eq!(
        *area.peek(),
        Some(Area::new((0., 0.).into(), (50., 50.).into()))
    );
}

#[test]
fn visible_fires_again_when_remounted() {
    fn app() -> impl IntoElement {
        let show = use_consume::<State<bool>>();
        let mut count = use_consume::<State<i32>>();

        rect().expanded().maybe_child((*show.read()).then(|| {
            rect()
                .width(Size::px(50.))
                .height(Size::px(50.))
                .on_visible(move |_| *count.write() += 1)
        }))
    }

    let (mut test, (mut show, count)) = TestingRunner::new(
        app,
        (100., 100.).into(),
        |runner| {
            (
                runner.provide_root_context(|| State::create(true)),
                runner.provide_root_context(|| State::create(0i32)),
            )
        },
        1.,
    );
    test.sync_and_update();
    assert_eq!(*count.peek(), 1);

    // Unmount and mount again
    *show.write() = false;
    test.sync_and_update();
    *show.write() = true;
    test.sync_and_update();
    test.sync_and_update();
    assert_eq!(*count.peek(), 2);
}
