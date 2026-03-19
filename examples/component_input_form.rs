#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    icons,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let password = use_state(String::new);
    let mut mode = use_state(InputMode::new_password);

    rect().center().expanded().spacing(6.).child(
        Input::new(password)
            .placeholder("Password")
            .mode(mode.read().clone())
            .width(Size::px(200.))
            .leading(
                svg(icons::lucide::lock())
                    .width(Size::px(18.))
                    .height(Size::px(18.))
                    .color((150, 150, 150)),
            )
            .trailing(
                CursorArea::new().icon(CursorIcon::Pointer).child(
                    svg(if matches!(*mode.read(), InputMode::Shown) {
                        icons::lucide::eye()
                    } else {
                        icons::lucide::eye_off()
                    })
                    .width(Size::px(18.))
                    .height(Size::px(18.))
                    .color((150, 150, 150))
                    .on_pointer_down(move |e: Event<PointerEventData>| {
                        e.stop_propagation();
                    })
                    .on_press(move |e: Event<PressEventData>| {
                        e.stop_propagation();
                        e.prevent_default();
                        let mut mode = mode.write();
                        if matches!(*mode, InputMode::Shown) {
                            *mode = InputMode::new_password();
                        } else {
                            *mode = InputMode::Shown;
                        }
                    }),
                ),
            ),
    )
}
