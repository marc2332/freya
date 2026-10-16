use freya::prelude::*;
use freya_testing::prelude::*;
use torin::prelude::Direction;

#[test]
pub fn resizable_container_basic() {
    fn resizable_container_app() -> impl IntoElement {
        ResizableContainer::new()
            .panel(ResizablePanel::new(PanelSize::percent(50.)).child(label().text("Panel 1")))
            .panel(ResizablePanel::new(PanelSize::percent(50.)).child(label().text("Panel 2")))
    }

    let mut test = launch_test(resizable_container_app);
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));

    assert_eq!(labels.len(), 2);
    assert_eq!(
        Label::try_downcast(&*labels[0].element()).unwrap().text,
        "Panel 1"
    );
    assert_eq!(
        Label::try_downcast(&*labels[1].element()).unwrap().text,
        "Panel 2"
    );
}

#[test]
pub fn resizable_container_layout() {
    fn resizable_container_layout_app() -> impl IntoElement {
        ResizableContainer::new()
            .width(Size::px(300.))
            .height(Size::px(200.))
            .panel(
                ResizablePanel::new(PanelSize::percent(50.)).child(label().expanded().text("Top")),
            )
            .panel(
                ResizablePanel::new(PanelSize::percent(50.))
                    .child(label().expanded().text("Bottom")),
            )
    }

    let mut test = launch_test(resizable_container_layout_app);
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));

    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0].layout().area.width(), 300.);
    assert_eq!(labels[0].layout().area.height(), 98.);
    assert_eq!(labels[1].layout().area.width(), 300.);
    assert_eq!(labels[1].layout().area.height(), 98.);
}

#[test]
pub fn resizable_container_horizontal() {
    fn resizable_container_horizontal_app() -> impl IntoElement {
        ResizableContainer::new()
            .direction(Direction::Horizontal)
            .panel(ResizablePanel::new(PanelSize::percent(50.)).child(label().text("Left")))
            .panel(ResizablePanel::new(PanelSize::percent(50.)).child(label().text("Right")))
    }

    let mut test = launch_test(resizable_container_horizontal_app);
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));

    assert_eq!(labels.len(), 2);
    assert_eq!(
        Label::try_downcast(&*labels[0].element()).unwrap().text,
        "Left"
    );
    assert_eq!(
        Label::try_downcast(&*labels[1].element()).unwrap().text,
        "Right"
    );
}

#[test]
pub fn resizable_container_drag_divider() {
    fn resizable_container_drag_app() -> impl IntoElement {
        ResizableContainer::new()
            .panel(
                ResizablePanel::new(PanelSize::percent(50.))
                    .child(label().expanded().text("Panel 1")),
            )
            .panel(
                ResizablePanel::new(PanelSize::percent(50.))
                    .child(label().expanded().text("Panel 2")),
            )
    }

    let mut test = launch_test(resizable_container_drag_app);
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));

    // Get initial panel heights
    // 2 panels, 1 handle (4px)
    // Available space: 500 - 4 = 496px
    // Each panel: 496 / 2 = 248px
    let panel1_initial_height = labels[0].layout().area.height();
    let panel2_initial_height = labels[1].layout().area.height();

    assert!((panel1_initial_height - 248.0).abs() < 10.0);
    assert!((panel2_initial_height - 248.0).abs() < 10.0);

    // Drag the divider down (vertical container, so drag vertically)
    test.move_cursor((250., 250.));
    test.sync_and_update();
    test.press_cursor((250., 250.));
    test.sync_and_update();
    test.move_cursor((250., 350.));
    test.sync_and_update();
    test.release_cursor((250., 350.));
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));

    // Panel 1 should now be larger, Panel 2 smaller
    let panel1_new_height = labels[0].layout().area.height();
    let panel2_new_height = labels[1].layout().area.height();

    assert!(panel1_new_height > panel1_initial_height);
    assert!(panel2_new_height < panel2_initial_height);
}

