use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_transparency(true)
                .with_background(Color::TRANSPARENT)
                .with_decorations(false),
        ),
    )
}

fn app() -> impl IntoElement {
    let mut count = use_state(|| 0);

    rect().expanded().center().child(
        Button::new()
            .on_press(move |_| *count.write() += 1)
            .child(format!("{}", count.read())),
    )
}
