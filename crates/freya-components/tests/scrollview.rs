use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn scroll_drag_only_cancels_press_when_content_overflows() {
    for (virtualized, length) in [(false, 1usize), (false, 20), (true, 1), (true, 20)] {
        let (mut test, (presses, controller)) = TestingRunner::new(
            move || {
                let mut presses = use_consume::<State<usize>>();
                let controller = use_consume::<ScrollController>();
                let button = move |index| {
                    Button::new()
                        .key(index)
                        .height(Size::px(150.))
                        .on_press(move |_| *presses.write() += 1)
                        .child(format!("Button {index}"))
                        .into_element()
                };

                if virtualized {
                    VirtualScrollView::new_controlled(move |item, _| button(item.index), controller)
                        .length(length)
                        .item_size(150.)
                        .into_element()
                } else {
                    ScrollView::new_controlled(controller)
                        .children((0..length).map(button))
                        .into_element()
                }
            },
            (500., 500.).into(),
            |runner| {
                let presses = runner.provide_root_context(|| State::create(0usize));
                let controller = runner.provide_root_context(|| ScrollController::new(0, 0));
                (presses, controller)
            },
            1.,
        );
        test.sync_and_update();

        test.press_touch((40., 80.));
        test.move_touch((40., 50.));
        assert_eq!(test.run_in(|| controller.position().y) < 0., length > 1);
        test.release_touch((40., 50.));
        assert_eq!(*presses.peek(), usize::from(length == 1));
    }
}

#[test]
fn nested_scroll_drag_ownership() {
    for virtualized in [false, true] {
        for (direction, content, movement, inner_scrolls) in [
            (Direction::Horizontal, (200., 100.), (1., -10.), false),
            (Direction::Horizontal, (1500., 100.), (1., -10.), false),
            (Direction::Horizontal, (1500., 100.), (-10., -1.), true),
            (Direction::Vertical, (200., 1000.), (0., -10.), true),
        ] {
            let (mut test, (mut page, mut inner)) = TestingRunner::new(
                move || {
                    let (page, inner) = use_consume::<(ScrollController, ScrollController)>();
                    let row = move |index| {
                        if index == 3 {
                            ScrollView::new_controlled(inner)
                                .key(index)
                                .direction(direction)
                                .width(Size::fill())
                                .height(Size::px(100.))
                                .child(
                                    rect()
                                        .width(Size::px(content.0))
                                        .height(Size::px(content.1)),
                                )
                                .into_element()
                        } else {
                            rect()
                                .key(index)
                                .width(Size::fill())
                                .height(Size::px(100.))
                                .into_element()
                        }
                    };
                    if virtualized {
                        VirtualScrollView::new_controlled(move |item, _| row(item.index), page)
                            .length(10usize)
                            .item_size(100.)
                            .into_element()
                    } else {
                        ScrollView::new_controlled(page)
                            .children((0..10).map(row))
                            .into_element()
                    }
                },
                (500., 500.).into(),
                |runner| {
                    runner.provide_root_context(|| {
                        (ScrollController::new(0, 0), ScrollController::new(0, 0))
                    })
                },
                1.,
            );
            test.sync_and_update();
            let position = |test: &TestingRunner, controller: ScrollController| {
                test.run_in(|| {
                    let position = controller.position();
                    (position.x as i32, position.y as i32)
                })
            };

            test.press_touch((400., 350.));
            for step in 1..=3 {
                test.move_touch((
                    400. + movement.0 * step as f64,
                    350. + movement.1 * step as f64,
                ));
                let expected_inner = if !inner_scrolls {
                    (0, 0)
                } else if direction == Direction::Horizontal {
                    (-10 * step, 0)
                } else {
                    (0, -10 * step)
                };
                assert_eq!(position(&test, inner), expected_inner);
                assert_eq!(
                    position(&test, page),
                    (0, if inner_scrolls { 0 } else { -10 * step })
                );
            }
            test.release_touch((400. + movement.0 * 3., 350. + movement.1 * 3.));

            test.run_in(|| {
                page.stop();
                inner.stop();
            });
            test.sync_and_update();

            // Start a new drag outside the inner view.
            let inner_before = position(&test, inner);
            let page_before = position(&test, page);
            test.press_touch((400., 450.));
            test.move_touch((370., 450.));
            assert_eq!(position(&test, inner), inner_before);
            assert_eq!(position(&test, page).0, page_before.0);
            test.move_touch((370., 420.));
            assert_eq!(position(&test, inner), inner_before);
            assert_eq!(position(&test, page).1, page_before.1 - 30);
            test.release_touch((370., 420.));
        }
    }
}

