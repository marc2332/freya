#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::thread;

use freya::prelude::*;
use freya_core::prelude::{
    EventMessage,
    EventName,
    FreyaPlugin,
    PlatformEvent,
    PluginEvent,
    PluginHandle,
};
use gilrs::{
    EventType,
    Gilrs,
};

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new().with_plugin(GamePadPlugin),
    )
}

#[derive(Default)]
pub struct GamePadPlugin;

impl GamePadPlugin {
    pub fn listen_gamepad(handle: PluginHandle) {
        thread::spawn(move || {
            println!("Listening for gamepads");

            let mut gilrs_instance = Gilrs::new().unwrap();

            loop {
                while let Some(ev) = gilrs_instance.next_event() {
                    if let EventType::ButtonReleased(_, code) = ev.event {
                        // NOTE: You might need to tweak these codes
                        match code.into_u32() {
                            4 => {
                                handle.send_event_loop_event(
                                    EventMessage::FocusPrevAccessibilityNode,
                                );
                            }
                            6 => {
                                handle.send_event_loop_event(
                                    EventMessage::FocusNextAccessibilityNode,
                                );
                            }
                            13 => {
                                handle.send_platform_event(PlatformEvent::Keyboard {
                                    name: EventName::KeyDown,
                                    key: Key::Enter,
                                    code: Code::Enter,
                                    modifiers: Modifiers::default(),
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }
}

impl FreyaPlugin for GamePadPlugin {
    fn on_event(&mut self, event: &PluginEvent, handle: PluginHandle) {
        if let PluginEvent::WindowCreated(_) = event {
            Self::listen_gamepad(handle);
        }
    }
}

fn app() -> Element {
    let mut count = use_signal(|| 0);
    let mut enabled = use_signal(|| true);

    rsx!(
        rect {
            height: "fill",
            width: "fill",
            main_align: "center",
            cross_align: "center",
            Button {
                onpress: move |_| count += 1,
                label {
                    "Increase -> {count}"
                }
            }
            Switch {
                enabled: *enabled.read(),
                ontoggled: move |_| {
                    enabled.toggle();
                }
            }
            Button {
                onpress: move |_| count -= 1,
                label {
                    "Decrease -> {count}"
                }
            }
        }
    )
}
