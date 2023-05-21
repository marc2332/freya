use std::{any::Any, collections::HashMap, rc::Rc};

use dioxus_core::ElementId;
use freya_elements::events::{
    keyboard::{Code, Key, Modifiers},
    pointer::PointerType,
    KeyboardData, MouseData, PointerData, TouchData, WheelData,
};
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

/// Event emitted to the DOM.
#[derive(Debug, Clone)]
pub struct DomEvent {
    pub name: String,
    pub element_id: ElementId,
    pub data: DomEventData,
}

impl DomEvent {
    pub fn does_mouse_move(&self) -> bool {
        return ["mouseover", "mouseenter"].contains(&self.name.as_str());
    }

    pub fn from_freya_event(
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
                    DomEventData::Point(PointerData::new(
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
                    element_id,
                    name: event_name,
                    data: event_data,
                }
            }
            FreyaEvent::Wheel { scroll, .. } => Self {
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
                    DomEventData::Point(PointerData::new(
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
    Point(PointerData),
}

impl DomEventData {
    pub fn any(self) -> Rc<dyn Any> {
        match self {
            DomEventData::Mouse(m) => Rc::new(m),
            DomEventData::Keyboard(k) => Rc::new(k),
            DomEventData::Wheel(w) => Rc::new(w),
            DomEventData::Touch(t) => Rc::new(t),
            DomEventData::Point(p) => Rc::new(p),
        }
    }
}

/// Cached state between re-renders
#[derive(Default)]
struct ElementState {
    mouseover: bool,
}

/// [`EventsProcessor`] stores the elements events states.
///
/// TODO(marc2332): Remove deleted Elements
#[derive(Default)]
pub struct EventsProcessor {
    states: HashMap<ElementId, ElementState>,
}

impl EventsProcessor {
    /// Update the Element states given the new events
    pub fn process_events(
        &mut self,
        events_to_emit: Vec<DomEvent>,
        events: &[FreyaEvent],
        event_emitter: &EventEmitter,
    ) {
        let cursor_was_moved = events.iter().any(|e| e.get_name() == "mouseover");

        for (element, state) in self.states.iter_mut() {
            {
                let mut no_recent_mouseover = true;

                for event in &events_to_emit {
                    // The element was nor hovered if there was no movement on this element
                    if event.does_mouse_move() && &event.element_id == element {
                        no_recent_mouseover = false;
                        break;
                    }
                }

                if no_recent_mouseover && state.mouseover && cursor_was_moved {
                    event_emitter
                        .send(DomEvent {
                            element_id: *element,
                            name: "mouseleave".to_string(),
                            data: DomEventData::Mouse(MouseData::new(
                                CursorPoint::default(), // TODO: Use actual locations
                                CursorPoint::default(), // TODO: Use actual locations
                                Some(MouseButton::Left),
                            )),
                        })
                        .unwrap();

                    // Mark the element as no longer being hovered
                    state.mouseover = false;
                }
            }
        }

        for event in events_to_emit {
            let id = &event.element_id;

            match event.name.as_str() {
                "mouseover" | "mouseenter" => {
                    if !self.states.contains_key(id) {
                        self.states.insert(*id, ElementState::default());
                    }

                    let node_state = self.states.get_mut(&event.element_id).unwrap();

                    // Mark the element as being hovered
                    node_state.mouseover = true;
                }
                _ => {}
            }
        }
    }
}
