#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use accelerator::Accelerator;
use freya::prelude::*;
use muda::*;

fn main() {
    launch_cfg(app, LaunchConfig::<()>::new().with_title("Muda"));
}

fn app() -> Element {
    let platform = use_platform();

    use_hook(|| {
        platform.with_window(|window| {
            let menu_bar = Menu::new();
            let submenu = Submenu::with_items(
                "Submenu Outer",
                true,
                &[
                    &MenuItem::new("Menu item #3", true, None),
                ],
            )
            .unwrap();

            menu_bar.append_items(&[&submenu]);

            use winit::raw_window_handle::*;
            if let RawWindowHandle::Win32(handle) = window.window_handle().unwrap().as_raw() {
                println!("Works");
                unsafe { menu_bar.init_for_hwnd(handle.hwnd.get()).unwrap() };
            }
        });
    });

    rsx!(label {
        "hi"
    })
}