#[test]
pub fn resizable_container_min_size() {
    fn resizable_container_min_size_app() -> impl IntoElement {
        ResizableContainer::new()
            .panel(
                ResizablePanel::new(PanelSize::percent(50.))
                    .min_size(100.)
                    .child(label().expanded().text("Panel 1")),
            )
            .panel(
                ResizablePanel::new(PanelSize::percent(50.))
                    .min_size(50.)
                    .child(label().expanded().text("Panel 2")),
            )
    }

    let mut test = launch_test(resizable_container_min_size_app);
    test.sync_and_update();

    // Try to drag the divider beyond the minimum size
    test.move_cursor((250., 250.));
    test.sync_and_update();
    test.press_cursor((250., 250.));
    test.sync_and_update();
    // Try to make Panel 1 very small (below its 100px minimum)
    test.move_cursor((250., 50.));
    test.sync_and_update();
    test.release_cursor((250., 50.));
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));

    // Panel 1 should respect its minimum size and not become smaller than 100px
    let panel1_new_height = labels[0].layout().area.height();

    assert!(
        panel1_new_height >= 100.0,
        "Panel 1 height {} should be at least 100px",
        panel1_new_height
    );

    // Both panels should still be visible
    assert_eq!(labels.len(), 2);
    assert_eq!(
        Label::try_downcast(&*labels[0].element()).unwrap().text,
        "Panel 1"
    );
    assert_eq!(
        Label::try_downcast(&*labels[1].element()).unwrap().text,
        "Panel 2"
    );
}

#[test]
pub fn resizable_container_multiple_panels() {
    fn resizable_container_multiple_app() -> impl IntoElement {
        ResizableContainer::new()
            .panel(ResizablePanel::new(PanelSize::percent(33.33)).child(label().text("Panel 1")))
            .panel(ResizablePanel::new(PanelSize::percent(33.33)).child(label().text("Panel 2")))
            .panel(ResizablePanel::new(PanelSize::percent(33.33)).child(label().text("Panel 3")))
    }

    let mut test = launch_test(resizable_container_multiple_app);
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));

    assert_eq!(labels.len(), 3);
    assert_eq!(
        Label::try_downcast(&*labels[0].element()).unwrap().text,
        "Panel 1"
    );
    assert_eq!(
        Label::try_downcast(&*labels[1].element()).unwrap().text,
        "Panel 2"
    );
    assert_eq!(
        Label::try_downcast(&*labels[2].element()).unwrap().text,
        "Panel 3"
    );
}

#[test]
pub fn resizable_container_nested() {
    fn resizable_container_nested_app() -> impl IntoElement {
        ResizableContainer::new()
            .panel(ResizablePanel::new(PanelSize::percent(50.)).child(label().text("Left")))
            .panel(
                ResizablePanel::new(PanelSize::percent(50.)).child(
                    ResizableContainer::new()
                        .direction(Direction::Horizontal)
                        .panel(
                            ResizablePanel::new(PanelSize::percent(50.))
                                .child(label().text("Top Right")),
                        )
                        .panel(
                            ResizablePanel::new(PanelSize::percent(50.))
                                .child(label().text("Bottom Right")),
                        ),
                ),
            )
    }

    let mut test = launch_test(resizable_container_nested_app);
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));

    assert_eq!(labels.len(), 3);
    assert_eq!(
        Label::try_downcast(&*labels[0].element()).unwrap().text,
        "Left"
    );
    assert_eq!(
        Label::try_downcast(&*labels[1].element()).unwrap().text,
        "Top Right"
    );
    assert_eq!(
        Label::try_downcast(&*labels[2].element()).unwrap().text,
        "Bottom Right"
    );
}

