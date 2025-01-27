use std::{
    any::Any,
    rc::Rc,
};

use freya_elements::{
    elements::ErasedEventData,
    events::{
        pointer::PointerType,
        FileData,
        KeyboardData,
        MouseData,
        PointerData,
        TouchData,
        WheelData,
    },
};
use freya_native_core::NodeId;
use torin::prelude::*;

use super::{
    EventName,
    PlatformEventData,
};
use crate::prelude::PotentialEvent;

/// Event emitted to the DOM.
#[derive(Debug, Clone, PartialEq)]
pub struct DomEvent {
    pub name: EventName,
    pub node_id: NodeId,
    pub data: DomEventData,
    pub bubbles: bool,
    pub layer: Option<i16>,
}

impl Eq for DomEvent {}

impl PartialOrd for DomEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DomEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl DomEvent {
    pub fn new(
        PotentialEvent {
            node_id,
            layer,
            name,
            data,
        }: PotentialEvent,
        node_area: Option<Area>,
        scale_factor: f64,
    ) -> Self {
        let bubbles = name.does_bubble();

        match data {
            PlatformEventData::Mouse { cursor, button, .. } => {
                let screen_coordinates = cursor / scale_factor;
                let element_x =
                    (cursor.x - node_area.unwrap_or_default().min_x() as f64) / scale_factor;
                let element_y =
                    (cursor.y - node_area.unwrap_or_default().min_y() as f64) / scale_factor;

                let event_data = if name.is_pointer() {
                    DomEventData::Pointer(PointerData::new(
                        screen_coordinates,
                        (element_x, element_y).into(),
                        PointerType::Mouse {
                            trigger_button: button,
                        },
                    ))
                } else {
                    DomEventData::Mouse(MouseData::new(
                        screen_coordinates,
                        (element_x, element_y).into(),
                        button,
                    ))
                };

                Self {
                    node_id,
                    name,
                    data: event_data,
                    bubbles,
                    layer,
                }
            }
            PlatformEventData::Wheel { scroll, .. } => Self {
                node_id,
                name,
                data: DomEventData::Wheel(WheelData::new(scroll.x, scroll.y)),
                bubbles,
                layer,
            },
            PlatformEventData::Keyboard {
                ref key,
                code,
                modifiers,
                ..
            } => Self {
                node_id,
                name,
                data: DomEventData::Keyboard(KeyboardData::new(key.clone(), code, modifiers)),
                bubbles,
                layer,
            },
            PlatformEventData::Touch {
                location,
                finger_id,
                phase,
                force,
                ..
            } => {
                let element_x = location.x - node_area.unwrap_or_default().min_x() as f64;
                let element_y = location.y - node_area.unwrap_or_default().min_y() as f64;

                let event_data = if name.is_pointer() {
                    DomEventData::Pointer(PointerData::new(
                        location,
                        (element_x, element_y).into(),
                        PointerType::Touch {
                            finger_id,
                            phase,
                            force,
                        },
                    ))
                } else {
                    DomEventData::Touch(TouchData::new(
                        location,
                        (element_x, element_y).into(),
                        finger_id,
                        phase,
                        force,
                    ))
                };

                Self {
                    node_id,
                    name,
                    data: event_data,
                    bubbles,
                    layer,
                }
            }
            PlatformEventData::File { file_path, .. } => {
                let event_data = DomEventData::File(FileData { file_path });

                Self {
                    node_id,
                    name,
                    data: event_data,
                    bubbles,
                    layer,
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
    File(FileData),
}

impl DomEventData {
    pub fn any(self) -> Rc<dyn Any> {
        match self {
            DomEventData::Mouse(m) => Rc::new(ErasedEventData::new(Box::new(m))),
            DomEventData::Keyboard(k) => Rc::new(ErasedEventData::new(Box::new(k))),
            DomEventData::Wheel(w) => Rc::new(ErasedEventData::new(Box::new(w))),
            DomEventData::Touch(t) => Rc::new(ErasedEventData::new(Box::new(t))),
            DomEventData::Pointer(p) => Rc::new(ErasedEventData::new(Box::new(p))),
            DomEventData::File(fd) => Rc::new(ErasedEventData::new(Box::new(fd))),
        }
    }
}
