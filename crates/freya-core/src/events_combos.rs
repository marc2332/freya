use std::time::{
    Duration,
    Instant,
};

use torin::prelude::CursorPoint;

use crate::{
    integration::ScopeId,
    prelude::{
        State,
        *,
    },
};

#[derive(Clone, Copy, PartialEq)]
pub struct EventsCombos {
    pub(crate) last_press: State<Option<(Instant, CursorPoint, u8)>>,
}

impl EventsCombos {
    pub fn get() -> Self {
        match try_consume_root_context() {
            Some(rt) => rt,
            None => {
                let context_menu_state = EventsCombos {
                    last_press: State::create_in_scope(None, ScopeId::ROOT),
                };
                provide_context_for_scope_id(context_menu_state, ScopeId::ROOT);
                context_menu_state
            }
        }
    }

    pub fn pressed(location: CursorPoint) -> PressEventType {
        let mut combos = Self::get();
        let (event_type, click_count) = match &*combos.last_press.read() {
            Some((inst, last_location, count)) if inst.elapsed() <= MULTI_PRESS_ELAPSED => {
                if last_location.distance_to(location) <= LOCATION_THRESHOLD {
                    match count {
                        1 => (PressEventType::Double, 2),
                        2 => (PressEventType::Triple, 3),
                        _ => (PressEventType::Single, 1),
                    }
                } else {
                    (PressEventType::Single, 1)
                }
            }
            _ => (PressEventType::Single, 1),
        };
        combos
            .last_press
            .set(Some((Instant::now(), location, click_count)));
        event_type
    }
}

const LOCATION_THRESHOLD: f64 = 5.0;
const MULTI_PRESS_ELAPSED: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressEventType {
    Single,
    Double,
    Triple,
}

impl PressEventType {
    pub fn is_single(&self) -> bool {
        matches!(self, Self::Single)
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double)
    }

    pub fn is_triple(&self) -> bool {
        matches!(self, Self::Triple)
    }
}
