use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut position = use_state(|| 1);
    rect()
        .content(Content::flex())
        .horizontal()
        .children_iter((0..3).map(|i| {
            rect()
                .center()
                .width(Size::flex(1.))
                .height(Size::fill())
                .child(DropZone::<usize>::new(
                    if position() == i {
                        DragZone::<usize>::new(
                            i,
                            rect()
                                .background((125, 189, 25))
                                .width(Size::px(100.))
                                .height(Size::px(100.))
                                .child(format!("Drag Element {i}")),
                        )
                        .drag_element(
                            rect()
                                .background((25, 189, 125))
                                .width(Size::px(100.))
                                .height(Size::px(100.))
                                .child(format!("Drop Element {i}")),
                        )
                        .show_while_dragging(false)
                        .into_element()
                    } else {
                        rect()
                            .center()
                            .height(Size::fill())
                            .child(label().text("Drop me here!"))
                            .into_element()
                    },
                    move |_| {
                        position.set(i);
                    },
                ))
                .into()
        }))
}
