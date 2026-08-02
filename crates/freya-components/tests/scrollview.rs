use std::time::{
    Duration,
    Instant,
};

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
pub fn scroll_view_scrollbar() {
    fn scroll_view_scrollbar_app() -> impl IntoElement {
        ScrollView::new()
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
            .child(rect().height(Size::px(200.)).width(Size::px(200.)))
    }

    let mut test = launch_test(scroll_view_scrollbar_app);
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

    assert!(content[0].is_visible());
    assert!(content[1].is_visible());
    assert!(content[2].is_visible());
    assert!(!content[3].is_visible());

    // Scroll to the bottom with arrows
    test.press_key(Key::Named(NamedKey::End));

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
pub fn scroll_view_wheel_latching() {
    use std::time::Duration;

    // Longer than the shared wheel-gesture window, so the next event starts a new gesture.
    fn end_gesture() {
        std::thread::sleep(Duration::from_millis(300));
    }

    fn latching_app() -> impl IntoElement {
        // A latched 200px inner scroll view (400px of content) at the top of an outer scroll
        // view whose own content (200 + 3x200 = 800px) overflows the 500px viewport.
        ScrollView::new()
            .child(
                ScrollView::new()
                    .height(Size::px(200.))
                    .latch_wheel()
                    .child(rect().height(Size::px(200.)).width(Size::fill()))
                    .child(rect().height(Size::px(200.)).width(Size::fill())),
            )
            .child(rect().height(Size::px(200.)).width(Size::fill()))
            .child(rect().height(Size::px(200.)).width(Size::fill()))
            .child(rect().height(Size::px(200.)).width(Size::fill()))
    }

    let mut test = launch_test(latching_app);
    let outer = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let outer_content = outer.children()[0].children()[0].children();
    let inner_content = outer_content[0].children()[0].children()[0].children();

    // Initial state: the inner view shows only its first item; the outer's last item is
    // below the fold.
    assert!(inner_content[0].is_visible());
    assert!(!inner_content[1].is_visible());
    assert!(!outer_content[3].is_visible());

    // A gesture starting over the inner view (which can move down) latches to it: the inner
    // scrolls to its end, and the surplus of the same gesture must NOT chain to the outer.
    test.scroll((100., 100.), (0., -200.));
    assert!(inner_content[1].is_visible());
    assert!(!outer_content[3].is_visible());
    test.scroll((100., 100.), (0., -300.)); // same gesture, inner already at its end
    assert!(!outer_content[3].is_visible());

    // A NEW gesture starting with the inner at its end passes through wholesale, so the
    // outer takes over.
    end_gesture();
    test.scroll((100., 100.), (0., -300.));
    assert!(outer_content[3].is_visible());

    // Reset: scroll the outer back up (the inner sits off screen above), then latch a new
    // gesture to the inner to bring it back to its top.
    end_gesture();
    test.scroll((100., 250.), (0., 300.));
    end_gesture();
    test.scroll((100., 100.), (0., 200.));
    assert!(!inner_content[1].is_visible());
    assert!(!outer_content[3].is_visible());

    // A gesture starting over the OUTER (below the inner) belongs to it: when the scrolled
    // content brings the inner under the pointer mid-gesture, the inner must not steal the
    // gesture even though it could scroll, so the outer keeps moving and the inner stays put.
    end_gesture();
    test.scroll((100., 250.), (0., -150.));
    test.scroll((100., 25.), (0., -150.)); // now over the inner, same gesture
    assert!(!inner_content[1].is_visible());
    assert!(outer_content[3].is_visible());
}

/// Every latching view that saw a gesture's first event is a candidate for it, and the innermost
/// one able to move takes it. An outer latching view therefore gets the gesture exactly when the
/// inner one declined it, rather than being excluded for not having been the first to look.
#[test]
pub fn scroll_view_wheel_latching_nested() {
    fn nested_latching_app() -> impl IntoElement {
        // A latched inner view inside a latched outer view, both able to scroll.
        ScrollView::new()
            .latch_wheel()
            .child(
                ScrollView::new()
                    .height(Size::px(200.))
                    .latch_wheel()
                    .child(rect().height(Size::px(200.)).width(Size::fill()))
                    .child(rect().height(Size::px(200.)).width(Size::fill())),
            )
            .child(rect().height(Size::px(200.)).width(Size::fill()))
            .child(rect().height(Size::px(200.)).width(Size::fill()))
            .child(rect().height(Size::px(200.)).width(Size::fill()))
    }

    let mut test = launch_test(nested_latching_app);
    let outer = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let outer_content = outer.children()[0].children()[0].children();
    let inner_content = outer_content[0].children()[0].children()[0].children();

    // The inner view can move, so it takes the gesture and the outer never sees it.
    test.scroll((100., 100.), (0., -200.));
    assert!(inner_content[1].is_visible());
    assert!(!outer_content[3].is_visible());

    // A new gesture with the inner at its end: the inner declines, so the outer takes it.
    std::thread::sleep(Duration::from_millis(300));
    test.scroll((100., 100.), (0., -300.));
    assert!(outer_content[3].is_visible());
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

/// Acceleration lands on every scroll surface, so it is applied in `scrollviews::shared` rather
/// than per view. Same gesture as `virtual_scroll_view_wheel_acceleration`, on a plain view.
#[test]
pub fn scroll_view_wheel_acceleration() {
    fn scroll_view_acceleration_app() -> impl IntoElement {
        ScrollView::new().children((0..10).map(|_| {
            rect()
                .height(Size::px(200.))
                .width(Size::px(200.))
                .into_element()
        }))
    }

    let first_visible_item = |test: &TestingRunner| {
        let scrollview = test
            .find(|node, element| {
                Rect::try_downcast(element)
                    .filter(|rect| {
                        rect.accessibility.builder.role() == AccessibilityRole::ScrollView
                    })
                    .map(move |_| node)
            })
            .unwrap();
        let content = scrollview.children()[0].children()[0].children();
        content.iter().position(|child| child.is_visible()).unwrap()
    };

    // A fast gesture: 53 pixels for the first notch, then the second accelerated to the ceiling
    // and capped at the 500 pixel viewport. 553 pixels in, the item spanning 400 to 600 is the
    // first still on screen.
    let mut test = launch_test(scroll_view_acceleration_app);
    test.sync_and_update();
    let start = Instant::now();
    test.scroll_lines_at((5., 5.), (0., -1.), start);
    test.scroll_lines_at((5., 5.), (0., -1.), start + Duration::from_millis(15));
    assert_eq!(first_visible_item(&test), 2);

    // The same two notches at a reading pace: 106 pixels, so the first item is still on screen.
    let mut test = launch_test(scroll_view_acceleration_app);
    test.sync_and_update();
    let start = Instant::now();
    test.scroll_lines_at((5., 5.), (0., -1.), start);
    test.scroll_lines_at((5., 5.), (0., -1.), start + Duration::from_millis(90));
    assert_eq!(first_visible_item(&test), 0);
}
