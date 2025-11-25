use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .child(
            ScrollView::new()
                .height(Size::percent(50.))
                .spacing(6.)
                .children_iter((0..30).map(|_| {
                    rect()
                        .width(Size::fill())
                        .height(Size::px(80.))
                        .background((182, 119, 0))
                        .into()
                })),
        )
        .child(
            ScrollView::new()
                .direction(Direction::Horizontal)
                .height(Size::percent(50.))
                .spacing(6.)
                .children_iter((0..30).map(|_| {
                    rect()
                        .width(Size::px(80.))
                        .height(Size::fill())
                        .background((0, 119, 182))
                        .into()
                })),
        )
}
