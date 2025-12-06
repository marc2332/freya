use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
pub fn basic_render() {
    fn app() -> impl IntoElement {
        rect()
            .background((210, 210, 210))
            .expanded()
            .center()
            .child("Hello, World!")
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    let data = test.render();

    assert!(!data.is_empty());
}
