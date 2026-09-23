use freya::prelude::*;
use freya_testing::prelude::*;
use freya_wgpu::prelude::*;

const WINDOW: f32 = 500.0;

fn footer_area(mut test: TestingRunner) -> Option<Area> {
    test.sync_and_update();
    test.find(|node, element| {
        Label::try_downcast(element)
            .filter(|label| label.text.as_ref() == "footer")
            .map(|_| node.layout().area)
    })
}

fn app_with(height: Size) -> impl Fn() -> Element + Clone {
    move || {
        rect()
            .expanded()
            .vertical()
            .content(Content::flex())
            .child(
                WgpuViewer::new(|_, _| {})
                    .width(Size::fill())
                    .height(height.clone()),
            )
            .child(label().text("footer"))
            .into_element()
    }
}

#[test]
fn a_flexible_viewer_leaves_room_for_its_siblings() {
    let area = footer_area(launch_test(app_with(Size::flex(1.0)))).expect("footer is laid out");

    assert!(area.height() > 0.0);
    assert!(area.min_y() < WINDOW);
    assert!(area.max_y() <= WINDOW + 1.0);
}

#[test]
fn a_filled_viewer_takes_the_whole_parent() {
    let area = footer_area(launch_test(app_with(Size::fill()))).expect("footer is laid out");

    assert!(area.min_y() >= WINDOW);
}

#[test]
fn children_are_laid_out_inside_the_viewer() {
    let mut test = launch_test(|| {
        WgpuViewer::new(|_, _| {})
            .padding(16.0)
            .child(label().text("overlay"))
            .into_element()
    });
    test.sync_and_update();

    let area = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|label| label.text.as_ref() == "overlay")
                .map(|_| node.layout().area)
        })
        .expect("the overlay is laid out");

    assert_eq!(area.min_x(), 16.0);
    assert_eq!(area.min_y(), 16.0);
}
