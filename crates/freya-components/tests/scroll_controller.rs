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
fn follower_views_preserve_controller_bounds() {
    for virtual_follower in [false, true] {
        let (mut test, mut controller) = TestingRunner::new(
            move || {
                let controller = use_consume::<ScrollController>();
                rect()
                    .horizontal()
                    .child(
                        ScrollView::new()
                            .scroll_controller(controller)
                            .width(Size::px(400.))
                            .show_scrollbar(false)
                            .child(rect().width(Size::px(2000.)).height(Size::px(2000.))),
                    )
                    .child(if virtual_follower {
                        VirtualScrollView::new(|item, _| {
                            rect().key(item.index).height(Size::px(item.size)).into()
                        })
                        .length(1usize)
                        .item_size(50.)
                        .width(Size::px(100.))
                        .show_scrollbar(false)
                        .follow_scroll_controller(controller)
                        .into_element()
                    } else {
                        ScrollView::new()
                            .width(Size::px(100.))
                            .show_scrollbar(false)
                            .follow_scroll_controller(controller)
                            .child(rect().height(Size::px(50.)))
                            .into_element()
                    })
            },
            (500., 500.).into(),
            |runner| runner.provide_root_context(|| ScrollController::new(0, 0)),
            1.,
        );
        test.sync_and_update();
        let bounds = *controller.bounds.peek();
        assert_eq!(bounds, Some(((2000., 2000.).into(), (400., 500.).into())));

        controller.scroll_to_x(-200);
        controller.scroll_to_y(-300);
        test.sync_and_update();
        assert_eq!(controller.position(), (-200., -300.).into());
        assert_eq!(*controller.bounds.peek(), bounds);

        test.scroll((450., 100.), (0., -100.));
        assert_eq!(controller.position(), (-200., -400.).into());
        assert_eq!(*controller.bounds.peek(), bounds);

        test.scroll((450., 100.), (-100., 0.));
        assert_eq!(controller.position(), (-300., -400.).into());
        assert_eq!(*controller.bounds.peek(), bounds);
    }
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

    assert_ne!(reported_position(&test), "position -10000");
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
