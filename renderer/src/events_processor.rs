use std::{collections::HashMap, sync::Arc};

use dioxus_core::{EventPriority, UserEvent};
use dioxus_html::{
    geometry::{euclid::Point2D, Coordinates},
    input_data::{keyboard_types::Modifiers, MouseButton},
    on::MouseData,
};
use enumset::enum_set;
use layers_engine::NodeData;

use crate::RendererRequest;

#[derive(Default)]
pub struct EventsProcessor {
    events: Vec<UserEvent>,
}

impl EventsProcessor {
    pub fn process_events_batch(
        &mut self,
        events: Vec<UserEvent>,
        events_filtered: HashMap<&str, Vec<(NodeData, RendererRequest)>>,
    ) -> Vec<UserEvent> {
        let mut new_events = Vec::new();

        for saved_event in &self.events {
            if saved_event.name == "mouseover" {
                let mut found = false;
                for event in &events {
                    if event.name == "mouseover"
                        && event.element.unwrap() == saved_event.element.unwrap()
                    {
                        found = true;
                    }
                }

                let mouseover_events = events_filtered.get("mouseover");

                let at_least_cursor_was_moved = mouseover_events.is_some();

                if !found && at_least_cursor_was_moved {
                    // And also at least one mouseover event ocurred
                    new_events.push(UserEvent {
                        scope_id: None,
                        priority: EventPriority::Medium,
                        element: Some(saved_event.element.unwrap()),
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
                }
            }
        }

        self.events.retain(|event| {
            let mut keep = true;
            for ev in &new_events {
                if ev.name == "mouseleave"
                    && ev.element == event.element
                    && event.name == "mouseover"
                {
                    keep = false;
                }
            }
            for ev in &events {
                if ev.name == "mouseover"
                    && ev.element == event.element
                    && event.name == "mouseover"
                {
                    keep = false;
                }
            }
            keep
        });

        for event in &events {
            if event.name == "mouseover" {
                self.events.push(event.clone());
            }
        }

        new_events
    }
}
