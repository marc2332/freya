#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    ScrollView::new()
        .scrollbar(custom_scrollbar)
        .child(rect().spacing(8.).padding(8.).children((0..30).map(|_| {
            rect()
                .width(Size::fill())
                .height(Size::px(80.))
                .padding(8.)
                .corner_radius(8.)
                .background((46, 125, 246))
        })))
}

fn custom_scrollbar(context: ScrollBarContext) -> Element {
    let ScrollBarContext {
        axis,
        thumb_events,
        thumb_offset,
        thumb_length,
        ..
    } = context;
    let (thumb_width, thumb_height, thumb_offset_x, thumb_offset_y) = match axis {
        Axis::X => (
            Size::px(thumb_length),
            Size::px(20.),
            thumb_offset + 3.,
            -25.,
        ),
        Axis::Y => (
            Size::px(20.),
            Size::px(thumb_length),
            -25.,
            thumb_offset + 3.,
        ),
    };

    rect()
        .position(Position::new_absolute())
        .layer(999)
        .child(
            rect()
                .expanded()
                .direction(if axis == Axis::Y {
                    Direction::vertical()
                } else {
                    Direction::horizontal()
                })
                .cross_align(Alignment::end())
                .child(
                    rect()
                        .position(Position::new_absolute())
                        .width(thumb_width)
                        .height(thumb_height)
                        .offset_x(thumb_offset_x)
                        .offset_y(thumb_offset_y)
                        .child(
                            rect()
                                .expanded()
                                .corner_radius(8.)
                                .background((46, 125, 246))
                                .border(Border::new().fill((255, 255, 255)).width(2.))
                                .shadow((0., 4., 20., 4., (0, 0, 0, 80)))
                                .on_pointer_down(thumb_events.on_pointer_down)
                                .on_pointer_press(thumb_events.on_pointer_press),
                        ),
                ),
        )
        .into()
}
