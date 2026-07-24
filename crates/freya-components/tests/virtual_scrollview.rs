use freya::prelude::*;
use freya_core::prelude::Label;
use freya_testing::prelude::*;

#[test]
pub fn virtual_scroll_view_wheel() {
    fn virtual_scroll_view_wheel_app() -> impl IntoElement {
        VirtualScrollView::new(|i, _| {
            label()
                .key(i)
                .height(Size::px(50.))
                .text(format!("{i} Hello, World!"))
                .into()
        })
        .length(30usize)
        .item_size(50.)
    }

    let mut test = launch_test(virtual_scroll_view_wheel_app);
    test.sync_and_update();
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    assert_eq!(content.len(), 11);

    // Check that visible items are from indexes 0 to 10, because 500 / 50 = 10 + 1 (for smooth scrolling) = 11.
    for (n, i) in (0..11).enumerate() {
        let child = &content[n];
        assert_eq!(
            Label::try_downcast(&*child.element()).unwrap().text,
            format!("{i} Hello, World!").as_str()
        );
    }

    test.scroll((5., 5.), (0., -300.));

    let content = scrollview.children()[0].children()[0].children();
    assert_eq!(content.len(), 11);

    // It has scrolled 300 pixels, which equals to 6 items because 300 / 50 = 6
    // So we must start checking from 6 to +10, 16 in this case because 6 + 10 = 16 + 1 (for smooth scrolling) = 17.
    for (n, i) in (6..17).enumerate() {
        let child = &content[n];
        assert_eq!(
            Label::try_downcast(&*child.element()).unwrap().text,
            format!("{i} Hello, World!").as_str()
        );
    }
}

#[test]
pub fn virtual_scroll_view_scrollbar() {
    fn virtual_scroll_view_scrollbar_app() -> impl IntoElement {
        VirtualScrollView::new(|i, _| {
            label()
                .key(i)
                .height(Size::px(50.))
                .text(format!("{i} Hello, World!"))
                .into()
        })
        .length(30usize)
        .item_size(50.)
    }

    let mut test = launch_test(virtual_scroll_view_scrollbar_app);
    test.sync_and_update();
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    assert_eq!(content.len(), 11);

    // Check that visible items are from indexes 0 to 10, because 500 / 50 = 10 + 1 (for smooth scrolling) = 11.
    for (n, i) in (0..11).enumerate() {
        let child = &content[n];
        assert_eq!(
            Label::try_downcast(&*child.element()).unwrap().text,
            format!("{i} Hello, World!").as_str()
        );
    }

    // Simulate the user dragging the scrollbar
    test.move_cursor((495., 20.));
    test.sync_and_update();
    test.press_cursor((495., 20.));
    test.sync_and_update();
    test.move_cursor((495., 320.));
    test.sync_and_update();
    test.release_cursor((495., 320.));
    test.sync_and_update();

    let content = scrollview.children()[0].children()[0].children();
    assert_eq!(content.len(), 11);

    // It has dragged the scrollbar 300 pixels
    for (n, i) in (18..29).enumerate() {
        let child = &content[n];
        assert_eq!(
            Label::try_downcast(&*child.element()).unwrap().text,
            format!("{i} Hello, World!").as_str()
        );
    }

    // Scroll up with arrows
    for _ in 0..11 {
        test.press_key(Key::Named(NamedKey::ArrowUp));
    }

    let content = scrollview.children()[0].children()[0].children();
    assert_eq!(content.len(), 11);

    for (n, i) in (0..11).enumerate() {
        let child = &content[n];
        assert_eq!(
            Label::try_downcast(&*child.element()).unwrap().text,
            format!("{i} Hello, World!").as_str()
        );
    }

    // Scroll to the bottom with arrows
    test.press_key(Key::Named(NamedKey::End));

    let content = scrollview.children()[0].children()[0].children();
    assert_eq!(content.len(), 10);

    for (n, i) in (20..30).enumerate() {
        let child = &content[n];
        assert_eq!(
            Label::try_downcast(&*child.element()).unwrap().text,
            format!("{i} Hello, World!").as_str()
        );
    }
}

#[test]
pub fn virtual_scroll_view_max_height() {
    fn virtual_scroll_view_max_height_app() -> impl IntoElement {
        VirtualScrollView::new(|i, _| {
            label()
                .key(i)
                .height(Size::px(50.))
                .text(format!("{i} Hello, World!"))
                .into()
        })
        .length(30usize)
        .item_size(50.)
        .max_height(Size::px(250.))
    }

    let mut test = launch_test(virtual_scroll_view_max_height_app);
    test.sync_and_update();
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();

    // Capped at 250 by max_height
    assert_eq!(scrollview.layout().area.height(), 250.);

    let content = scrollview.children()[0].children()[0].children();

    // 250 / 50 = 5 + 1 (for smooth scrolling) = 6
    assert_eq!(content.len(), 6);

    for (n, i) in (0..6).enumerate() {
        let child = &content[n];
        assert_eq!(
            Label::try_downcast(&*child.element()).unwrap().text,
            format!("{i} Hello, World!").as_str()
        );
    }

    test.scroll((5., 5.), (0., -300.));

    let content = scrollview.children()[0].children()[0].children();
    assert_eq!(content.len(), 6);

    // Scrolled 300 pixels, 300 / 50 = 6 items
    for (n, i) in (6..12).enumerate() {
        let child = &content[n];
        assert_eq!(
            Label::try_downcast(&*child.element()).unwrap().text,
            format!("{i} Hello, World!").as_str()
        );
    }
}

