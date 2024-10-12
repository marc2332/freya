#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use freya::prelude::*;
use tokio::sync::broadcast::Receiver;
use winit::platform::windows::EventLoopBuilderExtWindows;

fn main() {
    let menu_bar = muda::Menu::new();

    launch_cfg(
        app,
        LaunchConfig::<muda::Menu>::new()
            .with_title("Muda")
            .with_state(menu_bar.clone())
            .with_event_loop_builder(move |event_loop_builder| {
                #[cfg(target_os = "windows")]
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
            }),
    );
}

fn app() -> Element {
    let mut count = use_signal(|| 0);

    rsx!(
        WindowMenu {
            menu: rsx!(
                MenuItem {
                    text: "+",
                    enabled: true,
                    onclick: move |_| count += 1
                }
                MenuItem {
                    text: "{count}",
                    enabled: true,
                    onclick: move |_| {}
                }
                MenuItem {
                    text: "-",
                    enabled: true,
                    onclick: move |_| count -= 1
                }
                if count() < 3 {
                    MenuItem {
                        text: "count is smaller than 3",
                        enabled: true,
                        onclick: move |_| {}
                    }
                }
                SubMenu {
                    text: "Stuff",
                    enabled: true,
                    MenuItem {
                        text: "Reset to 0",
                        enabled: true,
                        onclick: move |_| count.set(0)
                    }
                }
            )
        }
        label {
            "{count}"
        }
    )
}

#[component]
fn WindowMenu(menu: ReadOnlySignal<Element>) -> Element {
    let platform = use_platform();

    let click_receiver = use_hook(|| {
        let menu_bar = Arc::new(SharedMenu(consume_context::<muda::Menu>()));
        struct SharedMenu(pub muda::Menu);

        unsafe impl Send for SharedMenu {}
        unsafe impl Sync for SharedMenu {}

        platform.with_window(move |window| {
            use winit::raw_window_handle::*;

            #[cfg(target_os = "windows")]
            if let RawWindowHandle::Win32(handle) = window.window_handle().unwrap().as_raw() {
                unsafe { menu_bar.0.init_for_hwnd(handle.hwnd.get()).unwrap() };
            }

            #[cfg(target_os = "linux")]
            menu_bar.0.init_for_gtk_window(&window, None);

            #[cfg(target_os = "macos")]
            menu_bar.0.init_for_nsapp();
        });

        let (tx, rx) = tokio::sync::broadcast::channel::<muda::MenuEvent>(5);

        muda::MenuEvent::set_event_handler(Some(move |event| {
            tx.send(event).ok();
        }));

        MenuEventReceiver(rx)
    });

    let (creation_sender, creation_receiver) = use_hook(|| {
        let (tx, rx) = tokio::sync::broadcast::channel::<()>(50);

        (tx, MenuEventReceiver(rx))
    });

    use_effect(move || {
        menu.read();

        let menu_bar = consume_context::<muda::Menu>();

        for item in menu_bar.items() {
            if let Some(item) = item.as_menuitem() {
                menu_bar.remove(item).expect("Failed to remove menu.");
            } else if let Some(submenu) = item.as_submenu() {
                menu_bar.remove(submenu).expect("Failed to remove submenu.");
            }
        }

        creation_sender
            .send(())
            .expect("Failed to notify menus of an update.");
    });

    provide_context(click_receiver);
    provide_context(creation_receiver);

    menu()
}

struct MenuEventReceiver<T>(pub Receiver<T>);

impl<T: Clone> Clone for MenuEventReceiver<T> {
    fn clone(&self) -> Self {
        Self(self.0.resubscribe())
    }
}

#[component]
fn SubMenu(
    text: ReadOnlySignal<String>,
    enabled: ReadOnlySignal<bool>,
    children: Element,
) -> Element {
    let mut submenu = use_context_provider(|| Signal::new(None));

    use_hook(move || {
        let mut creation_receiver = consume_context::<MenuEventReceiver<()>>();

        spawn(async move {
            while creation_receiver.0.recv().await.is_ok() {
                let menu_bar = consume_context::<muda::Menu>();
                let new_submenu = muda::Submenu::new(&*text.peek(), *enabled.peek());

                menu_bar.append(&new_submenu).unwrap();
                submenu.set(Some(new_submenu));
            }
        });
    });

    children
}

#[component]
fn MenuItem(
    onclick: ReadOnlySignal<EventHandler<()>>,
    text: ReadOnlySignal<String>,
    enabled: ReadOnlySignal<bool>,
) -> Element {
    let mut menu = use_signal(|| None);

    use_hook(move || {
        let mut creation_receiver = consume_context::<MenuEventReceiver<()>>();

        spawn(async move {
            while creation_receiver.0.recv().await.is_ok() {
                let submenu_signal = try_consume_context::<Signal<Option<muda::Submenu>>>();

                let new_menu_item = muda::MenuItem::new(&*text.peek(), *enabled.peek(), None);

                if let Some(submenu_signal) = submenu_signal {
                    let submenu = submenu_signal.peek();
                    let submenu = submenu.as_ref().unwrap();
                    submenu.append(&new_menu_item).unwrap();
                } else {
                    let menu_bar = consume_context::<muda::Menu>();
                    menu_bar.append(&new_menu_item).unwrap();
                }

                menu.set(Some(new_menu_item));
            }
        });
    });

    use_hook(move || {
        let mut click_receiver = consume_context::<MenuEventReceiver<muda::MenuEvent>>();

        spawn(async move {
            while let Ok(event) = click_receiver.0.recv().await {
                if let Some(menu) = &*menu.read() {
                    if event.id == menu.id() {
                        onclick.peek().call(());
                    }
                }
            }
        });
    });

    None
}