#[test]
fn scroll_view_auto_hide_scrollbars() {
    for auto_hide_scrollbars in [true, false] {
        let mut test = launch_test(move || {
            ScrollView::new()
                .auto_hide_scrollbars(auto_hide_scrollbars)
                .scrollbar(move |context: ScrollBarContext| {
                    assert_eq!(context.auto_hide_scrollbars, auto_hide_scrollbars);
                    ScrollBar::new(context).into()
                })
                .child(rect().width(Size::px(1000.)).height(Size::px(1000.)))
        });
        test.sync_and_update();
        let scrollbars = test.find_many(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.relative_layer == 999.into())
                .map(|_| node)
        });
        assert_eq!(scrollbars.len(), 2);
        for scrollbar in &scrollbars {
            assert!(scrollbar.layout().area.width() > 0.);
            assert!(scrollbar.layout().area.height() > 0.);
        }

        test.poll(Duration::from_millis(20), Duration::from_millis(1000));

        for scrollbar in &scrollbars {
            assert_eq!(scrollbar.layout().area.width() > 0., !auto_hide_scrollbars);
            assert_eq!(scrollbar.layout().area.height() > 0., !auto_hide_scrollbars);
            assert_eq!(scrollbar.children().is_empty(), auto_hide_scrollbars);
        }
    }
}

#[test]
fn scroll_view_mixed_auto_hide_scrollbars() {
    fn app() -> impl IntoElement {
        rect()
            .child(
                ScrollView::new().height(Size::percent(50.)).child(
                    rect().spacing(6.).children(
                        (0..30).map(|_| rect().width(Size::fill()).height(Size::px(80.))),
                    ),
                ),
            )
            .child(
                ScrollView::new()
                    .auto_hide_scrollbars(false)
                    .height(Size::percent(50.))
                    .child(rect().horizontal().spacing(6.).children(
                        (0..30).map(|_| rect().width(Size::px(80.)).height(Size::fill())),
                    )),
            )
    }

    let mut test = launch_test(app);
    test.sync_and_update();
    let scrollbars = test.find_many(|node, element| {
        Rect::try_downcast(element)
            .filter(|rect| rect.relative_layer == 999.into())
            .map(|_| node)
    });
    assert_eq!(scrollbars.len(), 2);
    test.move_cursor((495., 20.));
    test.sync_and_update();
    test.poll(Duration::from_millis(20), Duration::from_secs(2));
    for scrollbar in &scrollbars {
        assert!(scrollbar.layout().area.width() > 0.);
        assert!(!scrollbar.children().is_empty());
    }

    test.move_cursor((100., 100.));
    test.sync_and_update();
    test.poll(Duration::from_millis(20), Duration::from_secs(2));

    let visible_scrollbars = scrollbars
        .iter()
        .filter(|scrollbar| scrollbar.layout().area.width() > 0.)
        .collect::<Vec<_>>();
    for scrollbar in &scrollbars {
        if scrollbar.layout().area.width() == 0. {
            assert!(scrollbar.children().is_empty());
        }
    }
    assert_eq!(visible_scrollbars.len(), 1);
    assert!(visible_scrollbars[0].layout().area.width() > 20.);
    assert_eq!(visible_scrollbars[0].layout().area.height(), 20.);
}

#[test]
pub fn scroll_view_wheel() {
    fn scroll_view_wheel_app() -> impl IntoElement {
        ScrollView::new()
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
    }

    let mut test = launch_test(scroll_view_wheel_app);
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    // Only the first three items are visible
    // Scrollview height is 500 and the user hasn't scrolled yet
    assert!(content[0].is_visible()); // 1. 0   -> 200, 200 < 500
    assert!(content[1].is_visible()); // 2. 200 -> 400, 200 < 500
    assert!(content[2].is_visible()); // 3. 400 -> 600, 400 < 500
    assert!(!content[3].is_visible()); // 4. 600 -> 800, 600 is NOT < 500, which means it is not visible.

    test.scroll((5., 5.), (0., -300.));

    // Only the last three items are visible
    // Scrollview height is 500 but the user has scrolled 300 pixels
    assert!(!content[0].is_visible()); // 1. 0   -> 200, 200 is NOT > 300, which means it is not visible.
    assert!(content[1].is_visible()); // 2. 200 -> 400, 400 > 300
    assert!(content[2].is_visible()); // 3. 400 -> 600, 600 > 300
    assert!(content[3].is_visible()); // 4. 600 -> 800, 800 > 300
}

