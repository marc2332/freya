//! Simulate touch input and Android UI behavior without changing the native backend.
//!
//! Add `freya-platform-simulator-plugin` as a dependency and register
//! [`PlatformSimulatorPlugin`] with `.with_plugin(PlatformSimulatorPlugin::new())`.
//!
//! - **Ctrl+Shift+T** toggles mouse-to-touch input in the focused window.
//! - **Ctrl+Shift+A** switches all windows to Android UI behavior. Press it again to restore the previous platform.

use freya_core::{
    integration::*,
    prelude::TargetPlatform,
};
use freya_winit::plugins::{
    FreyaPlugin,
    PluginEvent,
    PluginHandle,
};
use keyboard_types::{
    Code,
    Modifiers,
};
use rustc_hash::FxHashMap;
use torin::prelude::CursorPoint;
use winit::{
    event::{
        PointerKind,
        WindowEvent,
    },
    window::WindowId,
};

/// Adds keyboard shortcuts for simulating touch input and Android UI behavior.
#[derive(Default)]
pub struct PlatformSimulatorPlugin {
    windows: FxHashMap<WindowId, TouchSimulator>,
    previous_platform: Option<TargetPlatform>,
}

impl PlatformSimulatorPlugin {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FreyaPlugin for PlatformSimulatorPlugin {
    fn plugin_id(&self) -> &'static str {
        "platform-simulator"
    }

    fn on_event(&mut self, event: &mut PluginEvent, _handle: PluginHandle) {
        match event {
            PluginEvent::WindowEvent {
                window, raw, input, ..
            } => {
                let Some(simulator) = self.windows.get_mut(&window.id()) else {
                    return;
                };

                match raw {
                    WindowEvent::Focused(false) => input.extend(simulator.cancel()),
                    WindowEvent::PointerLeft {
                        kind: PointerKind::Mouse,
                        ..
                    } => {
                        simulator.cursor_in_window = false;
                        input.extend(simulator.cancel());
                    }
                    _ => {}
                }
            }
            PluginEvent::WindowClosed { window, .. } => {
                self.windows.remove(&window.id());
            }
            _ => {}
        }
    }

    fn filter_input(&mut self, window_id: WindowId, events: &mut Vec<PlatformEvent>) {
        let simulator = self.windows.entry(window_id).or_default();
        for event in std::mem::take(events) {
            if let PlatformEvent::Keyboard {
                name,
                code,
                modifiers,
                ..
            } = &event
            {
                let shortcut = match code {
                    Code::KeyT => Some(&mut simulator.touch_shortcut),
                    Code::KeyA => Some(&mut simulator.platform_shortcut),
                    _ => None,
                };

                if let Some(shortcut) = shortcut {
                    match shortcut.handle(*name, *modifiers) {
                        ShortcutAction::Pass => {}
                        ShortcutAction::Consume => continue,
                        ShortcutAction::Toggle => {
                            if *code == Code::KeyT {
                                simulator.toggle(events);
                            } else {
                                let platform = TargetPlatform::get();
                                if platform == TargetPlatform::Android {
                                    TargetPlatform::set(
                                        self.previous_platform
                                            .take()
                                            .unwrap_or_else(TargetPlatform::detect),
                                    );
                                } else {
                                    self.previous_platform = Some(platform);
                                    TargetPlatform::set(TargetPlatform::Android);
                                }
                            }
                            continue;
                        }
                    }
                }
            }

            events.extend(simulator.translate(event));
        }
    }
}

#[derive(Default)]
enum ShortcutState {
    #[default]
    Released,
    PassedThrough,
    Consumed,
}

enum ShortcutAction {
    Pass,
    Consume,
    Toggle,
}

impl ShortcutState {
    fn handle(&mut self, name: KeyboardEventName, modifiers: Modifiers) -> ShortcutAction {
        if name != KeyboardEventName::KeyDown {
            return match std::mem::take(self) {
                Self::Consumed => ShortcutAction::Consume,
                _ => ShortcutAction::Pass,
            };
        }

        match self {
            Self::Released if modifiers.contains(Modifiers::CONTROL | Modifiers::SHIFT) => {
                *self = Self::Consumed;
                ShortcutAction::Toggle
            }
            Self::Released => {
                *self = Self::PassedThrough;
                ShortcutAction::Pass
            }
            Self::PassedThrough => ShortcutAction::Pass,
            Self::Consumed => ShortcutAction::Consume,
        }
    }
}

#[derive(Default)]
struct TouchSimulator {
    enabled: bool,
    touch_shortcut: ShortcutState,
    platform_shortcut: ShortcutState,
    cursor_in_window: bool,
    touch_active: bool,
    location: CursorPoint,
}

impl TouchSimulator {
    fn toggle(&mut self, events: &mut Vec<PlatformEvent>) {
        events.extend(self.cancel());
        self.enabled = !self.enabled;
        events.push(PlatformEvent::PointerExit {
            cursor: self.location,
        });
        if !self.enabled && self.cursor_in_window {
            events.push(PlatformEvent::Mouse {
                name: MouseEventName::MouseMove,
                cursor: self.location,
                button: None,
            });
        }
    }

    fn cancel(&mut self) -> Option<PlatformEvent> {
        std::mem::take(&mut self.touch_active)
            .then(|| self.touch(TouchEventName::TouchCancel, TouchPhase::Cancelled))
    }

    fn translate(&mut self, event: PlatformEvent) -> Option<PlatformEvent> {
        if let PlatformEvent::Mouse { cursor, .. } = &event {
            self.location = *cursor;
            self.cursor_in_window = true;
        }

        if !self.enabled {
            return Some(event);
        }

        match event {
            PlatformEvent::Mouse { name, button, .. } => match (name, button) {
                (MouseEventName::MouseDown, Some(MouseButton::Left)) if !self.touch_active => {
                    self.touch_active = true;
                    Some(self.touch(TouchEventName::TouchStart, TouchPhase::Started))
                }
                (MouseEventName::MouseMove, _) if self.touch_active => {
                    Some(self.touch(TouchEventName::TouchMove, TouchPhase::Moved))
                }
                (MouseEventName::MouseUp, Some(MouseButton::Left)) if self.touch_active => {
                    self.touch_active = false;
                    Some(self.touch(TouchEventName::TouchEnd, TouchPhase::Ended))
                }
                _ => None,
            },
            PlatformEvent::PointerCancel { .. } if self.touch_active => self.cancel(),
            PlatformEvent::Wheel { .. } => None,
            event => Some(event),
        }
    }

    fn touch(&self, name: TouchEventName, phase: TouchPhase) -> PlatformEvent {
        PlatformEvent::Touch {
            name,
            location: self.location,
            finger_id: u64::MAX,
            phase,
            force: None,
        }
    }
}
