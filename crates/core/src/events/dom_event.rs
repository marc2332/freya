use std::{
    any::Any,
    rc::Rc,
};

use freya_elements::{
    events::{
        pointer::PointerType,
        ErasedEventData,
        FileData,
        KeyboardData,
        MouseData,
        PointerData,
        TouchData,
        WheelData,
    },
    WheelSource,
};
use freya_native_core::NodeId;
use ragnarok::NameOfEvent;
use torin::prelude::*;

use super::EventName;
use crate::events::PlatformEvent;

/// Event emitted to the DOM.
#[derive(Debug, Clone, PartialEq)]
pub struct DomEvent {
    pub name: EventName,
    pub source_event: EventName,
    pub node_id: NodeId,
    pub data: DomEventData,
    pub bubbles: bool,
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

impl ragnarok::EmmitableEvent for DomEvent {
    type Key = NodeId;
    type Name = EventName;

    fn key(&self) -> Self::Key {
        self.node_id
    }

    fn name(&self) -> Self::Name {
        self.name
    }

    fn source(&self) -> Self::Name {
        self.source_event
    }
}

impl DomEvent {
    pub fn new(
        node_id: NodeId,
        name: EventName,
        platform_event: PlatformEvent,
        node_area: Option<Area>,
        scale_factor: f64,
    ) -> Self {
        let bubbles = name.does_bubble();

        match platform_event {
            PlatformEvent::Mouse {
                name: platform_event_name,
                cursor,
                button,
                ..
            } => {
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
                    source_event: platform_event_name.into(),
                    data: event_data,
                    bubbles,
                }
            }
            PlatformEvent::Wheel {
                name: platform_event_name,
                scroll,
                ..
            } => Self {
                node_id,
                name,
                source_event: platform_event_name.into(),
                data: DomEventData::Wheel(WheelData::new(WheelSource::Device, scroll.x, scroll.y)),
                bubbles,
            },
            PlatformEvent::Keyboard {
                name: platform_event_name,
                ref key,
                code,
                modifiers,
                ..
            } => Self {
                node_id,
                name,

                source_event: platform_event_name.into(),
                data: DomEventData::Keyboard(KeyboardData::new(key.clone(), code, modifiers)),
                bubbles,
            },
            PlatformEvent::Touch {
                name: platform_event_name,
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
                    source_event: platform_event_name.into(),
                    data: event_data,
                    bubbles,
                }
            }
            PlatformEvent::File {
                name: platform_event_name,
                file_path,
                ..
            } => {
                let event_data = DomEventData::File(FileData { file_path });

                Self {
                    node_id,
                    name,
                    source_event: platform_event_name.into(),
                    data: event_data,
                    bubbles,
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
