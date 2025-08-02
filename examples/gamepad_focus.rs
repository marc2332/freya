#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::thread;

use freya::{
    core::accessibility::AccessibilityFocusStrategy,
    prelude::*,
};
use freya_core::{
    event_loop_messages::{
        EventLoopAppMessage,
        EventLoopAppMessageAction,
        EventLoopMessage,
    },
    events::{
        KeyboardEventName,
        PlatformEvent,
    },
    plugins::{
        FreyaPlugin,
        PluginEvent,
        PluginHandle,
    },
};
use gilrs::{
    EventType,
    Gilrs,
};
use winit::window::WindowId;

fn main() {
    launch_cfg(
        LaunchConfig::new()
            .with_plugin(GamePadPlugin)
            .with_window(WindowConfig::new(app)),
    )
}

#[derive(Default)]
pub struct GamePadPlugin;

impl GamePadPlugin {
    pub fn listen_gamepad(handle: PluginHandle, window_id: WindowId) {
        thread::spawn(move || {
            println!("Listening for gamepads");

            let mut gilrs_instance = Gilrs::new().unwrap();

            loop {
                while let Some(ev) = gilrs_instance.next_event() {
                    match ev.event {
                        EventType::ButtonReleased(gilrs::Button::DPadLeft, _) => {
                            handle.send_event_loop_event(EventLoopMessage::App(
                                EventLoopAppMessage {
                                    window_id: Some(window_id),
                                    action: EventLoopAppMessageAction::FocusAccessibilityNode(
                                        AccessibilityFocusStrategy::Backward,
                                    ),
                                },
                            ));
                        }
                        EventType::ButtonReleased(gilrs::Button::DPadRight, _) => {
                            handle.send_event_loop_event(EventLoopMessage::App(
                                EventLoopAppMessage {
                                    window_id: Some(window_id),
                                    action: EventLoopAppMessageAction::FocusAccessibilityNode(
                                        AccessibilityFocusStrategy::Forward,
                                    ),
                                },
                            ));
                        }
                        EventType::ButtonReleased(gilrs::Button::East, _) => {
                            handle.send_platform_event(
                                PlatformEvent::Keyboard {
                                    name: KeyboardEventName::KeyDown,
                                    key: Key::Enter,
                                    code: Code::Enter,
                                    modifiers: Modifiers::default(),
                                },
                                window_id,
                            );
                        }
                        _ => {}
                    }
                }
            }
        });
    }
}

impl FreyaPlugin for GamePadPlugin {
    fn on_event(&mut self, event: &PluginEvent, handle: PluginHandle) {
        if let PluginEvent::WindowCreated(window) = event {
            Self::listen_gamepad(handle, window.id());
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
