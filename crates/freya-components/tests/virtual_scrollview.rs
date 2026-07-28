use freya::prelude::*;
use freya_core::prelude::Label;
use freya_testing::prelude::*;

#[test]
pub fn virtual_scroll_view_wheel() {
    fn virtual_scroll_view_wheel_app() -> impl IntoElement {
        VirtualScrollView::new(|item, _| {
            label()
                .key(item.index)
                .height(Size::px(50.))
                .text(format!("{} Hello, World!", item.index))
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
        VirtualScrollView::new(|item, _| {
            label()
                .key(item.index)
                .height(Size::px(50.))
                .text(format!("{} Hello, World!", item.index))
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
pub fn virtual_scroll_view_controlled() {
    fn virtual_scroll_view_controlled_app() -> impl IntoElement {
        let scroll_controller = use_scroll_controller(ScrollConfig::default);

        rect()
            .horizontal()
            .content(Content::Flex)
            .child(
                VirtualScrollView::new_controlled(
                    |item, _| {
                        label()
                            .key(item.index)
                            .height(Size::px(50.))
                            .text(format!("{} Hello, World!", item.index))
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
                    |item, _| {
                        label()
                            .key(item.index)
                            .height(Size::px(50.))
                            .text(format!("{} Second View", item.index))
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
pub fn virtual_scroll_view_closure_item_size() {
    fn virtual_scroll_view_closure_app() -> impl IntoElement {
        VirtualScrollView::new(|item, _| {
            label()
                .key(item.index)
                .height(Size::px(item.size))
                .text(format!("{}:{}", item.index, item.size))
                .into()
        })
        .length(30usize)
        .item_size(|index: usize| if index.is_multiple_of(2) { 100. } else { 50. })
    }

    let mut test = launch_test(virtual_scroll_view_closure_app);
    test.sync_and_update();
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    // Heights accumulate as 100, 150, 250, 300, 400, 450, 550, so the 7th item
    // is the one that crosses the 500px viewport. The closure size also reaches the builder.
    assert_eq!(content.len(), 7);

    let expected = ["0:100", "1:50", "2:100", "3:50", "4:100", "5:50", "6:100"];
    for (child, text) in content.iter().zip(expected) {
        assert_eq!(Label::try_downcast(&*child.element()).unwrap().text, text);
    }

    // Scrolling 300 pixels lands on index 4, since 100 + 50 + 100 + 50 = 300.
    test.scroll((5., 5.), (0., -300.));

    let content = scrollview.children()[0].children()[0].children();
    assert_eq!(content.len(), 7);
    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "4:100"
    );
}

#[test]
pub fn virtual_scroll_view_closure_item_size_horizontal() {
    fn virtual_scroll_view_closure_horizontal_app() -> impl IntoElement {
        VirtualScrollView::new(|item, _| {
            label()
                .key(item.index)
                .width(Size::px(item.size))
                .text(format!("{}:{}", item.index, item.size))
                .into()
        })
        .length(30usize)
        .item_size(|index: usize| if index.is_multiple_of(2) { 120. } else { 60. })
        .direction(Direction::Horizontal)
    }

    let mut test = launch_test(virtual_scroll_view_closure_horizontal_app);
    test.sync_and_update();
    let scrollview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
                .map(move |_| node)
        })
        .unwrap();
    let content = scrollview.children()[0].children()[0].children();

    // Widths accumulate as 120, 180, 300, 360, 480, 540, so 6 items cover the 500px viewport.
    assert_eq!(content.len(), 6);

    let expected = ["0:120", "1:60", "2:120", "3:60", "4:120", "5:60"];
    for (child, text) in content.iter().zip(expected) {
        assert_eq!(Label::try_downcast(&*child.element()).unwrap().text, text);
    }
}

#[test]
pub fn virtual_scroll_view_keyboard_navigation() {
    fn virtual_scroll_view_keyboard_app() -> impl IntoElement {
        VirtualScrollView::new(|item, _| {
            label()
                .key(item.index)
                .height(Size::px(50.))
                .text(format!("{} Hello, World!", item.index))
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
        VirtualScrollView::new(|item, _| {
            label()
                .key(item.index)
                .width(Size::px(50.))
                .text(format!("{}", item.index))
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
