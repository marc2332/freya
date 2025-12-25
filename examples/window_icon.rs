use freya::prelude::*;

const ICON: &[u8] = include_bytes!("./freya_icon.png");

fn main() {
    launch(
        LaunchConfig::new()
            .with_window(WindowConfig::new(app).with_icon(LaunchConfig::window_icon(ICON))),
    )
}

fn app() -> impl IntoElement {
    rect().center().expanded().child("Window with an icon!")
}
