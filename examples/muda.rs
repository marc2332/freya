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
                let menu_bar = menu_bar.clone();
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
            SubMenu {
                text: "Stuff",
                enabled: true,
                MenuItem {
                    text: "Reset to 0",
                    enabled: true,
                    onclick: move |_| count.set(0)
                }
            }
        }
        label {
            "{count}"
        }
    )
}

#[component]
fn WindowMenu(children: Element) -> Element {
    let platform = use_platform();
    let mut menus_notifier = use_context_provider(|| Signal::new(()));

    let rx = use_hook(|| {
        let menu_bar = Arc::new(SharedMenu(consume_context::<muda::Menu>()));
        struct SharedMenu(pub muda::Menu);

        unsafe impl Send for SharedMenu {};
        unsafe impl Sync for SharedMenu {};

        platform.with_window(move |window| {
            use winit::raw_window_handle::*;
            if let RawWindowHandle::Win32(handle) = window.window_handle().unwrap().as_raw() {
                unsafe { menu_bar.0.init_for_hwnd(handle.hwnd.get()).unwrap() };
            }
        });

        let (tx, rx) = tokio::sync::broadcast::channel::<muda::MenuEvent>(5);

        {
            let tx = tx.clone();
            muda::MenuEvent::set_event_handler(Some(move |event| {
                tx.send(event);
            }));
        }

        MenuReceiver(rx)
    });

    let (tx, rx2) = use_hook(|| {
        let (tx, rx) = tokio::sync::broadcast::channel::<()>(50);

        (tx, MenuReceiver(rx))
    });

    use_memo(move || {
        menus_notifier();

        let menu_bar = consume_context::<muda::Menu>();

        for item in menu_bar.items() {
            if let Some(item) = item.as_menuitem() {
                menu_bar.remove(item);
            } else if let Some(submenu) = item.as_submenu() {
                menu_bar.remove(submenu);
            }
        }

        tx.send(());
    });

    provide_context(rx);
    provide_context(rx2);

    children
}

struct MenuReceiver<T>(pub Receiver<T>);

impl<T: Clone> Clone for MenuReceiver<T> {
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

    use_memo(move || {
        let mut menus_notifier = consume_context::<Signal<()>>();

        text.read();
        enabled.read();

        menus_notifier.write();
    });

    use_hook(move || {
        let mut rx = consume_context::<MenuReceiver<()>>();

        spawn(async move {
            while let Ok(ev) = rx.0.recv().await {
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
    use_memo(move || {
        let mut menus_notifier = consume_context::<Signal<()>>();

        onclick.read();
        text.read();
        enabled.read();

        menus_notifier.write();
    });

    let mut menu = use_signal(|| None);

    use_hook(move || {
        let mut rx = consume_context::<MenuReceiver<()>>();

        spawn(async move {
            while let Ok(ev) = rx.0.recv().await {
                let mut submenu_signal = try_consume_context::<Signal<Option<muda::Submenu>>>();

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
        let mut rx = consume_context::<MenuReceiver<muda::MenuEvent>>();

        spawn(async move {
            while let Ok(ev) = rx.0.recv().await {
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
