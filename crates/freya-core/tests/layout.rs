use freya::prelude::*;
use freya_testing::prelude::*;

/// Regression: an inner-sized absolute rect inside a centered parent used to
/// panic in `recursive_translate` because descendants weren't cached during
/// the initial-phase pass.
#[test]
pub fn inner_sized_absolute_under_centered_parent_does_not_panic() {
    fn app() -> impl IntoElement {
        rect().expanded().center().child(
            rect().child(
                rect()
                    .position(Position::new_absolute().right(0.))
                    .child(rect().width(Size::px(50.)).height(Size::px(50.))),
            ),
        )
    }

    let test = launch_test(app);

    let leaves = test.find_many(|t, e| {
        Rect::try_downcast(e)
            .filter(|_| t.children().is_empty())
            .map(|_| t.layout())
    });
    assert!(
        leaves
            .iter()
            .any(|l| l.area.size.width == 50. && l.area.size.height == 50.),
        "expected the 50x50 leaf rect to be measured, got: {:?}",
        leaves.iter().map(|l| l.area.size).collect::<Vec<_>>()
    );
}
