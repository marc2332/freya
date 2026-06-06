use std::collections::HashMap;

use freya::prelude::*;
use freya_testing::prelude::*;

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

    let mut test = launch_test(app);
    test.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("../../freya-edit/tests/NotoSans-Regular.ttf").as_slice(),
    )]));
    test.set_default_fonts(&["NotoSans".into()]);
    test.sync_and_update();

    let area = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|label| label.text.as_ref() == "false")
                .map(|_| node.layout().area)
        })
        .expect("inline element should be in the tree");
    // Placed onto its placeholder in the middle of the text, not piled at the origin.
    assert!(area.min_x() > 0.0);

    test.click_cursor((
        (area.min_x() + area.width() / 2.0) as f64,
        (area.min_y() + area.height() / 2.0) as f64,
    ));

    assert!(
        test.find(|_, element| {
            Label::try_downcast(element).filter(|label| label.text.as_ref() == "true")
        })
        .is_some(),
        "inline element did not receive the click"
    );
}
