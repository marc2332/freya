use std::collections::HashMap;

use freya::prelude::*;
use freya_core::integration::AppComponent;
use freya_testing::prelude::*;

fn launch(app: impl Into<AppComponent>) -> TestingRunner {
    let mut test = launch_test(app);
    test.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("../../freya-edit/tests/NotoSans-Regular.ttf").as_slice(),
    )]));
    test.set_default_fonts(&["NotoSans".into()]);
    test.sync_and_update();
    test
}

fn find_label_area(test: &TestingRunner, text: &str) -> Option<Area> {
    test.find(|node, element| {
        Label::try_downcast(element)
            .filter(|label| label.text.as_ref() == text)
            .map(|_| node.layout().area)
    })
}

#[test]
fn inline_element_in_text_receives_events() {
    fn app() -> impl IntoElement {
        let mut clicked = use_state(|| false);

        paragraph()
            .span("Some text before the element ")
            .child(
                rect()
                    .width(Size::px(40.))
                    .height(Size::px(20.))
                    .background(Color::BLUE)
                    .on_press(move |_| clicked.toggle())
                    .child(label().text(clicked.read().to_string())),
            )
            .span(" and some text after it.")
    }

    let mut test = launch(app);

    let area = find_label_area(&test, "false").expect("inline element should be in the tree");
    // Placed onto its placeholder in the middle of the text, not piled at the origin.
    assert!(area.min_x() > 0.0);

    test.click_cursor((
        (area.min_x() + area.width() / 2.0) as f64,
        (area.min_y() + area.height() / 2.0) as f64,
    ));

    assert!(
        find_label_area(&test, "true").is_some(),
        "inline element did not receive the click"
    );
}

#[test]
fn inline_element_in_fixed_size_paragraph_keeps_position_after_child_change() {
    fn app() -> impl IntoElement {
        let mut expanded = use_state(|| false);

        paragraph()
            .width(Size::px(400.))
            .height(Size::px(100.))
            .span("Some text before ")
            .child(
                rect()
                    .width(Size::px(if *expanded.read() { 80. } else { 40. }))
                    .height(Size::px(20.))
                    .on_press(move |_| expanded.toggle())
                    .child(label().text(expanded.read().to_string())),
            )
            .span(" and some text after.")
    }

    let mut test = launch(app);

    let area_before =
        find_label_area(&test, "false").expect("inline element should be in the tree");
    assert!(area_before.min_x() > 0.0);

    test.click_cursor((
        (area_before.min_x() + 1.0) as f64,
        (area_before.min_y() + 1.0) as f64,
    ));

    // The paragraph has a fixed size but must still relayout around the resized child.
    let area_after = find_label_area(&test, "true").expect("inline element should be in the tree");
    assert!(
        (area_after.min_x() - area_before.min_x()).abs() < 1.0,
        "inline element moved from {} to {}",
        area_before.min_x(),
        area_after.min_x()
    );
}

#[test]
fn paragraph_with_only_inline_elements_is_sized_by_them() {
    fn app() -> impl IntoElement {
        paragraph().child(
            rect()
                .width(Size::px(40.))
                .height(Size::px(20.))
                .child(label().text("inline")),
        )
    }

    let test = launch(app);

    let paragraph_area = test
        .find(|node, element| Paragraph::try_downcast(element).map(|_| node.layout().area))
        .expect("paragraph should be in the tree");
    // With no spans, the placeholder alone defines the paragraph size.
    assert!(paragraph_area.width() >= 39.5);
    assert!(paragraph_area.height() >= 19.5);

    let label_area =
        find_label_area(&test, "inline").expect("inline element should be in the tree");
    assert!(label_area.min_x() >= paragraph_area.min_x() - 0.5);
    assert!(label_area.min_y() >= paragraph_area.min_y() - 0.5);
}

#[test]
fn inline_element_beyond_max_lines_is_hidden() {
    fn app() -> impl IntoElement {
        let mut clicked = use_state(|| false);

        paragraph().max_lines(1usize).span("First line\n").child(
            rect()
                .width(Size::px(40.))
                .height(Size::px(20.))
                .on_press(move |_| clicked.toggle())
                .child(label().text(clicked.read().to_string())),
        )
    }

    let mut test = launch(app);

    let layout = test
        .find(|node, element| {
            Rect::try_downcast(element).and_then(|_| {
                let layout = node.layout();
                (layout.area.width() == 40.0).then_some(layout)
            })
        })
        .expect("inline element should be in the tree");
    // Its placeholder falls on a line cut off by max_lines, so the whole subtree is hidden.
    assert!(layout.hidden);

    test.click_cursor((
        (layout.area.min_x() + 1.0) as f64,
        (layout.area.min_y() + 1.0) as f64,
    ));

    assert!(
        find_label_area(&test, "true").is_none(),
        "hidden inline element received a click"
    );
}

#[test]
fn inline_element_respects_paragraph_margin() {
    fn app() -> impl IntoElement {
        paragraph().margin(20.).span("Before ").child(
            rect()
                .width(Size::px(40.))
                .height(Size::px(20.))
                .child(label().text("inline")),
        )
    }

    let test = launch(app);

    let label_area =
        find_label_area(&test, "inline").expect("inline element should be in the tree");
    // Children are placed relative to the visible area, which excludes the margin.
    assert!(label_area.min_x() > 20.0);
    assert!(label_area.min_y() >= 19.5);
}
