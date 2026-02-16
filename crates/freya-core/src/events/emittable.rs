use ragnarok::{
    Area,
    NameOfEvent,
};
use torin::prelude::CursorPoint;

use crate::{
    events::{
        data::{
            EventType,
            KeyboardEventData,
            MouseEventData,
            PointerEventData,
            TouchEventData,
            WheelEventData,
        },
        name::EventName,
    },
    integration::PlatformEvent,
    node_id::NodeId,
    prelude::{
        FileEventData,
        ImePreeditEventData,
    },
};
/// Event emitted to the Tree.
#[derive(Debug, Clone, PartialEq)]
pub struct EmmitableEvent {
    pub name: EventName,
    pub source_event: EventName,
    pub node_id: NodeId,
    pub data: EventType,
    pub bubbles: bool,
}

impl Eq for EmmitableEvent {}

impl PartialOrd for EmmitableEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EmmitableEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl ragnarok::EmmitableEvent for EmmitableEvent {
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

impl EmmitableEvent {
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
            } if name.is_enter() || name.is_left() || name.is_press() || name.is_down() => {
                let global_location = cursor / scale_factor;
                let element_x =
                    (cursor.x - node_area.unwrap_or_default().min_x() as f64) / scale_factor;
                let element_y =
                    (cursor.y - node_area.unwrap_or_default().min_y() as f64) / scale_factor;

                let event_data = EventType::Pointer(PointerEventData::Mouse(MouseEventData {
                    global_location,
                    element_location: CursorPoint::new(element_x, element_y),
                    button,
                }));

                Self {
                    node_id,
                    name,
                    source_event: platform_event_name.into(),
                    data: event_data,
                    bubbles,
                }
            }
            PlatformEvent::Touch {
                name: platform_event_name,
                location,
                finger_id,
                phase,
                force,
                ..
            } if name.is_enter() || name.is_left() || name.is_press() || name.is_down() => {
                let global_location = location / scale_factor;
                let element_x =
                    (location.x - node_area.unwrap_or_default().min_x() as f64) / scale_factor;
                let element_y =
                    (location.y - node_area.unwrap_or_default().min_y() as f64) / scale_factor;

                let event_data = EventType::Pointer(PointerEventData::Touch(TouchEventData::new(
                    global_location,
                    CursorPoint::new(element_x, element_y),
                    finger_id,
                    phase,
                    force,
                )));

                Self {
                    node_id,
                    name,
                    source_event: platform_event_name.into(),
                    data: event_data,
                    bubbles,
                }
            }
            PlatformEvent::Mouse {
                name: platform_event_name,
                cursor,
                button,
                ..
            } => {
                let global_location = cursor / scale_factor;
                let element_x =
                    (cursor.x - node_area.unwrap_or_default().min_x() as f64) / scale_factor;
                let element_y =
                    (cursor.y - node_area.unwrap_or_default().min_y() as f64) / scale_factor;

                let event_data = EventType::Mouse(MouseEventData {
                    global_location,
                    element_location: CursorPoint::new(element_x, element_y),
                    button,
                });

                Self {
                    node_id,
                    name,
                    source_event: platform_event_name.into(),
                    data: event_data,
                    bubbles,
                }
            }
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
                data: EventType::Keyboard(KeyboardEventData::new(key.clone(), code, modifiers)),
                bubbles,
            },
            PlatformEvent::Wheel {
                name: platform_event_name,
                scroll,
                source,
                cursor,
                ..
            } => {
                let global_location = cursor / scale_factor;
                let element_x =
                    (cursor.x - node_area.unwrap_or_default().min_x() as f64) / scale_factor;
                let element_y =
                    (cursor.y - node_area.unwrap_or_default().min_y() as f64) / scale_factor;
                let element_location = CursorPoint::new(element_x, element_y);

                Self {
                    node_id,
                    name,
                    source_event: platform_event_name.into(),
                    data: EventType::Wheel(WheelEventData::new(
                        scroll.x,
                        scroll.y,
                        source,
                        global_location,
                        element_location,
                    )),
                    bubbles,
                }
            }
            PlatformEvent::Touch {
                name: platform_event_name,
                location,
                finger_id,
                phase,
                force,
                ..
            } => {
                let global_location = location / scale_factor;
                let element_x =
                    (location.x - node_area.unwrap_or_default().min_x() as f64) / scale_factor;
                let element_y =
                    (location.y - node_area.unwrap_or_default().min_y() as f64) / scale_factor;

                let event_data = EventType::Touch(TouchEventData::new(
                    global_location,
                    CursorPoint::new(element_x, element_y),
                    finger_id,
                    phase,
                    force,
                ));

                Self {
                    node_id,
                    name,
                    source_event: platform_event_name.into(),
                    data: event_data,
                    bubbles,
                }
            }
            PlatformEvent::ImePreedit {
                name: platform_event_name,
                cursor,
                text,
            } => Self {
                node_id,
                name,

                source_event: platform_event_name.into(),
                data: EventType::ImePreedit(ImePreeditEventData::new(text, cursor)),
                bubbles,
            },
            PlatformEvent::File {
                name: platform_event_name,
                cursor,
                file_path,
            } => Self {
                node_id,
                name,

                source_event: platform_event_name.into(),
                data: EventType::File(FileEventData::new(cursor, file_path)),
                bubbles,
            },
        }
    }
}
