use freya::prelude::*;
use freya_testing::prelude::*;

fn lazy_app(keep_rendered: bool) -> impl IntoElement {
    ScrollView::new()
        .child(rect().height(Size::px(600.)))
        .child(
            Lazy::new()
                .keep_rendered(keep_rendered)
                .height(Size::px(200.))
                .width(Size::px(200.))
                .child("Lazy content"),
        )
}

fn is_rendered(test: &TestingRunner) -> bool {
    test.find(|_, element| Label::try_downcast(element))
        .is_some()
}

#[test]
pub fn lazy_renders_once_visible() {
    let mut test = launch_test(|| lazy_app(true));
    test.sync_and_update();
    assert!(!is_rendered(&test));

    test.scroll((250., 250.), (0., -300.));
    test.sync_and_update();
    assert!(is_rendered(&test));

    test.scroll((250., 250.), (0., 300.));
    test.sync_and_update();
    assert!(is_rendered(&test));
}

#[test]
pub fn lazy_unrenders_when_hidden() {
    let mut test = launch_test(|| lazy_app(false));
    test.sync_and_update();
    assert!(!is_rendered(&test));

    test.scroll((250., 250.), (0., -300.));
    test.sync_and_update();
    assert!(is_rendered(&test));

    test.scroll((250., 250.), (0., 300.));
    test.sync_and_update();
    assert!(!is_rendered(&test));
}
