use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
pub fn gif_viewer_load_and_render() {
    fn gif_viewer_app() -> impl IntoElement {
        let source: GifSource = (
            "test-gif",
            include_bytes!("../../../examples/frog_typing.gif"),
        )
            .into();

        GifViewer::new(source)
            .width(Size::px(300.))
            .height(Size::px(300.))
    }

    let mut test = launch_test(gif_viewer_app);
    test.sync_and_update();

    // Initially should show a loading indicator (CircularLoader)
    let loader_rect = test.find(|node, element| {
        Rect::try_downcast(element)
            .filter(|rect| rect.layout.main_alignment == Alignment::Center)
            .map(|_| node)
    });

    assert!(
        loader_rect.is_some(),
        "Should show loading indicator initially"
    );

    // Wait a bit for the GIF to load and render
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(800),
    );
    test.sync_and_update();

    for _ in 0..25 {
        // After loading, the GIF element should be rendered
        let gif_element = test.find(|node, element| Gif::try_downcast(element).map(|_| node));
        if gif_element.is_some() {
            return;
        }
        test.poll(
            std::time::Duration::from_millis(10),
            std::time::Duration::from_millis(200),
        );
        test.sync_and_update();
    }

    panic!(
        "GIF element should be rendered after loading"
    );
}
