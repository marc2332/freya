#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::PathBuf;

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let uri = "https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExeXh5YWhscmo0YmF3OG1oMmpnMzBnbXFjcDR5Y2xoODE2ZnRpc2FhZiZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/HTZVeK0esRjyw/giphy.gif";
    let path = PathBuf::from("./examples/frog_typing.gif");
    let embedded = ("frog-typing", include_bytes!("./frog_typing.gif"));

    rect()
        .expanded()
        .horizontal()
        .center()
        .child(
            GifViewer::new(uri)
                .width(Size::percent(33.))
                .a11y_alt("Frog typing"),
        )
        .child(
            GifViewer::new(path)
                .width(Size::percent(33.))
                .a11y_alt("Frog typing"),
        )
        .child(
            GifViewer::new(embedded)
                .width(Size::percent(33.))
                .a11y_alt("Frog typing"),
        )
}
