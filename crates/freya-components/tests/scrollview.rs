use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

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
    }
    test.poll(Duration::from_millis(1), Duration::from_millis(20));

    assert!(content[0].is_visible());
    assert!(content[1].is_visible());
    assert!(content[2].is_visible());
    assert!(!content[3].is_visible());

    // Scroll to the bottom with arrows
    test.press_key(Key::Named(NamedKey::End));
    test.poll(Duration::from_millis(1), Duration::from_millis(20));

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