#[test]
pub fn scroll_view_smooth_scrolling() {
    fn scroll_view_smooth_scrolling_app() -> impl IntoElement {
        ScrollView::new()
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
    }

    let mut test = launch_test(scroll_view_smooth_scrolling_app);
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    test.send_event(PlatformEvent::Wheel {
        name: WheelEventName::Wheel,
        scroll: (0., -300.).into(),
        cursor: (5., 5.).into(),
        source: WheelSource::Line,
        phase: TouchPhase::Moved,
    });
    test.sync_and_update();

    // A line-based wheel scroll animates, so nothing has moved yet
    assert!(content[0].is_visible());
    assert!(!content[3].is_visible());

    test.poll(Duration::from_millis(16), Duration::from_secs(1));

    // The animation has settled on the destination
    assert!(!content[0].is_visible());
    assert!(content[3].is_visible());
}

#[test]
pub fn scroll_view_hover_updates_on_scroll() {
    fn scroll_view_hover_app() -> impl IntoElement {
        let mut hovered = use_state(|| None::<usize>);

        rect()
            .child(
                label()
                    .height(Size::px(100.))
                    .text(format!("hovered {:?}", hovered())),
            )
            .child(
                ScrollView::new()
                    .height(Size::px(400.))
                    .children((0..4).map(|i| {
                        rect()
                            .key(i)
                            .height(Size::px(200.))
                            .width(Size::px(200.))
                            .on_pointer_enter(move |_| hovered.set(Some(i)))
                    })),
            )
    }

    let mut test = launch_test(scroll_view_hover_app);

    let hovered_label = |test: &TestingRunner| {
        test.find(|_, element| Label::try_downcast(element).map(|label| label.text.to_string()))
            .unwrap()
    };

    assert_eq!(hovered_label(&test), "hovered None");

    test.move_cursor((100., 350.));
    test.sync_and_update();
    assert_eq!(hovered_label(&test), "hovered Some(1)");

    // Scrolling moves the third item under the cursor
    test.scroll((100., 350.), (0., -300.));
    assert_eq!(hovered_label(&test), "hovered Some(2)");
}

#[test]
pub fn scroll_view_scrollbar() {
    fn scroll_view_scrollbar_app() -> impl IntoElement {
        ScrollView::new()
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
    }

    let mut test = launch_test(scroll_view_scrollbar_app);
    test.animation_clock().disable();
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    // Only the first three items are visible
    // Scrollview height is 500 and the user hasn't scrolled yet
    assert!(content[0].is_visible()); // 1. 0   -> 200, 200 < 500
    assert!(content[1].is_visible()); // 2. 200 -> 400, 200 < 500
    assert!(content[2].is_visible()); // 3. 400 -> 600, 400 < 500
    assert!(!content[3].is_visible()); // 4. 600 -> 800, 600 is NOT < 500, which means it is not visible.

    // Simulate the user dragging the scrollbar
    test.move_cursor((495., 20.));
    test.sync_and_update();
    test.press_cursor((495., 20.));
    test.sync_and_update();
    test.move_cursor((495., 320.));
    test.sync_and_update();
    test.release_cursor((495., 320.));
    test.sync_and_update();

    // Only the last three items are visible
    // Scrollview height is 500 but the user has dragged the scrollbar 300 pixels
    assert!(!content[0].is_visible()); // 1. 0   -> 200, 200 is NOT > 300, which means it is not visible.
    assert!(content[1].is_visible()); // 2. 200 -> 400, 400 > 300
    assert!(content[2].is_visible()); // 3. 400 -> 600, 600 > 300
    assert!(content[3].is_visible()); // 4. 600 -> 800, 800 > 300

    // Scroll up with arrows
    for _ in 0..5 {
        test.press_key(Key::Named(NamedKey::ArrowUp));
        test.poll_n(Duration::from_millis(1), 5);
    }

    assert!(content[0].is_visible());
    assert!(content[1].is_visible());
    assert!(content[2].is_visible());
    assert!(!content[3].is_visible());

    // Scroll to the bottom with arrows
    test.press_key(Key::Named(NamedKey::End));
    test.poll_n(Duration::from_millis(1), 5);

    assert!(!content[0].is_visible());
    assert!(content[1].is_visible());
    assert!(content[2].is_visible());
    assert!(content[3].is_visible());
}

