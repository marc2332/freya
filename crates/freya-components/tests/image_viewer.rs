use std::rc::Rc;

use freya::{
    elements::image::Image,
    prelude::*,
};
use freya_components::cache::{
    Asset,
    AssetAge,
    AssetCacher,
    AssetConfiguration,
    use_asset,
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
        std::time::Duration::from_millis(200),
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
    test.click_cursor(button_area.center().to_f64());
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
        std::time::Duration::from_millis(200),
    );
    test.sync_and_update();

    let image_element = test.find(|node, element| Image::try_downcast(element).map(|_| node));
    assert!(
        image_element.is_some(),
        "Image element should be rendered after new source loads"
    );
}

#[test]
pub fn image_viewer_asset_age_zero_clears_cache_on_unmount() {
    fn app() -> impl IntoElement {
        let mut show = use_state(|| true);
        let cacher = use_hook(AssetCacher::get);

        rect()
            .child(format!("size:{}", cacher.size()))
            .child(
                Button::new()
                    .on_press(move |_| *show.write() = false)
                    .child("hide"),
            )
            .maybe(show(), |r| {
                r.child(
                    ImageViewer::new((
                        "rust-logo-zero-age",
                        include_bytes!("../../../examples/rust_logo.png"),
                    ))
                    .asset_age(AssetAge::zero())
                    .width(Size::px(300.))
                    .height(Size::px(300.)),
                )
            })
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(200),
    );
    test.sync_and_update();

    let read_size = |test: &mut TestingRunner| {
        test.poll(
            std::time::Duration::from_millis(1),
            std::time::Duration::from_millis(50),
        );
        test.sync_and_update();
        test.find_many(|node, element| Label::try_downcast(element).map(|_| node))
            .into_iter()
            .find_map(|l| {
                Label::try_downcast(&*l.element())
                    .unwrap()
                    .text
                    .strip_prefix("size:")
                    .map(|s| s.to_string())
            })
            .unwrap()
    };

    assert_eq!(read_size(&mut test), "1");

    test.click_cursor((20.0, 30.0));
    test.sync_and_update();

    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(200),
    );
    test.sync_and_update();

    assert_eq!(read_size(&mut test), "0");
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
        std::time::Duration::from_millis(200),
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
        std::time::Duration::from_millis(200),
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

#[test]
pub fn asset_load_completing_after_unmount_does_not_leak() {
    #[derive(PartialEq)]
    struct Tile {
        config: AssetConfiguration,
    }

    impl Component for Tile {
        fn render(&self) -> impl IntoElement {
            let _asset = use_asset(&self.config);
            rect()
        }
    }

    fn app() -> impl IntoElement {
        let mut show = use_state(|| true);
        let cacher = use_hook(AssetCacher::get);
        let config = AssetConfiguration::new("wallpaper-tile", AssetAge::zero());
        let load_config = config.clone();

        rect()
            .child(format!("size:{}", cacher.size()))
            .child(
                Button::new()
                    .on_press(move |_| *show.write() = false)
                    .child("leave"),
            )
            .child(
                Button::new()
                    .on_press(move |_| {
                        let mut cacher = cacher;
                        cacher.update_asset(load_config.clone(), Asset::Cached(Rc::new(())));
                    })
                    .child("late-load"),
            )
            .maybe(show(), move |r| r.child(Tile { config }))
    }

    let read_size = |test: &mut TestingRunner| {
        test.poll(
            std::time::Duration::from_millis(1),
            std::time::Duration::from_millis(50),
        );
        test.sync_and_update();
        test.find_many(|node, element| Label::try_downcast(element).map(|_| node))
            .into_iter()
            .find_map(|l| {
                Label::try_downcast(&*l.element())
                    .unwrap()
                    .text
                    .strip_prefix("size:")
                    .map(|s| s.to_string())
            })
            .unwrap()
    };

    let click_label = |test: &mut TestingRunner, text: &str| {
        let button = test
            .find(|node, element| {
                Label::try_downcast(element)
                    .filter(|label| label.text.as_ref() == text)
                    .map(|_| node)
            })
            .unwrap();
        let area = button.layout().area;
        test.click_cursor(area.center().to_f64());
    };

    let mut test = launch_test(app);
    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(200),
    );
    test.sync_and_update();

    assert_eq!(
        read_size(&mut test),
        "1",
        "the tile's asset should be cached while it is mounted"
    );

    click_label(&mut test, "leave");
    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(200),
    );
    test.sync_and_update();

    assert_eq!(
        read_size(&mut test),
        "0",
        "the asset should be evicted once the tile is gone"
    );

    click_label(&mut test, "late-load");
    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(200),
    );
    test.sync_and_update();

    assert_eq!(
        read_size(&mut test),
        "0",
        "a load that completes after eviction must not resurrect the cache entry"
    );
}

#[test]
pub fn image_viewer_source_change_clears_previous_asset() {
    fn app() -> impl IntoElement {
        let mut index = use_state(|| 0usize);
        let cacher = use_hook(AssetCacher::get);

        let sources: [ImageSource; 2] = [
            ("logo-a", include_bytes!("../../../examples/rust_logo.png")).into(),
            ("logo-b", include_bytes!("../../../examples/rust_logo.png")).into(),
        ];

        rect()
            .child(format!("size:{}", cacher.size()))
            .child(
                ImageViewer::new(sources[index()].clone())
                    .asset_age(AssetAge::zero())
                    .width(Size::px(300.))
                    .height(Size::px(300.)),
            )
            .child(
                Button::new()
                    .on_press(move |_| *index.write() = (index() + 1) % 2)
                    .child("switch"),
            )
    }

    let read_size = |test: &mut TestingRunner| {
        test.poll(
            std::time::Duration::from_millis(1),
            std::time::Duration::from_millis(50),
        );
        test.sync_and_update();
        test.find_many(|node, element| Label::try_downcast(element).map(|_| node))
            .into_iter()
            .find_map(|l| {
                Label::try_downcast(&*l.element())
                    .unwrap()
                    .text
                    .strip_prefix("size:")
                    .map(|s| s.to_string())
            })
            .unwrap()
    };

    let mut test = launch_test(app);
    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(200),
    );
    test.sync_and_update();

    assert_eq!(
        read_size(&mut test),
        "1",
        "only the first source should be cached"
    );

    let button = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|label| label.text.as_ref() == "switch")
                .map(|_| node)
        })
        .unwrap();
    let button_area = button.layout().area;
    test.click_cursor(button_area.center().to_f64());
    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(1),
        std::time::Duration::from_millis(200),
    );
    test.sync_and_update();

    assert_eq!(
        read_size(&mut test),
        "1",
        "the previous source should be evicted after switching, leaving only the new one"
    );
}
