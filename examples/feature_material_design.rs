#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    material_design::*,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(550., 600.)))
}

fn context_menu() -> Menu {
    Menu::new()
        .child(
            MenuItem::new()
                .on_press(|e: Event<PressEventData>| {
                    e.stop_propagation();
                    e.prevent_default();
                    println!("Open pressed");
                })
                .ripple()
                .child("Open"),
        )
        .child(
            MenuItem::new()
                .on_press(|e: Event<PressEventData>| {
                    e.stop_propagation();
                    e.prevent_default();
                    println!("Save pressed");
                })
                .ripple()
                .child("Save"),
        )
}

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .spacing(16.)
        .padding(16.)
        .child(
            Button::new()
                .on_press(|_| {
                    if ContextMenu::is_open() {
                        ContextMenu::close();
                    } else {
                        ContextMenu::open(context_menu());
                    }
                })
                .flat()
                .ripple()
                .child("Open Menu"),
        )
        .child(
            rect()
                .horizontal()
                .main_align(Alignment::center())
                .spacing(8.)
                .child(FloatingTab::new().ripple().child("Home"))
                .child(FloatingTab::new().ripple().child("Settings"))
                .child(FloatingTab::new().ripple().child("Profile")),
        )
        .child(
            rect()
                .width(Size::px(200.))
                .child(
                    SideBarItem::new()
                        .on_press(|_| println!("Home pressed"))
                        .ripple()
                        .child("Home"),
                )
                .child(
                    SideBarItem::new()
                        .on_press(|_| println!("Settings pressed"))
                        .ripple()
                        .child("Settings"),
                ),
        )
}
