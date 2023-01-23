use std::collections::HashMap;

use dioxus_core::ElementId;
use euclid::Point2D;
use freya_elements::{events_data::MouseData, Code, Key};
use freya_layers::RenderData;
use glutin::event::MouseButton;
use rustc_hash::FxHashMap;

use crate::{DomEvent, DomEventData};

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
    },
}

#[derive(Default)]
struct ElementState {
    mouseover: bool,
}

/// Some events are not produced directly by the user.
/// The EventsProcessor calculates by comparing previous and current events
/// if new events must be produced.
///
/// For example, mouseleave indicates the user has left the hovering area of
/// a particular element, which previously had to enter that area.
/// At the moment, whether if it has entered or not is defined by the mouseover event.

#[derive(Default)]
pub struct EventsProcessor {
    states: HashMap<ElementId, ElementState>,
}

impl EventsProcessor {
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
