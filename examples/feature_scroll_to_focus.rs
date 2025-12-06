use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    ScrollView::new()
        .spacing(4.)
        .direction(Direction::Horizontal)
        .children_iter((0..7).map(|_| {
            ScrollView::new()
                .width(Size::px(200.))
                .spacing(4.)
                .children_iter(
                    (0..25).map(|i| Button::new().key(i).child(format!("Button {i}")).into()),
                )
                .into()
        }))
}
