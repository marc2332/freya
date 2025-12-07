use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(500., 450.)))
}

fn app() -> impl IntoElement {
    let mut count = use_state(|| 4);

    let counter = rect()
        .width(Size::fill())
        .height(Size::percent(50.))
        .center()
        .color((255, 255, 255))
        .background((15, 163, 242))
        .font_size(75.)
        .shadow((0., 4., 20., 4., (0, 0, 0, 80)))
        .child(count.read().to_string());

    let actions = rect()
        .horizontal()
        .width(Size::fill())
        .height(Size::percent(50.))
        .center()
        .spacing(8.0)
        .child(
            Button::new()
                .on_press(move |_| {
                    *count.write() += 1;
                })
                .child("Increase"),
        )
        .child(
            Button::new()
                .on_press(move |_| {
                    *count.write() -= 1;
                })
                .child("Decrease"),
        );

    rect().child(counter).child(actions)
}