#[test]
pub fn scroll_view_drag_scrolling() {
    fn scroll_view_drag_scrolling_app() -> impl IntoElement {
        ScrollView::new()
            .drag_scrolling(true)
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
    }

    let mut test = launch_test(scroll_view_drag_scrolling_app);
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    // Initial state: first three items visible
    assert!(content[0].is_visible());
    assert!(content[1].is_visible());
    assert!(content[2].is_visible());
    assert!(!content[3].is_visible());

    // Simulate a touch drag: press down on content, drag upward (scroll down)
    test.press_touch((100., 400.));
    test.move_touch((100., 100.));
    test.release_touch((100., 100.));

    // After dragging 300px upward, first item should be hidden and last visible
    assert!(!content[0].is_visible());
    assert!(content[1].is_visible());
    assert!(content[2].is_visible());
    assert!(content[3].is_visible());
}

#[test]
pub fn scroll_view_drag_scrolling_release_stops() {
    fn scroll_view_drag_release_app() -> impl IntoElement {
        ScrollView::new()
            .drag_scrolling(true)
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
    }

    let mut test = launch_test(scroll_view_drag_release_app);
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    // Drag down a small amount, then release
    test.press_touch((100., 300.));
    test.move_touch((100., 200.));
    test.release_touch((100., 200.));

    // Scrolled 100px: all four items partially in view since content is 800px, viewport 500px
    assert!(content[0].is_visible());
    assert!(content[1].is_visible());
    assert!(content[2].is_visible());
    assert!(!content[3].is_visible());

    // Move touch further after releasing, should NOT scroll further
    test.move_touch((100., 50.));

    // Visibility should remain unchanged since drag ended on release
    assert!(content[0].is_visible());
    assert!(content[1].is_visible());
    assert!(content[2].is_visible());
    assert!(!content[3].is_visible());
}

#[test]
pub fn scroll_view_drag_scrolling_horizontal() {
    fn scroll_view_drag_horizontal_app() -> impl IntoElement {
        ScrollView::new()
            .drag_scrolling(true)
            .direction(Direction::Horizontal)
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
    }

    let mut test = launch_test(scroll_view_drag_horizontal_app);
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    // Initial state: first three items visible (viewport 500px wide, items 200px each)
    assert!(content[0].is_visible());
    assert!(content[1].is_visible());
    assert!(content[2].is_visible());
    assert!(!content[3].is_visible());

    // Drag left (scroll right) by 300px
    test.press_touch((400., 100.));
    test.move_touch((100., 100.));
    test.release_touch((100., 100.));

    // After dragging 300px, first item hidden, last item visible
    assert!(!content[0].is_visible());
    assert!(content[1].is_visible());
    assert!(content[2].is_visible());
    assert!(content[3].is_visible());
}

#[test]
pub fn scroll_view_scrollbar_smaller_than_thumb() {
    fn small_scroll_view_app() -> impl IntoElement {
        rect()
            .height(Size::px(48.))
            .width(Size::px(200.))
            .child(ScrollView::new().children((0..30).map(|i| {
                label()
                    .key(i)
                    .height(Size::px(20.))
                    .text(format!("Row {i}"))
            })))
    }

    let mut test = launch_test(small_scroll_view_app);
    test.sync_and_update();

    test.move_cursor((100., 24.));
    test.sync_and_update();
    test.scroll((100., 24.), (0., -20.));
    test.sync_and_update();

    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let scrolled_before = scrollview.children()[0].children()[0].children()[0]
        .layout()
        .area
        .min_y();

    test.move_cursor((192., 5.));
    test.sync_and_update();
    test.press_cursor((192., 5.));
    test.sync_and_update();
    test.move_cursor((192., 30.));
    test.sync_and_update();
    test.release_cursor((192., 30.));
    test.sync_and_update();

    let scrolled_after = scrollview.children()[0].children()[0].children()[0]
        .layout()
        .area
        .min_y();
    assert_eq!(scrolled_before, scrolled_after);
}

