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

/// Maximum distance between two presses of the same combo.
const LOCATION_THRESHOLD: f64 = 5.0;

/// Maximum time between two presses of the same combo.
const MULTI_PRESS_ELAPSED: Duration = Duration::from_millis(500);

/// Turns consecutive presses into double, triple and quadruple presses.
///
/// # Example
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # fn app() -> impl IntoElement {
/// rect().on_pointer_down(|e: Event<PointerEventData>| {
///     if EventsCombos::pressed(e.global_location()).is_double() {
///         println!("Double press");
///     }
/// })
/// # }
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct EventsCombos {
    pub(crate) last_press: State<Option<(Instant, CursorPoint, u8)>>,
}

impl EventsCombos {
    /// Get the app-wide combos state, creating it on first use.
    pub fn get() -> Self {
        match try_consume_root_context() {
            Some(rt) => rt,
            None => {
                let combos = EventsCombos {
                    last_press: State::create_in_scope(None, ScopeId::ROOT),
                };
                provide_context_for_scope_id(combos, ScopeId::ROOT);
                combos
            }
        }
    }

    /// Break the combo when the pointer drags away from the last press.
    pub fn moved(location: CursorPoint) {
        let mut combos = Self::get();
        let dragged_away = matches!(
            &*combos.last_press.read(),
            Some((_, last_location, _)) if last_location.distance_to(location) > LOCATION_THRESHOLD
        );
        if dragged_away {
            combos.last_press.set(None);
        }
    }

    /// Register a press and get its position in the combo.
    pub fn pressed(location: CursorPoint) -> PressEventType {
        let mut combos = Self::get();
        let (event_type, click_count) = match &*combos.last_press.read() {
            Some((inst, last_location, count)) if inst.elapsed() <= MULTI_PRESS_ELAPSED => {
                if last_location.distance_to(location) <= LOCATION_THRESHOLD {
                    match count {
                        1 => (PressEventType::Double, 2),
                        2 => (PressEventType::Triple, 3),
                        3 => (PressEventType::Quadruple, 4),
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

/// Position of a press inside a combo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressEventType {
    /// First press of a combo.
    Single,
    /// Second press of a combo.
    Double,
    /// Third press of a combo.
    Triple,
    /// Fourth and last press of a combo.
    Quadruple,
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

    pub fn is_quadruple(&self) -> bool {
        matches!(self, Self::Quadruple)
    }
}