#[test]
pub fn resizable_container_dynamic_panels() {
    fn resizable_container_dynamic_app() -> impl IntoElement {
        let mut panel_count = use_state(|| 2);

        rect()
            .child(
                rect()
                    .horizontal()
                    .spacing(8.)
                    .padding(8.)
                    .height(Size::px(50.))
                    .child(
                        Button::new()
                            .on_press(move |_| {
                                *panel_count.write() += 1;
                            })
                            .child("Add Panel"),
                    )
                    .child(
                        Button::new()
                            .on_press(move |_| {
                                if *panel_count.read() > 1 {
                                    *panel_count.write() -= 1;
                                }
                            })
                            .child("Remove Panel"),
                    ),
            )
            .child({
                let mut container = ResizableContainer::new();
                let count = *panel_count.read();
                let percentage = 100.0 / count as f32;

                for i in 0..count {
                    container = container.panel(
                        ResizablePanel::new(PanelSize::percent(percentage))
                            .child(label().expanded().text(format!("Panel {}", i + 1))),
                    );
                }

                container
            })
    }

    let mut test = launch_test(resizable_container_dynamic_app);
    test.sync_and_update();

    // Buttons take 50px, container is 450px
    // Each handle is 4px

    // Initial state: 2 panels, 1 handle (4px)
    // Available space: 450 - 4 = 446px
    // Each panel: 446 / 2 = 223px
    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));
    let panel_labels: Vec<_> = labels
        .iter()
        .filter(|label| {
            let text = Label::try_downcast(&*label.element()).unwrap().text;
            text.starts_with("Panel ")
        })
        .collect();

    assert_eq!(panel_labels.len(), 2);
    assert_eq!(
        Label::try_downcast(&*panel_labels[0].element())
            .unwrap()
            .text,
        "Panel 1"
    );
    assert_eq!(
        Label::try_downcast(&*panel_labels[1].element())
            .unwrap()
            .text,
        "Panel 2"
    );

    let panel1_height = panel_labels[0].layout().area.height();
    let panel2_height = panel_labels[1].layout().area.height();
    assert!((panel1_height - 223.0).abs() < 10.0);
    assert!((panel2_height - 223.0).abs() < 10.0);

    // Click "Add Panel" button
    test.click_cursor((50.0, 25.0));

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));
    let panel_labels: Vec<_> = labels
        .iter()
        .filter(|label| {
            let text = Label::try_downcast(&*label.element()).unwrap().text;
            text.starts_with("Panel ")
        })
        .collect();

    assert_eq!(panel_labels.len(), 3);
    assert_eq!(
        Label::try_downcast(&*panel_labels[0].element())
            .unwrap()
            .text,
        "Panel 1"
    );
    assert_eq!(
        Label::try_downcast(&*panel_labels[1].element())
            .unwrap()
            .text,
        "Panel 2"
    );
    assert_eq!(
        Label::try_downcast(&*panel_labels[2].element())
            .unwrap()
            .text,
        "Panel 3"
    );

    let panel1_height = panel_labels[0].layout().area.height();
    let panel2_height = panel_labels[1].layout().area.height();
    let panel3_height = panel_labels[2].layout().area.height();
    assert!((panel1_height - 170.0).abs() < 10.0);
    assert!((panel2_height - 170.0).abs() < 10.0);
    assert!((panel3_height - 110.0).abs() < 10.0);

    // Click "Remove Panel" button
    test.click_cursor((150.0, 25.0));

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));
    let panel_labels: Vec<_> = labels
        .iter()
        .filter(|label| {
            let text = Label::try_downcast(&*label.element()).unwrap().text;
            text.starts_with("Panel ")
        })
        .collect();

    assert_eq!(panel_labels.len(), 2);

    // Back to 2 panels, 1 handle (4px)
    let panel1_height = panel_labels[0].layout().area.height();
    let panel2_height = panel_labels[1].layout().area.height();
    assert!((panel1_height - 223.0).abs() < 10.0);
    assert!((panel2_height - 223.0).abs() < 10.0);

    test.click_cursor((150.0, 25.0));

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));
    let panel_labels: Vec<_> = labels
        .iter()
        .filter(|label| {
            let text = Label::try_downcast(&*label.element()).unwrap().text;
            text.starts_with("Panel ")
        })
        .collect();

    assert_eq!(panel_labels.len(), 1);
    assert_eq!(
        Label::try_downcast(&*panel_labels[0].element())
            .unwrap()
            .text,
        "Panel 1"
    );

    // Single panel, no handles
    // Available space: 450px
    let panel1_height = panel_labels[0].layout().area.height();
    assert!((panel1_height - 450.0).abs() < 10.0);

    // Try to remove when only 1 panel left (should stay at 1)
    test.click_cursor((150.0, 25.0));

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));
    let panel_labels: Vec<_> = labels
        .iter()
        .filter(|label| {
            let text = Label::try_downcast(&*label.element()).unwrap().text;
            text.starts_with("Panel ")
        })
        .collect();

    assert_eq!(panel_labels.len(), 1);

    let panel1_height = panel_labels[0].layout().area.height();
    assert!((panel1_height - 450.0).abs() < 10.0);
}

#[test]
pub fn resizable_container_pixel_panels() {
    fn resizable_container_pixel_app() -> impl IntoElement {
        ResizableContainer::new()
            .panel(
                ResizablePanel::new(PanelSize::px(200.))
                    .child(label().expanded().text("Pixel Panel")),
            )
            .panel(
                ResizablePanel::new(PanelSize::percent(50.))
                    .child(label().expanded().text("Flex Panel")),
            )
    }

    let mut test = launch_test(resizable_container_pixel_app);
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));

    assert_eq!(labels.len(), 2);

    // Container is 500px, handle is 4px
    // Pixel panel: 200px
    // Flex panel gets the rest: 500 - 200 - 4 = 296px
    let pixel_panel_height = labels[0].layout().area.height();
    let flex_panel_height = labels[1].layout().area.height();

    assert!(
        (pixel_panel_height - 200.0).abs() < 2.0,
        "Pixel panel height {} should be ~200px",
        pixel_panel_height
    );
    assert!(
        (flex_panel_height - 296.0).abs() < 2.0,
        "Flex panel height {} should be ~296px",
        flex_panel_height
    );
}

