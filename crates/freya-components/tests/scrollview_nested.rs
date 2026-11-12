use freya::prelude::*;
use freya_testing::prelude::*;

/// The scroll positions of a page and of a scroll view inside it.
#[derive(Clone, Copy)]
struct Nested {
    page: ScrollController,
    inner: ScrollController,
}

/// Where the nested scroll view is in the page, and how tall each row of the page is.
const INNER_ROW: usize = 3;
const ROW: f32 = 100.;

/// A vertical page 500 high with 2100 of content, whose row from 300 to 400 is a scroll view of its own.
fn nested(
    inner_direction: Direction,
    inner_content: (f32, f32),
    virtualized: bool,
) -> (TestingRunner, Nested) {
    let (mut test, controllers) = TestingRunner::new(
        move || {
            let Nested { page, inner } = use_consume::<Nested>();
            let row = move |index: usize| {
                if index == INNER_ROW {
                    ScrollView::new_controlled(inner)
                        .key(index)
                        .direction(inner_direction)
                        .width(Size::fill())
                        .height(Size::px(ROW))
                        .child(
                            rect()
                                .width(Size::px(inner_content.0))
                                .height(Size::px(inner_content.1)),
                        )
                        .into_element()
                } else {
                    rect()
                        .key(index)
                        .width(Size::fill())
                        .height(Size::px(ROW))
                        .into_element()
                }
            };
            if virtualized {
                VirtualScrollView::new_controlled(move |item, _| row(item.index), page)
                    .length(21usize)
                    .item_size(ROW)
                    .into_element()
            } else {
                ScrollView::new_controlled(page)
                    .children((0..21).map(row))
                    .into_element()
            }
        },
        (500., 500.).into(),
        |runner| {
            runner.provide_root_context(|| Nested {
                page: ScrollController::new(0, 0),
                inner: ScrollController::new(0, 0),
            })
        },
        1.,
    );
    test.sync_and_update();
    (test, controllers)
}

fn position(test: &mut TestingRunner, controller: ScrollController) -> (i32, i32) {
    let position = test.run_in(|| controller.position());
    (position.x as i32, position.y as i32)
}

/// A finger put on a row that scrolls sideways and moved up scrolls the page, at every movement and by that movement.
#[test]
fn drag_over_a_nested_scroll_view_of_the_other_axis_scrolls_the_parent() {
    for virtualized in [false, true] {
        // A row with nothing to scroll, then one with more than fits.
        for inner_width in [200., 1500.] {
            let (mut test, Nested { page, inner }) =
                nested(Direction::Horizontal, (inner_width, ROW), virtualized);

            test.press_touch((100., 350.));
            for step in 1..=20 {
                // Up, and a little to the side as a finger goes.
                test.move_touch((100. + step as f64, 350. - step as f64 * 10.));
                assert_eq!(
                    position(&mut test, page),
                    (0, -step * 10),
                    "page after movement {step} (virtualized: {virtualized}, row content: {inner_width})"
                );
                assert_eq!(position(&mut test, inner), (0, 0));
            }
            test.release_touch((120., 150.));
        }
    }
}

/// The same finger moved sideways scrolls the row, and the page stays where it is.
#[test]
fn drag_along_a_nested_scroll_view_scrolls_it_and_not_the_parent() {
    for virtualized in [false, true] {
        let (mut test, Nested { page, inner }) =
            nested(Direction::Horizontal, (1500., ROW), virtualized);

        test.press_touch((400., 350.));
        for step in 1..=20 {
            test.move_touch((400. - step as f64 * 10., 350. - step as f64));
            assert_eq!(
                position(&mut test, inner),
                (-step * 10, 0),
                "row after movement {step} (virtualized: {virtualized})"
            );
            assert_eq!(position(&mut test, page), (0, 0));
        }
        test.release_touch((200., 330.));
    }
}

/// Two scroll views of the same axis, one inside the other: the inner one has the drag that starts on it, at every movement.
#[test]
fn drag_over_a_nested_scroll_view_of_the_same_axis_scrolls_the_inner_one() {
    for virtualized in [false, true] {
        let (mut test, Nested { page, inner }) =
            nested(Direction::Vertical, (200., 1000.), virtualized);

        test.press_touch((100., 390.));
        for step in 1..=20 {
            test.move_touch((100., 390. - step as f64 * 10.));
            assert_eq!(
                position(&mut test, inner),
                (0, -step * 10),
                "inner view after movement {step} (virtualized: {virtualized})"
            );
            assert_eq!(position(&mut test, page), (0, 0));
        }
        test.release_touch((100., 190.));

        // A drag that starts outside the inner view is the page's.
        let inner_before = position(&mut test, inner);
        test.press_touch((100., 450.));
        for step in 1..=20 {
            test.move_touch((100., 450. - step as f64 * 10.));
            assert_eq!(position(&mut test, page), (0, -step * 10));
        }
        test.release_touch((100., 250.));
        assert_eq!(position(&mut test, inner).1, inner_before.1);
    }
}

/// A scroll view that was under one drag and did not take it is not moved by a later drag that starts elsewhere.
#[test]
fn nested_scroll_view_does_not_keep_a_drag_it_did_not_take() {
    for virtualized in [false, true] {
        let (mut test, Nested { page, inner }) =
            nested(Direction::Horizontal, (1500., ROW), virtualized);

        // Over the row, up: the page's.
        test.press_touch((400., 350.));
        for step in 1..=10 {
            test.move_touch((400., 350. - step as f64 * 10.));
        }
        test.release_touch((400., 250.));
        assert_eq!(position(&mut test, inner), (0, 0));

        // Below the row, sideways: nothing scrolls that way there, and the row is not under the finger.
        let page_before = position(&mut test, page);
        test.press_touch((400., 450.));
        for step in 1..=10 {
            test.move_touch((400. - step as f64 * 10., 450.));
        }
        test.release_touch((300., 450.));
        assert_eq!(position(&mut test, inner), (0, 0));
        assert_eq!(position(&mut test, page).0, page_before.0);
    }
}

/// A press on a button inside a scroll view that has nothing to scroll is still a press when the finger slides a little.
#[test]
fn scroll_view_with_nothing_to_scroll_does_not_take_the_press() {
    let (mut test, presses) = TestingRunner::new(
        move || {
            let mut presses = use_consume::<State<usize>>();
            ScrollView::new().child(
                Button::new()
                    .height(Size::px(150.))
                    .on_press(move |_| *presses.write() += 1)
                    .child("Button"),
            )
        },
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(0usize)),
        1.,
    );
    test.sync_and_update();

    test.press_touch((40., 80.));
    test.move_touch((43., 84.));
    test.release_touch((43., 84.));
    assert_eq!(*presses.peek(), 1);
}
