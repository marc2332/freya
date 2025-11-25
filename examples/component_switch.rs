use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut toggled = use_state(|| false);
    rect()
        .center()
        .expanded()
        .spacing(6.)
        .child(Switch::new().toggled(toggled).on_toggle(move |_| {
            toggled.toggle();
        }))
        .child(Switch::new().toggled(toggled).enabled(false))
}
