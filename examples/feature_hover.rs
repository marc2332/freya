use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut hovering = use_state(|| false);

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .background(if *hovering.read() {
            (100, 100, 100)
        } else {
            (50, 50, 50)
        })
        .color((255, 255, 255))
        .center()
        .child("Hover me")
        .on_pointer_enter(move |_| hovering.set(true))
        .on_pointer_leave(move |_| hovering.set(false))
}
