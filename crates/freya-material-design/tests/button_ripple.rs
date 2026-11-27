use std::{
    cell::Cell,
    rc::Rc,
    time::Duration,
};

use freya::{
    engine::prelude::SkImage,
    material_design::RippleExt,
    prelude::*,
};
use freya_testing::{
    TestingNode,
    TestingRunner,
    launch_test,
};

fn button() -> Button {
    Button::new()
        .width(Size::px(100.))
        .height(Size::px(60.))
        .padding((18., 28., 6., 8.))
        .corner_radius(24.)
        .background(Color::WHITE)
        .hover_background(Color::WHITE)
        .border_fill(Color::TRANSPARENT)
        .child("Before")
}

fn button_node(test: &TestingRunner) -> TestingNode {
    test.find(|node, element| {
        (element.accessibility().builder.role() == AccessibilityRole::Button).then_some(node)
    })
    .unwrap()
}

#[test]
fn ripple_keeps_button_layout_and_children() {
    let plain = launch_test(|| button().child("After"));
    let ripple = launch_test(|| button().ripple().child("After"));
    let plain_button = button_node(&plain);
    let ripple_button = button_node(&ripple);

    assert_eq!(plain_button.layout(), ripple_button.layout());
    let plain_children = plain_button.children();
    let ripple_children = ripple_button.children();
    assert_eq!(ripple_children.len(), 2);
    for (plain, ripple) in plain_children.iter().zip(&ripple_children) {
        assert_eq!(plain.layout(), ripple.layout());
    }
}

#[test]
fn ripple_covers_button_padding_without_painting_outside_rounded_corners() {
    let pointer_downs = Rc::new(Cell::new(0));
    let presses = Rc::new(Cell::new(0));
    let mut test = launch_test({
        let pointer_downs = pointer_downs.clone();
        let presses = presses.clone();
        move || {
            let pointer_downs = pointer_downs.clone();
            let presses = presses.clone();
            rect().background(Color::GREEN).child(
                button()
                    .on_pointer_down(move |_| pointer_downs.set(pointer_downs.get() + 1))
                    .on_press(move |_| presses.set(presses.get() + 1))
                    .ripple()
                    .color(Color::RED),
            )
        }
    });
    let button = button_node(&test);
    let area = button.layout().visible_area();
    let point = |x: i32, y: i32| (area.min_x() as i32 + x, area.min_y() as i32 + y);
    let press = point(30, 12);
    test.animation_clock().set_speed(0.25);
    test.press_cursor((f64::from(press.0), f64::from(press.1)));
    test.poll(Duration::from_millis(5), Duration::from_millis(200));

    let image = SkImage::from_encoded(test.render())
        .and_then(|image| image.make_raster_image(None, None))
        .unwrap();
    let pixels = image.peek_pixels().unwrap();
    let inside = pixels.get_color(point(30, 2));
    assert!(
        inside.r() > inside.g(),
        "expected red ripple in top padding"
    );
    assert_eq!(Color::from(pixels.get_color(point(1, 1))), Color::GREEN);
    assert_eq!(button.layout().visible_area(), area);

    test.release_cursor((f64::from(press.0), f64::from(press.1)));
    assert_eq!(pointer_downs.get(), 1);
    assert_eq!(presses.get(), 1);
}

#[test]
fn button_pointer_handler_can_skip_the_ripple() {
    let mut test = launch_test(|| {
        button()
            .on_pointer_down(|event: Event<PointerEventData>| event.stop_stack())
            .ripple()
    });
    let button = button_node(&test);
    let area = button.layout().visible_area();
    test.press_cursor((f64::from(area.min_x()) + 50., f64::from(area.min_y()) + 30.));
    assert_eq!(button.children().len(), 1);
}

#[test]
fn custom_component_clips_the_ripple_without_clipping_its_children() {
    #[derive(PartialEq)]
    struct Surface;

    impl ComponentContent for Surface {
        type Content = Rect;

        fn render_content(&self) -> Self::Content {
            rect()
                .width(Size::px(100.))
                .height(Size::px(60.))
                .corner_radius(24.)
                .background(Color::WHITE)
                .child(
                    rect()
                        .position(Position::new_absolute().left(0.).top(0.))
                        .width(Size::px(5.))
                        .height(Size::px(5.))
                        .background(Color::BLUE),
                )
        }
    }

    impl Component for Surface {
        fn render(&self) -> impl IntoElement {
            self.render_content()
        }
    }

    let mut test = launch_test(|| {
        rect()
            .background(Color::GREEN)
            .child(Surface.ripple().color(Color::RED))
    });
    test.animation_clock().set_speed(0.25);
    test.press_cursor((30., 12.));
    test.poll(Duration::from_millis(5), Duration::from_millis(200));

    let image = SkImage::from_encoded(test.render())
        .and_then(|image| image.make_raster_image(None, None))
        .unwrap();
    let pixels = image.peek_pixels().unwrap();
    assert!(pixels.get_color((30, 2)).r() > pixels.get_color((30, 2)).g());
    assert_eq!(Color::from(pixels.get_color((1, 1))), Color::BLUE);
    assert_eq!(Color::from(pixels.get_color((7, 1))), Color::GREEN);
}