#[test]
pub fn scroll_view_drag_taken_over_from_inside() {
    fn app() -> impl IntoElement {
        let mut moves = use_consume::<State<usize>>();
        ScrollView::new()
            .drag_scrolling(true)
            .child(
                rect()
                    .height(Size::px(200.))
                    .width(Size::px(200.))
                    .on_pointer_move(move |_| *moves.write() += 1),
            )
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
    }

    let (mut test, moves) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(0usize)),
        1.,
    );
    test.sync_and_update();
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    test.press_touch((100., 190.));
    test.move_touch((100., 189.));
    test.run_in(TouchDrag::take_over);
    test.move_touch((100., 100.));
    test.move_touch((100., 10.));
    test.release_touch((100., 10.));
    assert!(content[0].is_visible());
    assert!(!content[3].is_visible());
    assert!(*moves.peek() > 1);

    // Once the finger lifted, drags scroll again.
    test.press_touch((100., 400.));
    test.move_touch((100., 100.));
    test.release_touch((100., 100.));
    assert!(content[3].is_visible());
}

#[test]
fn nested_scroll_drag_taken_over_before_or_after_ownership() {
    for outer_virtualized in [false, true] {
        for inner_virtualized in [false, true] {
            for (initial_movement, cancelled) in
                [(0., false), (1., false), (10., false), (10., true)]
            {
                let (mut test, (page, inner)) = TestingRunner::new(
                    move || {
                        let (page, inner) = use_consume::<(ScrollController, ScrollController)>();
                        let mut take_on_down = use_state(move || initial_movement == 0.);
                        let row = move |index| {
                            if index == 0 {
                                let content = move |index| {
                                    rect()
                                        .key(index)
                                        .width(Size::fill())
                                        .height(Size::px(200.))
                                        .on_pointer_down(move |_| {
                                            if *take_on_down.peek() {
                                                TouchDrag::take_over();
                                                take_on_down.set(false);
                                            }
                                        })
                                        .into_element()
                                };
                                if inner_virtualized {
                                    VirtualScrollView::new_controlled(
                                        move |item, _| content(item.index),
                                        inner,
                                    )
                                    .key(index)
                                    .length(4usize)
                                    .item_size(200.)
                                    .height(Size::px(200.))
                                    .into_element()
                                } else {
                                    ScrollView::new_controlled(inner)
                                        .key(index)
                                        .height(Size::px(200.))
                                        .children((0..4).map(content))
                                        .into_element()
                                }
                            } else {
                                rect().key(index).height(Size::px(200.)).into_element()
                            }
                        };
                        if outer_virtualized {
                            VirtualScrollView::new_controlled(move |item, _| row(item.index), page)
                                .length(4usize)
                                .item_size(200.)
                                .into_element()
                        } else {
                            ScrollView::new_controlled(page)
                                .children((0..4).map(row))
                                .into_element()
                        }
                    },
                    (500., 500.).into(),
                    |runner| {
                        runner.provide_root_context(|| {
                            (ScrollController::new(0, 0), ScrollController::new(0, 0))
                        })
                    },
                    1.,
                );
                test.sync_and_update();
                assert_eq!(test.run_in(|| page.position().y), 0.);
                assert_eq!(test.run_in(|| inner.position().y), 0.);

                test.press_touch((100., 150.));
                test.move_touch((100., 150. - initial_movement));
                let inner_before = test.run_in(|| inner.position().y);
                assert_eq!(inner_before < 0., initial_movement > 2.);
                if initial_movement > 0. {
                    test.run_in(TouchDrag::take_over);
                    test.run_in(TouchDrag::take_over);
                }
                test.move_touch((100., 100.));
                test.move_touch((100., 50.));
                assert_eq!(test.run_in(|| page.position().y), 0.);
                assert_eq!(test.run_in(|| inner.position().y), inner_before);
                if cancelled {
                    test.send_event(PlatformEvent::Touch {
                        name: TouchEventName::TouchCancel,
                        location: (100., 50.).into(),
                        finger_id: 0,
                        phase: TouchPhase::Cancelled,
                        force: None,
                    });
                    test.sync_and_update();
                } else {
                    test.release_touch((100., 50.));
                }
                assert_eq!(test.run_in(|| inner.position().y), inner_before);

                test.press_touch((100., 150.));
                test.move_touch((100., 130.));
                assert_eq!(test.run_in(|| page.position().y), 0.);
                assert_eq!(test.run_in(|| inner.position().y), inner_before - 20.);
                test.release_touch((100., 130.));
            }
        }
    }
}
