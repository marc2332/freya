use std::cell::RefCell;

use freya::prelude::*;
use freya_core::elements::svg::SvgRender;
use freya_testing::prelude::*;
use torin::prelude::LayoutNode;

const LOGO: &[u8] = include_bytes!("../../../logo.svg");

/// Identity of the raster cached on `layout_node`, or `None` if not yet rasterized.
fn cached_raster_id(layout_node: &LayoutNode) -> Option<u32> {
    layout_node
        .data
        .as_ref()?
        .downcast_ref::<RefCell<SvgRender>>()?
        .borrow()
        .raster
        .as_ref()
        .map(|(_, image)| image.unique_id())
}

/// Identity of the cached raster for the SVG node, or `None` if not yet rasterized.
fn cached_image_id(test: &TestingRunner) -> Option<u32> {
    test.find(|node, element| Svg::try_downcast(element).map(|_| cached_raster_id(&node.layout())))
        .flatten()
}

#[test]
fn svg_is_rasterized_and_cached_once() {
    fn app() -> impl IntoElement {
        svg(LOGO).width(Size::px(24.)).height(Size::px(24.))
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    // Nothing is rasterized until the node is painted.
    assert_eq!(cached_image_id(&test), None);

    test.render();
    let first = cached_image_id(&test).expect("SVG should be cached after the first render");

    // Painting again with the same params reuses the cached image.
    test.render();
    let second = cached_image_id(&test).expect("cached image should still be present");
    assert_eq!(
        first, second,
        "SVG was re-rasterized instead of reusing the cache"
    );
}

#[test]
fn svg_raster_is_invalidated_when_color_changes() {
    fn app() -> impl IntoElement {
        let color = use_consume::<State<Color>>();
        svg(LOGO)
            .width(Size::px(24.))
            .height(Size::px(24.))
            .color(color())
    }

    let (mut test, mut color) = TestingRunner::new(
        app,
        (100., 100.).into(),
        |runner| runner.provide_root_context(|| State::create(Color::BLACK)),
        1.0,
    );

    test.render();
    let before = cached_image_id(&test).expect("SVG should be cached after the first render");

    color.set(Color::RED);
    test.sync_and_update();
    test.render();
    let after = cached_image_id(&test).expect("SVG should be cached after the color change");

    assert_ne!(
        before, after,
        "SVG should be re-rasterized when a render param changes"
    );
}
