use std::{any::Any, collections::HashMap, rc::Rc};

use dioxus_core::ElementId;
use euclid::{Length, Point2D};
use freya_common::NodeArea;
use freya_elements::{
    events_data::MouseData, Code, Key, KeyboardData, Modifiers, TouchData, WheelData,
};
use freya_layout::RenderData;
use rustc_hash::FxHashMap;
use winit::event::MouseButton;

/// Events emitted in Freya.
#[derive(Clone, Debug)]
pub enum FreyaEvent {
    /// A Mouse Event.
    Mouse {
        name: &'static str,
        cursor: (f64, f64),
        button: Option<MouseButton>,
    },
    /// A Wheel event.
    Wheel {
        name: &'static str,
        scroll: (f64, f64),
        cursor: (f64, f64),
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
        location: (f64, f64),
        finger_id: u64,
    },
}

impl FreyaEvent {
    pub fn get_name(&self) -> &'static str {
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
        node_area: Option<NodeArea>,
    ) -> Self {
        match event {
            FreyaEvent::Mouse { cursor, button, .. } => Self {
                element_id,
                name: event_name.to_string(),
                data: DomEventData::Mouse(MouseData::new(
                    Point2D::from_lengths(Length::new(cursor.0), Length::new(cursor.1)),
                    Point2D::from_lengths(
                        Length::new(cursor.0 - node_area.unwrap_or_default().x as f64),
                        Length::new(cursor.1 - node_area.unwrap_or_default().y as f64),
                    ),
                    *button,
                )),
            },
            FreyaEvent::Wheel { scroll, .. } => Self {
                element_id,
                name: event_name.to_string(),
                data: DomEventData::Wheel(WheelData::new(scroll.0, scroll.1)),
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
                ..
            } => DomEvent {
                element_id,
                name: event_name.to_string(),
                data: DomEventData::Touch(TouchData::new(
                    Point2D::from_lengths(Length::new(location.0), Length::new(location.1)),
                    Point2D::from_lengths(
                        Length::new(location.0 - node_area.unwrap_or_default().x as f64),
                        Length::new(location.1 - node_area.unwrap_or_default().y as f64),
                    ),
                    *finger_id,
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

/// [`EventsProcessor`] calculates new events based on past and new events.
///
/// For example, `mouseleave` indicates the user has left the hovering area of
/// a particular element, which previously had to enter that area.
/// At the moment, whether if it has entered or not is defined by the `mouseover` event.
#[derive(Default)]
pub struct EventsProcessor {
    states: HashMap<ElementId, ElementState>,
}

impl EventsProcessor {
    /// Calculate new events according to the last leap and this one
    pub fn process_events_batch(
        &mut self,
        events_to_emit: Vec<DomEvent>,
        events_filtered: FxHashMap<&str, Vec<(RenderData, FreyaEvent)>>,
    ) -> Vec<DomEvent> {
        let mut new_events = Vec::new();

        for (element, state) in self.states.iter_mut() {
            // Process mouseover events
            {
                let mut no_recent_mouseover = true;

                // Check any mouse event at all
                for event in &events_to_emit {
                    if event.name == "mouseover" && &event.element_id == element {
                        no_recent_mouseover = false;
                        break;
                    }
                }

                let mouseover_events = events_filtered.get("mouseover");

                let cursor_was_moved = mouseover_events.is_some();

                // `no_recent_mouseover` means that the element was not hovered in the latest check
                // and `cursor_was_moved` indicates the mouse was moved in the latest check
                // therefore proving the mouse has moved outside the element area, therefore
                // the `mouseleave` event must be thrown

                if no_recent_mouseover && state.mouseover && cursor_was_moved {
                    // And also at least one mouseover event ocurred
                    new_events.push(DomEvent {
                        element_id: *element,
                        name: "mouseleave".to_string(),
                        data: DomEventData::Mouse(MouseData::new(
                            Point2D::default(),
                            Point2D::default(),
                            Some(MouseButton::Left),
                        )),
                    });

                    // Indicate the element is no longer being hovered
                    state.mouseover = false;
                }
            }
        }

        for event in &events_to_emit {
            if event.name == "mouseover" {
                let id = &event.element_id;
                if !self.states.contains_key(id) {
                    self.states.insert(*id, ElementState::default());
                }

                let node_state = self.states.get_mut(&event.element_id).unwrap();
                node_state.mouseover = true;
            }
        }

        new_events
    }
}