#[test]
pub fn virtual_scroll_view_min_height() {
    fn virtual_scroll_view_min_height_app() -> impl IntoElement {
        rect().width(Size::fill()).height(Size::px(100.)).child(
            VirtualScrollView::new(|i, _| {
                label()
                    .key(i)
                    .height(Size::px(50.))
                    .text(format!("{i} Hello, World!"))
                    .into()
            })
            .length(30usize)
            .item_size(50.)
            .min_height(Size::px(300.)),
        )
    }

    let mut test = launch_test(virtual_scroll_view_min_height_app);
    test.sync_and_update();
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();

    // Grown to 300 by min_height despite the 100 from the parent
    assert_eq!(scrollview.layout().area.height(), 300.);

    let content = scrollview.children()[0].children()[0].children();

    // 300 / 50 = 6 + 1 (for smooth scrolling) = 7
    assert_eq!(content.len(), 7);

    for (n, i) in (0..7).enumerate() {
        let child = &content[n];
        assert_eq!(
            Label::try_downcast(&*child.element()).unwrap().text,
            format!("{i} Hello, World!").as_str()
        );
    }
}

#[test]
pub fn virtual_scroll_view_max_height_window_percent() {
    fn app() -> impl IntoElement {
        VirtualScrollView::new(|i, _| {
            label()
                .key(i)
                .height(Size::px(100.))
                .text(format!("{i} Hello, World!"))
                .into()
        })
        .length(30usize)
        .item_size(100.)
        .height(Size::fill())
        .max_height(Size::window_percent(50.))
    }

    let mut test = launch_test(app);
    test.sync_and_update();
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();

    // Capped at half of the 500 window
    assert_eq!(scrollview.layout().area.height(), 250.);

    let content = scrollview.children()[0].children()[0].children();
    // Built items must cover the whole viewport
    assert!(content.len() * 100 >= 250);

    // Scroll to the end, last item must be reachable
    test.scroll((5., 5.), (0., -10000.));
    let content = scrollview.children()[0].children()[0].children();
    assert_eq!(
        Label::try_downcast(&*content[content.len() - 1].element())
            .unwrap()
            .text,
        "29 Hello, World!"
    );
}

#[test]
pub fn virtual_scroll_view_controlled() {
    fn virtual_scroll_view_controlled_app() -> impl IntoElement {
        let scroll_controller = use_scroll_controller(ScrollConfig::default);

        rect()
            .horizontal()
            .content(Content::Flex)
            .child(
                VirtualScrollView::new_controlled(
                    |i, _| {
                        label()
                            .key(i)
                            .height(Size::px(50.))
                            .text(format!("{i} Hello, World!"))
                            .into()
                    },
                    scroll_controller,
                )
                .length(30usize)
                .item_size(50.)
                .width(Size::flex(1.)),
            )
            .child(
                VirtualScrollView::new_controlled(
                    |i, _| {
                        label()
                            .key(i)
                            .height(Size::px(50.))
                            .text(format!("{i} Second View"))
                            .into()
                    },
                    scroll_controller,
                )
                .length(30usize)
                .item_size(50.)
                .width(Size::flex(1.)),
            )
    }

    let mut test = launch_test(virtual_scroll_view_controlled_app);
    test.sync_and_update();

    let scrollviews = test.find_many(|node, element| {
        Rect::try_downcast(element)
            .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
            .map(move |_| node)
    });
    let scrollview1 = &scrollviews[0];
    let scrollview2 = &scrollviews[1];

    let content1 = scrollview1.children()[0].children()[0].children();
    let content2 = scrollview2.children()[0].children()[0].children();

    // Both views should start at the same position
    assert_eq!(
        Label::try_downcast(&*content1[0].element()).unwrap().text,
        "0 Hello, World!"
    );
    assert_eq!(
        Label::try_downcast(&*content2[0].element()).unwrap().text,
        "0 Second View"
    );

    // Scroll the first view
    test.scroll((100., 100.), (0., -300.));

    let content1 = scrollview1.children()[0].children()[0].children();
    let content2 = scrollview2.children()[0].children()[0].children();

    // Both views should have scrolled together (controlled)
    // 300 pixels / 50 per item = 6 items scrolled
    assert_eq!(
        Label::try_downcast(&*content1[0].element()).unwrap().text,
        "6 Hello, World!"
    );
    assert_eq!(
        Label::try_downcast(&*content2[0].element()).unwrap().text,
        "6 Second View"
    );

    // Scroll the second view
    test.scroll((400., 100.), (0., -150.));

    let content1 = scrollview1.children()[0].children()[0].children();
    let content2 = scrollview2.children()[0].children()[0].children();

    // Both views should have scrolled together again
    // Additional 150 pixels / 50 per item = 3 more items = 9 total
    assert_eq!(
        Label::try_downcast(&*content1[0].element()).unwrap().text,
        "9 Hello, World!"
    );
    assert_eq!(
        Label::try_downcast(&*content2[0].element()).unwrap().text,
        "9 Second View"
    );
}

