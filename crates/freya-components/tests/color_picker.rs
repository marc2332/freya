use std::time::Duration;

use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
pub fn color_picker_sv_selects_correct_color() {
    fn cp_app() -> impl IntoElement {
        let mut color = use_state(|| Color::RED);

        rect()
            .child(label().text(format!(
                "Color: #{:02X}{:02X}{:02X}",
                color().r(),
                color().g(),
                color().b()
            )))
            .child(ColorPicker::new(move |c| color.set(c)).value(color()))
    }

    let mut test = launch_test(cp_app);
    test.sync_and_update();

    // Open popup
    let preview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|_| {
                    (node.layout().area.size.width - 40.0).abs() < 1.0
                        && (node.layout().area.size.height - 24.0).abs() < 1.0
                })
                .map(|_| node)
        })
        .unwrap();

    let preview_area = preview.layout().area;
    let preview_center = (
        preview_area.min_x() as f64 + (preview_area.size.width as f64) / 2.0,
        preview_area.min_y() as f64 + (preview_area.size.height as f64) / 2.0,
    );

    test.click_cursor(preview_center);
    test.sync_and_update();

    // Wait for popup to appear
    test.poll_n(Duration::from_millis(5), 100);
    test.sync_and_update();

    // Find popup and sv area
    let popup = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|_| (node.layout().area.size.width - 220.0).abs() < 5.0)
                .map(|_| node)
        })
        .unwrap();

    let popup_area = popup.layout().area;

    let sv = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|_| {
                    let a = node.layout().area;
                    (a.size.height - 140.0).abs() < 10.0
                        && a.min_x() >= popup_area.min_x()
                        && a.min_y() >= popup_area.min_y()
                })
                .map(|_| node)
        })
        .unwrap();

    let sv_area = sv.layout().area;

    // Choose a point: sat = 0.75, v = 0.75 (rel_x=0.75, rel_y=0.25)
    let rel_x = 0.75f64;
    let rel_y = 0.25f64;

    let sv_point = (
        sv_area.min_x() as f64 + sv_area.size.width as f64 * rel_x,
        sv_area.min_y() as f64 + sv_area.size.height as f64 * rel_y,
    );

    // Expected color computed using same algorithm as component
    const MIN_S: f32 = 0.07;
    const MIN_V: f32 = 0.07;
    let initial_h = Color::RED.to_hsv().h;
    let sat = (rel_x as f32).max(MIN_S);
    let v = (1.0 - rel_y as f32).clamp(MIN_V, 1.0 - MIN_V);
    let expected = Color::from_hsv(initial_h, sat, v);

    // Click and wait
    test.click_cursor(sv_point);
    test.sync_and_update();
    test.poll_n(Duration::from_millis(5), 20);
    test.sync_and_update();

    // Read label
    let label_node = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Color:"))
                .map(|_| node)
        })
        .unwrap();

    let label_text = Label::try_downcast(&*label_node.element())
        .unwrap()
        .text
        .clone();
    let hex = label_text.split_whitespace().nth(1).unwrap();
    let actual = Color::from_hex(hex).unwrap();

    // Compare in HSV space with tolerances
    let ah = actual.to_hsv();
    let eh = expected.to_hsv();
    let mut dh = (ah.h - eh.h).abs();
    if dh > 180.0 {
        dh = 360.0 - dh;
    }

    assert!(dh < 10.0, "h mismatch {} vs {}", ah.h, eh.h);
    assert!(
        (ah.s - eh.s).abs() < 0.15,
        "s mismatch {} vs {}",
        ah.s,
        eh.s
    );
    assert!(
        (ah.v - eh.v).abs() < 0.15,
        "v mismatch {} vs {}",
        ah.v,
        eh.v
    );
}

#[test]
pub fn color_picker_hue_changes_color() {
    fn cp_app() -> impl IntoElement {
        let mut color = use_state(|| Color::from_hsv(0.0, 1.0, 1.0));

        rect()
            .child(label().text(format!(
                "Color: #{:02X}{:02X}{:02X}",
                color().r(),
                color().g(),
                color().b()
            )))
            .child(ColorPicker::new(move |c| color.set(c)).value(color()))
    }

    let mut test = launch_test(cp_app);
    test.sync_and_update();

    // Open popup
    let preview = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|_| {
                    (node.layout().area.size.width - 40.0).abs() < 1.0
                        && (node.layout().area.size.height - 24.0).abs() < 1.0
                })
                .map(|_| node)
        })
        .unwrap();

    let preview_area = preview.layout().area;
    let preview_center = (
        preview_area.min_x() as f64 + (preview_area.size.width as f64) / 2.0,
        preview_area.min_y() as f64 + (preview_area.size.height as f64) / 2.0,
    );

    test.click_cursor(preview_center);
    test.sync_and_update();
    test.poll_n(Duration::from_millis(5), 100);
    test.sync_and_update();

    // Find popup and hue bar
    let popup = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|_| (node.layout().area.size.width - 220.0).abs() < 5.0)
                .map(|_| node)
        })
        .unwrap();

    let popup_area = popup.layout().area;

    let hue = test
        .find(|node, element| {
            Rect::try_downcast(element)
                .filter(|_| {
                    let a = node.layout().area;
                    (a.size.height - 18.0).abs() < 5.0
                        && a.min_x() >= popup_area.min_x()
                        && a.min_y() >= popup_area.min_y()
                })
                .map(|_| node)
        })
        .unwrap();

    let hue_area = hue.layout().area;
    let hue_point = (
        hue_area.min_x() as f64 + hue_area.size.width as f64 * 0.5,
        hue_area.min_y() as f64 + hue_area.size.height as f64 * 0.5,
    );

    // Expected hue: ~180 deg
    let expected = Color::from_hsv(180.0, 1.0, 1.0);

    test.click_cursor(hue_point);
    test.sync_and_update();
    test.poll_n(Duration::from_millis(5), 20);
    test.sync_and_update();

    let label_node = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Color:"))
                .map(|_| node)
        })
        .unwrap();

    let label_text = Label::try_downcast(&*label_node.element())
        .unwrap()
        .text
        .clone();
    let hex = label_text.split_whitespace().nth(1).unwrap();
    let actual = Color::from_hex(hex).unwrap();

    let ah = actual.to_hsv();
    let eh = expected.to_hsv();
    let mut dh = (ah.h - eh.h).abs();
    if dh > 180.0 {
        dh = 360.0 - dh;
    }

    assert!(dh < 5.0, "h mismatch {} vs {}", ah.h, eh.h);
    assert!(
        (ah.s - eh.s).abs() < 0.05,
        "s mismatch {} vs {}",
        ah.s,
        eh.s
    );
    assert!(
        (ah.v - eh.v).abs() < 0.05,
        "v mismatch {} vs {}",
        ah.v,
        eh.v
    );
}
