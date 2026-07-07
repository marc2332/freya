use freya::{
    elements::image::Image,
    prelude::*,
};
use freya_testing::prelude::*;

#[test]
pub fn svg_viewer_rasterizes_and_renders() {
    fn app() -> impl IntoElement {
        SvgViewer::new(("ferris", include_bytes!("../../../examples/ferris.svg")))
            .width(Size::px(100.))
            .height(Size::px(100.))
            .parallel(true)
    }

    let mut test = launch_test(app);
    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(120),
    );
    test.sync_and_update();

    assert!(
        test.find(|node, element| Image::try_downcast(element).map(|_| node))
            .is_some(),
        "SVG should be rasterized into an image element"
    );
}

#[test]
pub fn svg_viewer_rasterizes_synchronously_by_default() {
    fn app() -> impl IntoElement {
        SvgViewer::new(("ferris", include_bytes!("../../../examples/ferris.svg")))
            .width(Size::px(100.))
            .height(Size::px(100.))
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    assert!(
        test.find(|_, element| Image::try_downcast(element))
            .is_some(),
        "SVG should be rasterized without polling async tasks"
    );
}

#[test]
pub fn svg_viewer_rasterizes_at_visible_size() {
    fn app() -> impl IntoElement {
        SvgViewer::new(("ferris", include_bytes!("../../../examples/ferris.svg")))
            .width(Size::px(100.))
            .height(Size::px(100.))
            .margin((0., 0., 0., 20.))
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    let dimensions = test.find(|_, element| {
        Image::try_downcast(element).map(|image| image.image_handle.image.dimensions())
    });
    // The raster must match the element's visible size, margins excluded.
    assert_eq!(dimensions.map(|d| (d.width, d.height)), Some((100, 100)));
}

#[test]
pub fn svg_viewer_custom_error_renderer() {
    fn app() -> impl IntoElement {
        SvgViewer::new(std::path::PathBuf::from("/non/existent.svg"))
            .width(Size::px(100.))
            .height(Size::px(100.))
            .error_renderer(|err: String| label().text(format!("svg-error: {err}")).into())
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    let error_label = test.find(|node, element| {
        Label::try_downcast(element)
            .filter(|label| label.text.as_ref().starts_with("svg-error:"))
            .map(|_| node)
    });
    assert!(
        error_label.is_some(),
        "custom error renderer should run when the SVG fails to load"
    );
}
