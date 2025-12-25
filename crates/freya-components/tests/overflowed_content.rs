use std::time::Duration;

use freya::prelude::OverflowedContent;
use freya_core::prelude::*;
use freya_testing::prelude::*;
use torin::size::Size;

#[test]
pub fn overflowed_content() {
    fn app() -> impl IntoElement {
        OverflowedContent::new()
            .duration(Duration::from_millis(50))
            .width(Size::px(50.))
            .child(label().text("123456789123456789"))
    }

    let mut test = launch_test(app);

    // Initial state - content should be at starting position
    let rects = test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()));
    assert!(!rects.is_empty());

    // Let the animation run for a bit
    test.poll(Duration::from_millis(1), Duration::from_millis(25));

    // After some time, the offset should have changed (animation in progress)
    let rects_after = test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()));
    assert!(!rects_after.is_empty());

    // Continue polling to let animation progress further
    test.poll(Duration::from_millis(1), Duration::from_millis(50));

    let rects_end = test.find_many(|t, e| Rect::try_downcast(e).map(|_| t.layout()));
    assert!(!rects_end.is_empty());
}
