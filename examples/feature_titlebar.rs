use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_decorations(false)))
}

fn app() -> impl IntoElement {
    let minimize = EventHandler::new(move |_| {
        Platform::get().with_window(None, |window| {
            window.set_minimized(true);
        });
    });

    let maximize = EventHandler::new(move |_| {
        Platform::get().with_window(None, |window| {
            let is_max = window.is_maximized();
            window.set_maximized(!is_max);
        });
    });

    let close = EventHandler::new(move |_| {
        let platform = Platform::get();
        Platform::get().with_window(None, move |window| {
            platform.close_window(window.id());
        });
    });

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .vertical()
        .child(
            Titlebar::new()
                .title("My App Title")
                .child(minimize_button(minimize))
                .child(maximize_button(maximize))
                .child(close_button(close)),
        )
        .child(
            rect()
                .width(Size::fill())
                .height(Size::fill())
                .background(Color::WHITE)
                .center()
                .child(label().text("Main content area")),
        )
}

fn minimize_button(on_press: EventHandler<Event<PressEventData>>) -> Element {
    TitlebarButton::new()
        .on_press(on_press)
        .child(label().text("-").color(Color::BLACK))
        .into()
}

fn maximize_button(on_press: EventHandler<Event<PressEventData>>) -> Element {
    TitlebarButton::new()
        .on_press(on_press)
        .child(label().text("□").color(Color::BLACK))
        .into()
}

fn close_button(on_press: EventHandler<Event<PressEventData>>) -> Element {
    TitlebarButton::new()
        .on_press(on_press)
        .child(label().text("x").color(Color::BLACK))
        .into()
}
