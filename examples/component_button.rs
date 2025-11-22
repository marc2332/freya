use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    rect()
        .center()
        .expanded()
        .horizontal()
        .spacing(6.)
        .child(rect().spacing(6.).children(buttons()))
        .child(rect().spacing(6.).children(disabled_buttons()))
}

fn buttons() -> [Element; 9] {
    [
        Button::new().child("Hello, World!").into(),
        Button::new().child("Hello, World!").filled().into(),
        Button::new().child("Hello, World!").outline().into(),
        Button::new().child("Hello, World!").expanded().into(),
        Button::new()
            .child("Hello, World!")
            .expanded()
            .filled()
            .into(),
        Button::new()
            .child("Hello, World!")
            .expanded()
            .outline()
            .into(),
        Button::new().child("Hello, World!").compact().into(),
        Button::new()
            .child("Hello, World!")
            .compact()
            .filled()
            .into(),
        Button::new()
            .child("Hello, World!")
            .compact()
            .outline()
            .into(),
    ]
}

fn disabled_buttons() -> [Element; 9] {
    [
        Button::new().child("Hello, World!").enabled(false).into(),
        Button::new()
            .child("Hello, World!")
            .enabled(false)
            .filled()
            .into(),
        Button::new()
            .child("Hello, World!")
            .enabled(false)
            .outline()
            .into(),
        Button::new()
            .child("Hello, World!")
            .enabled(false)
            .expanded()
            .into(),
        Button::new()
            .child("Hello, World!")
            .enabled(false)
            .expanded()
            .filled()
            .into(),
        Button::new()
            .child("Hello, World!")
            .enabled(false)
            .expanded()
            .outline()
            .into(),
        Button::new()
            .child("Hello, World!")
            .enabled(false)
            .compact()
            .into(),
        Button::new()
            .child("Hello, World!")
            .enabled(false)
            .compact()
            .filled()
            .into(),
        Button::new()
            .child("Hello, World!")
            .enabled(false)
            .compact()
            .outline()
            .into(),
    ]
}
