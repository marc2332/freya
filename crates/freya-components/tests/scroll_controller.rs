use freya::prelude::*;
use freya_core::prelude::Label;
use freya_testing::{
    TestingNode,
    prelude::*,
};

fn reported_position(test: &TestingRunner) -> String {
    test.find(|node, element| {
        Label::try_downcast(element)
            .filter(|label| label.text.starts_with("position"))
            .map(move |_| node)
    })
    .map(|node| {
        Label::try_downcast(&*node.element())
            .unwrap()
            .text
            .to_string()
    })
    .unwrap()
}

fn scrollview(test: &mut TestingRunner) -> TestingNode {
    test.find(|node, element| {
        Rect::try_downcast(element)
            .filter(|rect| rect.accessibility.builder.role() == AccessibilityRole::ScrollView)
            .map(move |_| node)
    })
    .unwrap()
}

fn end_config() -> ScrollConfig {
    ScrollConfig {
        default_vertical_position: ScrollPosition::End,
        ..Default::default()
    }
}

fn position_label(scroll_controller: ScrollController) -> impl IntoElement {
    let (_, scrolled_y): (i32, i32) = scroll_controller.into();
    label()
        .height(Size::px(0.))
        .text(format!("position {scrolled_y}"))
}

#[test]
pub fn virtual_scroll_view_starts_at_the_end() {
    fn app() -> impl IntoElement {
        let scroll_controller = use_scroll_controller(end_config);

        rect().child(position_label(scroll_controller)).child(
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
            .item_size(50.),
        )
    }

    let mut test = launch_test(app);
    test.sync_and_update();
    test.sync_and_update();

    let scrollview = scrollview(&mut test);
    let content = scrollview.children()[0].children()[0].children();

    assert_eq!(
        Label::try_downcast(&*content[0].element()).unwrap().text,
        "20 Hello, World!"
    );
    assert_eq!(reported_position(&test), "position -1000");
}

#[test]
pub fn controlled_scroll_view_clamps_programmatic_scrolls() {
    fn app() -> impl IntoElement {
        let mut scroll_controller = use_scroll_controller(ScrollConfig::default);
        let scroll_to_end = move |_| {
            scroll_controller.scroll_to_y(-10_000);
        };

        rect()
            .child(position_label(scroll_controller))
            .child(Button::new().on_press(scroll_to_end).child("Scroll"))
            .child(
                ScrollView::new_controlled(scroll_controller).children((0..30).map(|i| {
                    label()
                        .key(i)
                        .height(Size::px(50.))
                        .text(format!("{i} Hello, World!"))
                })),
            )
    }

    let mut test = launch_test(app);
    test.sync_and_update();
    test.click_cursor((10., 10.));
    test.sync_and_update();

    assert_eq!(reported_position(&test), "position -1030");
}

#[test]
pub fn scroll_view_starts_at_the_end() {
    fn app() -> impl IntoElement {
        let scroll_controller = use_scroll_controller(end_config);

        rect().child(position_label(scroll_controller)).child(
            ScrollView::new_controlled(scroll_controller).children((0..30).map(|i| {
                label()
                    .key(i)
                    .height(Size::px(50.))
                    .text(format!("{i} Hello, World!"))
            })),
        )
    }

    let mut test = launch_test(app);
    test.sync_and_update();
    test.sync_and_update();

    assert_eq!(reported_position(&test), "position -1000");
}
