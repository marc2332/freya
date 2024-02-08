use std::{any::Any, rc::Rc};

use dioxus_core::ElementId;
use dioxus_native_core::NodeId;
use freya_elements::{
    elements::PlatformEventData,
    events::{pointer::PointerType, KeyboardData, MouseData, PointerData, TouchData, WheelData},
};
use torin::prelude::*;

use crate::events::FreyaEvent;

/// Event emitted to the DOM.
#[derive(Debug, Clone, PartialEq)]
pub struct DomEvent {
    pub name: String,
    pub node_id: NodeId,
    pub element_id: ElementId,
    pub data: DomEventData,
}

impl Eq for DomEvent {}

impl PartialOrd for DomEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DomEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.name.as_str() {
            "mouseleave" | "pointerleave" => {
                if self.name == other.name {
                    std::cmp::Ordering::Equal
                } else {
                    std::cmp::Ordering::Less
                }
            }
            _ => std::cmp::Ordering::Greater,
        }
    }
}

impl DomEvent {
    pub fn does_move_cursor(&self) -> bool {
        return does_event_move_cursor(self.name.as_str());
    }

    // Bubble all events except keyboard
    pub fn should_bubble(&self) -> bool {
        !matches!(self.data, DomEventData::Keyboard(_))
    }

    pub fn new(
        node_id: NodeId,
        element_id: ElementId,
        event: &FreyaEvent,
        node_area: Option<Area>,
        scale_factor: f64,
    ) -> Self {
        let is_pointer_event = event.is_pointer_event();
        let event_name = event.get_name().to_string();

        match event {
            FreyaEvent::Mouse { cursor, button, .. } => {
                let screen_coordinates = *cursor / scale_factor;
                let element_x =
                    (cursor.x - node_area.unwrap_or_default().min_x() as f64) / scale_factor;
                let element_y =
                    (cursor.y - node_area.unwrap_or_default().min_y() as f64) / scale_factor;

                let event_data = if is_pointer_event {
                    DomEventData::Pointer(PointerData::new(
                        screen_coordinates,
                        (element_x, element_y).into(),
                        PointerType::Mouse {
                            trigger_button: *button,
                        },
                    ))
                } else {
                    DomEventData::Mouse(MouseData::new(
                        screen_coordinates,
                        (element_x, element_y).into(),
                        *button,
                    ))
                };

                Self {
                    node_id,
                    element_id,
                    name: event_name,
                    data: event_data,
                }
            }
            FreyaEvent::Wheel { scroll, .. } => Self {
                node_id,
                element_id,
                name: event_name,
                data: DomEventData::Wheel(WheelData::new(scroll.x, scroll.y)),
            },
            FreyaEvent::Keyboard {
                ref key,
                code,
                modifiers,
                ..
            } => Self {
                node_id,
                element_id,
                name: event_name,
                data: DomEventData::Keyboard(KeyboardData::new(key.clone(), *code, *modifiers)),
            },
            FreyaEvent::Touch {
                location,
                finger_id,
                phase,
                force,
                ..
            } => {
                let element_x = location.x - node_area.unwrap_or_default().min_x() as f64;
                let element_y = location.y - node_area.unwrap_or_default().min_y() as f64;

                let event_data = if is_pointer_event {
                    DomEventData::Pointer(PointerData::new(
                        *location,
                        (element_x, element_y).into(),
                        PointerType::Touch {
                            finger_id: *finger_id,
                            phase: *phase,
                            force: *force,
                        },
                    ))
                } else {
                    DomEventData::Touch(TouchData::new(
                        *location,
                        (element_x, element_y).into(),
                        *finger_id,
                        *phase,
                        *force,
                    ))
                };

                Self {
                    node_id,
                    element_id,
                    name: event_name,
                    data: event_data,
                }
            }
        }
    }
}

/// Data of a DOM event.
#[derive(Debug, Clone, PartialEq)]
pub enum DomEventData {
    Mouse(MouseData),
    Keyboard(KeyboardData),
    Wheel(WheelData),
    Touch(TouchData),
    Pointer(PointerData),
}

impl DomEventData {
    pub fn any(self) -> Rc<dyn Any> {
        match self {
            DomEventData::Mouse(m) => Rc::new(PlatformEventData::new(Box::new(m))),
            DomEventData::Keyboard(k) => Rc::new(PlatformEventData::new(Box::new(k))),
            DomEventData::Wheel(w) => Rc::new(PlatformEventData::new(Box::new(w))),
            DomEventData::Touch(t) => Rc::new(PlatformEventData::new(Box::new(t))),
            DomEventData::Pointer(p) => Rc::new(PlatformEventData::new(Box::new(p))),
        }
    }
}

pub fn does_event_move_cursor(event_name: &str) -> bool {
    ["pointerover", "pointerenter", "mouseover", "mouseenter"].contains(&event_name)
}
