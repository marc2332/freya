#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fmt::Display;

use freya::prelude::*;

fn main() {
    let menu_bar = muda::Menu::new();

    launch_cfg(
        app,
        LaunchConfig::<muda::Menu>::new()
            .with_title("Muda")
            .with_state(menu_bar.clone())
            .with_event_loop_builder(move |event_loop_builder| {
                #[cfg(target_os = "windows")]
                {
                    use winit::platform::windows::EventLoopBuilderExtWindows;
                    event_loop_builder.with_msg_hook(move |msg| {
                        use windows_sys::Win32::UI::WindowsAndMessaging::{
                            TranslateAcceleratorW,
                            MSG,
                        };
                        unsafe {
                            let msg = msg as *const MSG;
                            let translated =
                                TranslateAcceleratorW((*msg).hwnd, menu_bar.haccel() as _, msg);
                            translated == 1
                        }
                    });
                }
            }),
    );
}


#[derive(Clone, Copy, PartialEq)]
enum MenuExample {
    Counter,
    FileEditor,
    Minimal
}

impl Display for MenuExample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Counter => f.write_str("Counter"),
            Self::FileEditor => f.write_str("FileEditor"),
            Self::Minimal => f.write_str("Minimal"),
        }
    }
}

#[component]
fn CounterExample() -> Element {
    let mut count = use_signal(|| 0);

    println!("{count:?}");

    rsx!(
        WindowMenu {
            menu: rsx!(
            WindowMenuItem {
                text: "+",
                onclick: move |_| {
                    println!("increased");
                    count += 1
                }
            }
            WindowMenuItem {
                text: "{count}",
                onclick: move |_| {}
            }
            WindowMenuItem {
                text: "-",
                onclick: move |_| count -= 1
            }
            if count() < 3 {
                WindowMenuItem {
                    text: "count is smaller than 3",
                    onclick: move |_| {}
                }
            }
            WindowSubMenu {
                text: "Stuff",
                WindowMenuItem {
                    text: "Reset to 0",
                    onclick: move |_| count.set(0)
                }
            }
        )
    }
    )
}


#[component]
fn FileEditor() -> Element {
    rsx!(
       
        WindowMenu {
            menu: rsx!(
                WindowSubMenu { 
                    text: "File",
                    WindowMenuItem {
                        text: "New File",
                    }
                    WindowMenuItem {
                        text: "Open",
                    }
                    WindowMenuItem {
                        text: "Save",
                    }
                    WindowMenuItem {
                        text: "Save As",
                    }
                    WindowMenuItem {
                        text: "Close",
                    }
                }
                WindowSubMenu {
                    text: "Beta Features",
                    enabled: false
                }
                WindowSubMenu {
                    text: "About",
                    WindowMenuItem {
                        text: "Help",
                    }
                    WindowMenuItem {
                        text: "Contact",
                    }
                    WindowMenuItem {
                        text: "Version 0.3.0",
                    }
                }
            )
        }
    )
}

#[component]
fn Minimal() -> Element {
    rsx!(
        WindowMenu {
            menu: rsx!(
            WindowSubMenu {
                text: "About",
                WindowMenuItem {
                    text: "Help",
                }
                WindowMenuItem {
                    text: "Contact",
                }
                WindowMenuItem {
                    text: "Version 0.3.0",
                }
            }
        )
    }
    )
}

fn app() -> Element {
    let mut example = use_signal(|| MenuExample::Counter);

    rsx!(
        match example() {
            MenuExample::Counter => rsx!( CounterExample {} ),
            MenuExample::FileEditor => rsx!( FileEditor {} ),
            MenuExample::Minimal => rsx!( Minimal {} ),
        }
        rect {
            main_align: "center",
            cross_align: "center",
            width: "fill",
            height: "fill",
            Dropdown {
                value: example(),
                for ex in [MenuExample::Counter, MenuExample::FileEditor, MenuExample::Minimal] {
                    DropdownItem {
                        value: ex,
                        onpress: move |_| example.set(ex),
                        label { "{ex}" }
                    }
                }
            }
        }
    )
}
