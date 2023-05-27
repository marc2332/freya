use std::{any::Any, rc::Rc};

use dioxus_core::ElementId;
use dioxus_native_core::NodeId;
use freya_elements::events::{
    keyboard::{Code, Key, Modifiers},
    pointer::PointerType,
    KeyboardData, MouseData, PointerData, TouchData, WheelData,
};
use rustc_hash::FxHashMap;
use torin::prelude::*;
use winit::event::{Force, MouseButton, TouchPhase};

use crate::EventEmitter;

/// Events emitted in Freya.
#[derive(Clone, Debug)]
pub enum FreyaEvent {
    /// A Mouse Event.
    Mouse {
        name: String,
        cursor: CursorPoint,
        button: Option<MouseButton>,
    },
    /// A Wheel event.
    Wheel {
        name: String,
        scroll: CursorPoint,
        cursor: CursorPoint,
    },
    /// A Keyboard event.
    Keyboard {
        name: String,
        key: Key,
        code: Code,
        modifiers: Modifiers,
    },
    /// A Touch event.
    Touch {
        name: String,
        location: CursorPoint,
        finger_id: u64,
        phase: TouchPhase,
        force: Option<Force>,
    },
}

impl FreyaEvent {
    pub fn get_name(&self) -> &str {
        match self {
            Self::Mouse { name, .. } => name,
            Self::Wheel { name, .. } => name,
            Self::Keyboard { name, .. } => name,
            Self::Touch { name, .. } => name,
        }
    }

    pub fn set_name(&mut self, new_name: String) {
        match self {
            Self::Mouse { name, .. } => *name = new_name,
            Self::Wheel { name, .. } => *name = new_name,
            Self::Keyboard { name, .. } => *name = new_name,
            Self::Touch { name, .. } => *name = new_name,
        }
    }

    pub fn is_pointer_event(&self) -> bool {
        self.get_name().starts_with("point")
    }
}

pub fn does_event_move_cursor(event_name: &str) -> bool {
    ["pointerover", "pointerenter", "mouseover", "mouseenter"].contains(&event_name)
}

/// Event emitted to the DOM.
#[derive(Debug, Clone)]
pub struct DomEvent {
    pub name: String,
    pub node_id: NodeId,
    pub element_id: ElementId,
    pub data: DomEventData,
}

impl DomEvent {
    pub fn does_move_cursor(&self) -> bool {
        return does_event_move_cursor(self.name.as_str());
    }

    pub fn from_freya_event(
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
#[derive(Debug, Clone)]
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
            DomEventData::Mouse(m) => Rc::new(m),
            DomEventData::Keyboard(k) => Rc::new(k),
            DomEventData::Wheel(w) => Rc::new(w),
            DomEventData::Touch(t) => Rc::new(t),
            DomEventData::Pointer(p) => Rc::new(p),
        }
    }
}

/// State of an element.
#[derive(Default)]
struct ElementState {
    hovered: bool,
}

/// [`EventsProcessor`] stores the elements events states.
///
/// TODO(marc2332): Remove deleted Elements
#[derive(Default)]
pub struct EventsProcessor {
    states: FxHashMap<NodeId, ElementState>,
}

impl EventsProcessor {
    /// Update the Element states given the new events
    pub fn process_events(
        &mut self,
        events_to_emit: Vec<DomEvent>,
        events: &[FreyaEvent],
        event_emitter: &EventEmitter,
    ) -> FxHashMap<String, Vec<(NodeId, FreyaEvent)>> {
        let mut new_events = FxHashMap::<String, Vec<(NodeId, FreyaEvent)>>::default();
        let cursor_was_moved = events.iter().any(|e| e.get_name() == "mouseover");

        for (element, element_state) in self.states.iter_mut() {
            {
                let no_recent_mouse_movement_on_me = events_to_emit
                    .iter()
                    .find_map(|event| {
                        if event.does_move_cursor() && &event.node_id == element {
                            Some(false)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(true);

                let recent_mouse_movement_event = events
                    .iter()
                    .find(|event| {
                        if let FreyaEvent::Mouse { name, .. } = event {
                            does_event_move_cursor(name)
                        } else {
                            false
                        }
                    })
                    .cloned();

                if element_state.hovered && cursor_was_moved && no_recent_mouse_movement_on_me {
                    if let Some(FreyaEvent::Mouse { cursor, button, .. }) =
                        recent_mouse_movement_event
                    {
                        let events = new_events.entry("mouseleave".to_string()).or_default();
                        events.push((
                            *element,
                            FreyaEvent::Mouse {
                                name: "mouseleave".to_string(),
                                cursor,
                                button,
                            },
                        ));

                        // Mark the element as no longer being hovered
                        element_state.hovered = false;
                    }
                }
            }
        }

        // All these events will mark the node as being hovered
        // "mouseover" "mouseenter" "pointerover"  "pointerenter"

        // Emit valid events
        for event in &events_to_emit {
            let id = &event.node_id;

            let should_trigger = match event.name.as_str() {
                name @ "mouseover"
                | name @ "mouseenter"
                | name @ "pointerover"
                | name @ "pointerenter" => {
                    if !self.states.contains_key(id) {
                        self.states.insert(*id, ElementState::default());
                    }

                    let node_state = self.states.get_mut(id).unwrap();

                    if name == "mouseenter" || name == "pointerenter" {
                        // If the event is already being hovered then it's pointless to trigger the movement event
                        !node_state.hovered
                    } else {
                        true
                    }
                }
                _ => true,
            };

            if should_trigger {
                event_emitter.send(event.clone()).unwrap();
            }
        }

        // Update the internal states of elements given the events
        // e.g `mouseover` will mark the element as hovered.
        for event in events_to_emit {
            let id = &event.node_id;
            if does_event_move_cursor(event.name.as_str()) {
                if !self.states.contains_key(id) {
                    self.states.insert(*id, ElementState::default());
                }

                let node_state = self.states.get_mut(id).unwrap();

                node_state.hovered = true;
            }
        }

        new_events
    }
}
