use freya::prelude::*;
use freya_testing::prelude::*;

fn lazy_app(keep_rendered: bool) -> impl IntoElement {
    ScrollView::new()
        .child(rect().height(Size::px(600.)).width(Size::px(200.)))
        .child(
            Lazy::new()
                .keep_rendered(keep_rendered)
                .height(Size::px(200.))
                .width(Size::px(200.))
                .child("Lazy content"),
        )
}

fn is_content_rendered(test: &TestingRunner) -> bool {
    test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "Lazy content")
    })
    .is_some()
}

#[test]
pub fn lazy_renders_once_visible() {
    let mut test = launch_test(|| lazy_app(true));
    test.sync_and_update();

    // The Lazy sits at 600..800, below the 500px tall viewport
    assert!(!is_content_rendered(&test));

    // Scrolling it into view renders the children
    test.scroll((250., 250.), (0., -300.));
    test.sync_and_update();
    assert!(is_content_rendered(&test));

    // Scrolling it back out keeps them rendered
    test.scroll((250., 250.), (0., 300.));
    test.sync_and_update();
    assert!(is_content_rendered(&test));
}

#[test]
pub fn lazy_unrenders_when_hidden() {
    let mut test = launch_test(|| lazy_app(false));
    test.sync_and_update();

    assert!(!is_content_rendered(&test));

    test.scroll((250., 250.), (0., -300.));
    test.sync_and_update();
    assert!(is_content_rendered(&test));

    // Scrolling it back out unrenders the children
    test.scroll((250., 250.), (0., 300.));
    test.sync_and_update();
    assert!(!is_content_rendered(&test));

    // And they are rendered again once it comes back into view
    test.scroll((250., 250.), (0., -300.));
    test.sync_and_update();
    assert!(is_content_rendered(&test));
}
