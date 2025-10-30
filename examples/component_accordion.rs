use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit,
sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. ";

fn app() -> Element {
    rect()
        .center()
        .expanded()
        .spacing(4.)
        .children_iter((0..3).map(|_| {
            accordion()
                .header("Click to expand!")
                .content(LOREM_IPSUM)
                .into()
        }))
        .into()
}
