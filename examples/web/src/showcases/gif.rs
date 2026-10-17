use freya::prelude::*;

use crate::showcases::heading;

const FROG_URL: &str =
    "https://raw.githubusercontent.com/marc2332/freya/main/examples/frog_typing.gif";

#[derive(PartialEq)]
pub struct GifShowcase;

impl Component for GifShowcase {
    fn render(&self) -> impl IntoElement {
        rect()
            .padding(24.)
            .spacing(20.)
            .expanded()
            .child(heading("Gif", "Animated, frame by frame"))
            .child(
                GifViewer::new(FROG_URL)
                    .width(Size::px(500.))
                    .height(Size::px(280.)),
            )
    }
}
