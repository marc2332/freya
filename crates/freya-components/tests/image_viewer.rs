use freya::{
    elements::image::Image,
    prelude::*,
};
use freya_testing::prelude::*;

#[test]
pub fn image_viewer_source_change() {
    fn image_viewer_app() -> impl IntoElement {
        let mut index = use_state(|| 0);

        let sources: [ImageSource; 2] = [
            ("logo-a", include_bytes!("../../../examples/rust_logo.png")).into(),
            ("logo-b", include_bytes!("../../../examples/rust_logo.png")).into(),
        ];

        rect()
            .child(
                ImageViewer::new(sources[index()].clone())
                    .width(Size::px(300.))
                    .height(Size::px(300.)),
            )
            .child(
                Button::new()
                    .on_press(move |_| *index.write() = (index() + 1) % sources.len())
                    .child("Switch"),
            )
    }

    let mut test = launch_test(image_viewer_app);
    test.sync_and_update();

    // Wait for the first image to load
    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(70),
    );
    test.sync_and_update();

    let image_element = test.find(|node, element| Image::try_downcast(element).map(|_| node));
    assert!(
        image_element.is_some(),
        "Image element should be rendered after initial load"
    );

    // Click the button to change the source
    let button = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|label| label.text.as_ref() == "Switch")
                .map(|_| node)
        })
        .unwrap();
    let button_area = button.layout().area;
    test.click_cursor((
        button_area.min_x() as f64 + button_area.size.width as f64 / 2.0,
        button_area.min_y() as f64 + button_area.size.height as f64 / 2.0,
    ));
    test.sync_and_update();

    // The new source should be loading, showing the loader again
    let loader_rect = test.find(|node, element| {
        Rect::try_downcast(element)
            .filter(|rect| rect.layout.main_alignment == Alignment::Center)
            .map(|_| node)
    });
    assert!(
        loader_rect.is_some(),
        "Should show loading indicator after source change"
    );

    // Wait for the new image to load
    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(70),
    );
    test.sync_and_update();

    let image_element = test.find(|node, element| Image::try_downcast(element).map(|_| node));
    assert!(
        image_element.is_some(),
        "Image element should be rendered after new source loads"
    );
}

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

#[test]
pub fn image_viewer_custom_error_renderer() {
    fn image_viewer_app() -> impl IntoElement {
        let source: ImageSource = std::path::PathBuf::from("/non/existent/image.png").into();

        ImageViewer::new(source)
            .width(Size::px(300.))
            .height(Size::px(300.))
            .error_renderer(|err: String| label().text(format!("custom-error: {err}")).into())
    }

    let mut test = launch_test(image_viewer_app);
    test.sync_and_update();

    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(70),
    );
    test.sync_and_update();

    let custom_label = test.find(|node, element| {
        Label::try_downcast(element)
            .filter(|label| label.text.as_ref().starts_with("custom-error:"))
            .map(|_| node)
    });

    assert!(
        custom_label.is_some(),
        "Custom error renderer should be invoked when the image fails to load"
    );
}
