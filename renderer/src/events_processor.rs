use std::{collections::HashMap, sync::Arc};

use dioxus_core::{EventPriority, GlobalNodeId, UserEvent};
use dioxus_html::{
    geometry::{euclid::Point2D, Coordinates},
    input_data::{keyboard_types::Modifiers, MouseButton},
    on::MouseData,
};
use enumset::enum_set;
use freya_layers::RenderData;

use crate::RendererRequest;

#[derive(Default)]
struct ElementState {
    mouseover: bool,
}

/// Some events are not produced directly by the user.
/// The EventsProcessor calculates by comparing previous and current events
/// if new events must be produced.
///
/// For example, mouseleave indicates the the user has left the hovering area of
/// a particular element, which previously had to enter that area.
/// At the moment, whether if it has entered or not is defined by the mouseover event.

#[derive(Default)]
pub struct EventsProcessor {
    states: HashMap<GlobalNodeId, ElementState>,
}

impl EventsProcessor {
    pub fn process_events_batch(
        &mut self,
        events_to_emit: Vec<UserEvent>,
        events_filtered: HashMap<&str, Vec<(RenderData, RendererRequest)>>,
    ) -> Vec<UserEvent> {
        let mut new_events = Vec::new();

        for (element, state) in self.states.iter_mut() {
            // Process mouseover events
            {
                let mut no_recent_mouseover = true;

                // Check any mouse event at all
                for event in &events_to_emit {
                    if event.name == "mouseover" && &event.element.unwrap() == element {
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
                    new_events.push(UserEvent {
                        scope_id: None,
                        priority: EventPriority::Medium,
                        element: Some(*element),
                        name: "mouseleave",
                        bubbles: false,
                        data: Arc::new(MouseData::new(
                            Coordinates::new(
                                Point2D::default(),
                                Point2D::default(),
                                Point2D::default(),
                                Point2D::default(),
                            ),
                            Some(MouseButton::Primary),
                            enum_set! {MouseButton::Primary},
                            Modifiers::empty(),
                        )),
                    });

                    // Indicate the element is no longer being hovered
                    state.mouseover = false;
                }
            }
        }

        for event in &events_to_emit {
            if event.name == "mouseover" {
                let id = &event.element.unwrap();
                if !self.states.contains_key(id) {
                    self.states.insert(*id, ElementState::default());
                }

                let node_state = self.states.get_mut(&event.element.unwrap()).unwrap();
                node_state.mouseover = true;
            }
        }

        new_events
    }
}
