use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    svg(freya_icons::lucide::antenna())
        .color((120, 50, 255))
        .expanded()
        .into()
}
