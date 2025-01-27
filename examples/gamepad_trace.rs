#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    thread::{
        self,
        sleep,
    },
    time::Duration,
};

use freya::prelude::*;
use freya_core::prelude::{
    EventName,
    FreyaPlugin,
    PlatformEvent,
    PluginEvent,
    PluginHandle,
};
use gilrs::{
    Axis,
    EventType,
    Gilrs,
};

fn main() {
    launch_cfg(app, LaunchConfig::<()>::new().with_plugin(GamePadPlugin))
}

#[derive(Default)]
pub struct GamePadPlugin;

impl GamePadPlugin {
    pub fn listen_gamepad(handle: PluginHandle) {
        thread::spawn(move || {
            println!("Listening for gamepads");

            let mut gilrs = Gilrs::new().unwrap();

            loop {
                let (mut x, mut y) = (200.0f64, 200.0f64);
                let (mut diff_x, mut diff_y) = (0., 0.);

                let mut event = gilrs.next_event();
                while let Some(ev) = event {
                    loop {
                        sleep(Duration::from_millis(16));
                        match ev.event {
                            EventType::AxisChanged(Axis::LeftStickX, diff, _) => {
                                diff_x = diff as f64;
                            }
                            EventType::AxisChanged(Axis::LeftStickY, diff, _) => {
                                diff_y = diff as f64;
                            }
                            _ => {}
                        }

                        if diff_x != 0.0 {
                            x += diff_x * 10.;
                            handle.send_platform_event(PlatformEvent {
                                name: EventName::MouseMove,
                                data: PlatformEventData::Mouse {
                                    cursor: (x, y).into(),
                                    button: None,
                                },
                            });
                        }

                        if diff_x != 0.0 {
                            y -= diff_y * 10.;
                            handle.send_platform_event(PlatformEvent {
                                name: EventName::MouseMove,
                                data: PlatformEventData::Mouse {
                                    cursor: (x, y).into(),
                                    button: None,
                                },
                            });
                        }

                        let new_event = gilrs.next_event();

                        if let Some(new_ev) = new_event {
                            if new_ev != ev {
                                event = new_event;
                                break;
                            }
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

const MOVEMENT_MARGIN: f64 = 75.0;
const BOX_COUNT: usize = 80;

#[allow(non_snake_case)]
fn Box() -> Element {
    rsx!(
        rect {
            background: "rgb(65, 53, 67)",
            width: "250",
            height: "250",
            main_align: "center",
            cross_align: "center",
            corner_radius: "100",
            rect {
                background: "rgb(143, 67, 238)",
                width: "180",
                height: "180",
                main_align: "center",
                cross_align: "center",
                corner_radius: "100",
                rect {
                    background: "rgb(240, 235, 141)",
                    width: "100",
                    height: "100",
                    corner_radius: "100",
                }
            }
        }
    )
}

fn app() -> Element {
    let mut positions = use_signal::<Vec<CursorPoint>>(Vec::new);

    let onmousemove = move |e: MouseEvent| {
        let coordinates = e.get_screen_coordinates();
        positions.with_mut(|positions| {
            if let Some(pos) = positions.first() {
                if (pos.x + MOVEMENT_MARGIN < coordinates.x
                    && pos.x - MOVEMENT_MARGIN > coordinates.x)
                    && (pos.y + MOVEMENT_MARGIN < coordinates.y
                        && pos.y - MOVEMENT_MARGIN > coordinates.y)
                {
                    return;
                }
            }
            positions.insert(0, (coordinates.x - 125.0, coordinates.y - 125.0).into());
            positions.truncate(BOX_COUNT);
        })
    };

    rsx!(
        rect {
            onmousemove,
            width: "100%",
            height: "100%",
            {positions.read().iter().map(|pos| rsx!(
                rect {
                    width: "0",
                    height: "0",
                    offset_x: "{pos.x}",
                    offset_y: "{pos.y}",
                    Box {}
                }
            ))}
        }
    )
}
