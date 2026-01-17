use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_decorations(false)))
}

fn app() -> impl IntoElement {
    let minimize = move |_| {
        Platform::get().with_window(None, |window| {
            window.set_minimized(true);
        });
    };

    let maximize = move |_| {
        Platform::get().with_window(None, |window| {
            let is_max = window.is_maximized();
            window.set_maximized(!is_max);
        });
    };

    let close = move |_| {
        let platform = Platform::get();
        Platform::get().with_window(None, move |window| {
            platform.close_window(window.id());
        });
    };

    rect()
        .expanded()
        .vertical()
        .child(
            rect()
                .horizontal()
                .background((225, 225, 225))
                .content(Content::Flex)
                .height(Size::px(32.))
                .cross_align(Alignment::Center)
                .padding((0., 0., 0., 8.))
                .child("Custom Titlebar!")
                .child(
                    rect()
                        .window_drag()
                        .width(Size::flex(1.))
                        .height(Size::fill()),
                )
                .child(TitlebarButton::new(TitlebarAction::Minimize).on_press(minimize))
                .child(TitlebarButton::new(TitlebarAction::Maximize).on_press(maximize))
                .child(TitlebarButton::new(TitlebarAction::Close).on_press(close)),
        )
        .child(rect().expanded().center().child("Hello, World!"))
}