#[test]
pub fn virtual_scroll_view_keyboard_navigation() {
    fn virtual_scroll_view_keyboard_app() -> impl IntoElement {
        VirtualScrollView::new(|i, _| {
            label()
                .key(i)
                .height(Size::px(50.))
                .text(format!("{i} Hello, World!"))
                .into()
        })
        .length(30usize)
        .item_size(50.)
    }

    let mut test = launch_test(virtual_scroll_view_keyboard_app);
    test.sync_and_update();

    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    assert_eq!(content.len(), 11);

    // Check initial position
    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "0 Hello, World!"
    );

    // Focus the scrollview by clicking and dragging on the scrollbar, then moving back
    test.move_cursor((495., 20.));
    test.sync_and_update();
    test.press_cursor((495., 20.));
    test.sync_and_update();
    test.move_cursor((495., 25.));
    test.sync_and_update();
    test.move_cursor((495., 20.));
    test.sync_and_update();
    test.release_cursor((495., 20.));
    test.sync_and_update();

    let content = scrollview.children()[0].children()[0].children();

    // Should still be at the start
    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "0 Hello, World!"
    );

    // Press ArrowDown multiple times
    for _ in 0..5 {
        test.press_key(Key::Named(NamedKey::ArrowDown));
    }

    let content = scrollview.children()[0].children()[0].children();

    // Should have scrolled down
    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "10 Hello, World!"
    );

    // Press ArrowUp to scroll back up
    for _ in 0..3 {
        test.press_key(Key::Named(NamedKey::ArrowUp));
    }

    let content = scrollview.children()[0].children()[0].children();

    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "4 Hello, World!"
    );

    // Press End to jump to bottom
    test.press_key(Key::Named(NamedKey::End));

    let content = scrollview.children()[0].children()[0].children();

    // At the end, should show last items
    assert_eq!(
        Label::try_downcast(&*content[content.len() - 1].element())
            .unwrap()
            .text,
        "29 Hello, World!"
    );

    // Press Home to jump to top
    test.press_key(Key::Named(NamedKey::Home));

    let content = scrollview.children()[0].children()[0].children();

    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "0 Hello, World!"
    );
}

#[test]
pub fn virtual_scroll_view_keyboard_navigation_horizontal() {
    fn virtual_scroll_view_horizontal_app() -> impl IntoElement {
        VirtualScrollView::new(|i, _| {
            label()
                .key(i)
                .width(Size::px(50.))
                .text(format!("{i}"))
                .into()
        })
        .length(30usize)
        .item_size(50.)
        .direction(Direction::Horizontal)
    }

    let mut test = launch_test(virtual_scroll_view_horizontal_app);
    test.sync_and_update();

    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    assert_eq!(content.len(), 11);

    // Check initial position
    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "0"
    );

    // Focus the scrollview by clicking and dragging on the scrollbar, then moving back
    test.move_cursor((20., 495.));
    test.sync_and_update();
    test.press_cursor((20., 495.));
    test.sync_and_update();
    test.move_cursor((25., 495.));
    test.sync_and_update();
    test.move_cursor((20., 495.));
    test.sync_and_update();
    test.release_cursor((20., 495.));
    test.sync_and_update();

    let content = scrollview.children()[0].children()[0].children();

    // Should still be at the start
    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "0"
    );

    // Press ArrowRight multiple times
    for _ in 0..5 {
        test.press_key(Key::Named(NamedKey::ArrowRight));
    }

    let content = scrollview.children()[0].children()[0].children();

    // Should have scrolled right
    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "10"
    );

    // Press ArrowLeft to scroll back
    for _ in 0..3 {
        test.press_key(Key::Named(NamedKey::ArrowLeft));
    }

    let content = scrollview.children()[0].children()[0].children();

    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "4"
    );

    // Press End to jump to the right
    test.press_key(Key::Named(NamedKey::End));

    let content = scrollview.children()[0].children()[0].children();

    assert_eq!(
        Label::try_downcast(&*content[content.len() - 1].element())
            .unwrap()
            .text,
        "29"
    );

    // Press Home to jump to the left
    test.press_key(Key::Named(NamedKey::Home));

    let content = scrollview.children()[0].children()[0].children();

    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "0"
    );
}
