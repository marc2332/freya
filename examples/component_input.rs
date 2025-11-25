use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut value = use_state(String::new);

    rect()
        .expanded()
        .center()
        .spacing(6.)
        .child(
            Input::new()
                .placeholder("Type your name")
                .value(value.read().clone())
                .onchange(move |v| value.set(v)),
        )
        .child(
            Input::new()
                .placeholder("Can't type here!")
                .enabled(false)
                .value(value.read().clone()),
        )
        .child(format!("Your name is {}", value.read()))
}