#[test]
pub fn resizable_container_two_pixel_panels() {
    fn resizable_container_two_pixel_app() -> impl IntoElement {
        ResizableContainer::new()
            .direction(Direction::Horizontal)
            .panel(ResizablePanel::new(PanelSize::px(150.)).child(label().expanded().text("Left")))
            .panel(ResizablePanel::new(PanelSize::px(300.)).child(label().expanded().text("Right")))
    }

    let mut test = launch_test(resizable_container_two_pixel_app);
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));

    assert_eq!(labels.len(), 2);

    let left_width = labels[0].layout().area.width();
    let right_width = labels[1].layout().area.width();

    assert!(
        (left_width - 150.0).abs() < 2.0,
        "Left panel width {} should be ~150px",
        left_width
    );
    assert!(
        (right_width - 300.0).abs() < 2.0,
        "Right panel width {} should be ~300px",
        right_width
    );
}

#[test]
pub fn resizable_container_readded_panel_drag_direction() {
    fn readded_panel_app() -> impl IntoElement {
        let mut show_left = use_state(|| true);
        let is_showing = *show_left.read();

        rect()
            .vertical()
            .width(Size::fill())
            .height(Size::fill())
            .child(
                Button::new()
                    .on_press(move |_| {
                        let next = !*show_left.read();
                        show_left.set(next);
                    })
                    .child(if is_showing { "Hide left" } else { "Show left" }),
            )
            .child(
                ResizableContainer::new()
                    .direction(Direction::Horizontal)
                    .width(Size::fill())
                    .height(Size::fill())
                    .panel(if is_showing {
                        Some(
                            ResizablePanel::new(PanelSize::px(280.))
                                .key("left")
                                .min_size(200.)
                                .child(label().expanded().text("Left")),
                        )
                    } else {
                        None
                    })
                    .panel(
                        ResizablePanel::new(PanelSize::percent(100.))
                            .key("middle")
                            .child(label().expanded().text("Middle")),
                    ),
            )
    }

    let mut test = launch_test(readded_panel_app);
    test.sync_and_update();

    let left_width = |test: &TestingRunner| -> f32 {
        let labels =
            test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));
        labels
            .iter()
            .find(|n| Label::try_downcast(&*n.element()).unwrap().text == "Left")
            .map(|n| n.layout().area.width())
            .unwrap()
    };

    // Left starts at its 280px initial size.
    let w0 = left_width(&test);
    assert!((w0 - 280.0).abs() < 5.0, "initial left width {} ~280", w0);

    // Hide the left panel, then show it again.
    test.click_cursor((40.0, 20.0));
    test.sync_and_update();
    test.click_cursor((40.0, 20.0));
    test.sync_and_update();

    // Left is back at its 280px initial size.
    let w1 = left_width(&test);
    assert!((w1 - 280.0).abs() < 5.0, "re-shown left width {} ~280", w1);

    // Grab the divider just to the right of the left panel and push it right.
    let labels = test.find_many(|node, element| Label::try_downcast(element).map(move |_| node));
    let left_area = labels
        .iter()
        .find(|n| Label::try_downcast(&*n.element()).unwrap().text == "Left")
        .unwrap()
        .layout()
        .area;
    let handle_x = left_area.max_x() as f64 + 2.0;
    let handle_y = (left_area.min_y() + left_area.height() / 2.0) as f64;

    test.move_cursor((handle_x, handle_y));
    test.sync_and_update();
    test.press_cursor((handle_x, handle_y));
    test.sync_and_update();
    test.move_cursor((handle_x + 120.0, handle_y));
    test.sync_and_update();
    test.release_cursor((handle_x + 120.0, handle_y));
    test.sync_and_update();

    // Pushing the divider to the right must GROW the left panel, not shrink it.
    let w2 = left_width(&test);
    assert!(
        w2 > w1 + 50.0,
        "left panel should grow when dragging right (was {}, now {})",
        w1,
        w2
    );
}
