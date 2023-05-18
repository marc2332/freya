use std::{any::Any, collections::HashMap, rc::Rc};

use dioxus_core::ElementId;
use freya_common::{Area, Point2D};
use freya_elements::events::{
    keyboard::{Code, Key, Modifiers},
    KeyboardData, MouseData, TouchData, WheelData,
};
use winit::event::{Force, MouseButton, TouchPhase};

use crate::EventEmitter;

/// Events emitted in Freya.
#[derive(Clone, Debug)]
pub enum FreyaEvent {
    /// A Mouse Event.
    Mouse {
        name: &'static str,
        cursor: Point2D,
        button: Option<MouseButton>,
    },
    /// A Wheel event.
    Wheel {
        name: &'static str,
        scroll: Point2D,
        cursor: Point2D,
    },
    /// A Keyboard event.
    Keyboard {
        name: &'static str,
        key: Key,
        code: Code,
        modifiers: Modifiers,
    },
    /// A Touch event.
    Touch {
        name: &'static str,
        location: Point2D,
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

    pub fn set_name(&mut self, new_name: &'static str) {
        match self {
            Self::Mouse { name, .. } => *name = new_name,
            Self::Wheel { name, .. } => *name = new_name,
            Self::Keyboard { name, .. } => *name = new_name,
            Self::Touch { name, .. } => *name = new_name,
        }
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
    pub fn from_freya_event(
        event_name: &str,
        element_id: ElementId,
        event: &FreyaEvent,
        node_area: Option<Area>,
        scale_factor: f64,
    ) -> Self {
        match event {
            FreyaEvent::Mouse { cursor, button, .. } => Self {
                element_id,
                name: event_name.to_string(),
                data: DomEventData::Mouse(MouseData::new(
                    *cursor / scale_factor,
                    (
                        (cursor.x - node_area.unwrap_or_default().min_x() as f64) / scale_factor,
                        (cursor.y - node_area.unwrap_or_default().min_y() as f64) / scale_factor,
                    )
                        .into(),
                    *button,
                )),
            },
            FreyaEvent::Wheel { scroll, .. } => Self {
                element_id,
                name: event_name.to_string(),
                data: DomEventData::Wheel(WheelData::new(scroll.x, scroll.y)),
            },
            FreyaEvent::Keyboard {
                ref key,
                code,
                modifiers,
                ..
            } => Self {
                element_id,
                name: event_name.to_string(),
                data: DomEventData::Keyboard(KeyboardData::new(key.clone(), *code, *modifiers)),
            },
            FreyaEvent::Touch {
                location,
                finger_id,
                phase,
                force,
                ..
            } => DomEvent {
                element_id,
                name: event_name.to_string(),
                data: DomEventData::Touch(TouchData::new(
                    *location,
                    (
                        location.x - node_area.unwrap_or_default().min_x() as f64,
                        location.y - node_area.unwrap_or_default().min_y() as f64,
                    )
                        .into(),
                    *finger_id,
                    *phase,
                    *force,
                )),
            },
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
}

impl DomEventData {
    pub fn any(self) -> Rc<dyn Any> {
        match self {
            DomEventData::Mouse(m) => Rc::new(m),
            DomEventData::Keyboard(k) => Rc::new(k),
            DomEventData::Wheel(w) => Rc::new(w),
            DomEventData::Touch(w) => Rc::new(w),
        }
    }
}

/// Cached state between re-renders
#[derive(Default)]
struct ElementState {
    mouseover: bool,
}

/// [`EventsProcessor`] stores the elements events states.
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

                // Check if the element has been hovered
                for event in &events_to_emit {
                    if event.name == "mouseover" && &event.element_id == element {
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
                                Point2D::default(), // TODO: Use actual locations
                                Point2D::default(), // TODO: Use actual locations
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
            if event.name == "mouseover" {
                let id = &event.element_id;
                if !self.states.contains_key(id) {
                    self.states.insert(*id, ElementState::default());
                }

                let node_state = self.states.get_mut(&event.element_id).unwrap();

                if !node_state.mouseover {
                    event_emitter
                        .send(DomEvent {
                            element_id: *id,
                            name: "mouseenter".to_string(),
                            data: event.data,
                        })
                        .unwrap();
                }

                // Mark the element as being hovered
                node_state.mouseover = true;
            }
        }
    }
}
