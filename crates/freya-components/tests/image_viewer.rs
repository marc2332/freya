use freya::{
    elements::image::Image,
    prelude::*,
};
use freya_testing::prelude::*;

#[test]
pub fn image_viewer_load_and_render() {
    fn image_viewer_app() -> impl IntoElement {
        let source: ImageSource = (
            "rust-logo",
            include_bytes!("../../../examples/rust_logo.png"),
        )
            .into();

        ImageViewer::new(source)
            .width(Size::px(300.))
            .height(Size::px(300.))
    }

    let mut test = launch_test(image_viewer_app);
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

    // Wait a bit for the image to load and render
    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(70),
    );
    test.sync_and_update();

    // After loading, the Image element should be rendered
    let image_element = test.find(|node, element| Image::try_downcast(element).map(|_| node));

    assert!(
        image_element.is_some(),
        "Image element should be rendered after loading"
    );
}
