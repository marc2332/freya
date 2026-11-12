use freya_core::prelude::*;
use torin::geometry::{
    CursorPoint,
    Vector2D,
};

/// Distance a touch has to move before it becomes a drag, so taps can reach children
/// (e.g. press a button) without being consumed by drag scrolling.
const DRAG_THRESHOLD: f64 = 2.0;

/// A scroll view under a touch, and the axes its content can be moved along.
#[derive(Clone, Copy, PartialEq)]
struct Candidate {
    id: usize,
    horizontal: bool,
    vertical: bool,
}

/// One touch, from when it lands on scroll views until it is lifted.
#[derive(Clone, PartialEq)]
struct Gesture {
    origin: CursorPoint,
    /// Scroll views under the touch, innermost first: the order `pointerdown` bubbles in.
    candidates: Vec<Candidate>,
    /// The scroll view the drag belongs to, once the touch moved far enough to tell.
    owner: Option<usize>,
}

impl Gesture {
    fn includes(&self, id: usize) -> bool {
        self.candidates.iter().any(|candidate| candidate.id == id)
    }

    /// The innermost scroll view that scrolls along the axis the touch moved on. When none does,
    /// the innermost one that scrolls along the other axis, if the touch moved on that one too.
    fn owner_at(&self, coords: CursorPoint) -> Option<usize> {
        let distance = (self.origin - coords).abs();
        let moved_x = distance.x > DRAG_THRESHOLD;
        let moved_y = distance.y > DRAG_THRESHOLD;
        if !moved_x && !moved_y {
            return None;
        }

        let mostly_vertical = distance.y >= distance.x;
        let along = |candidate: &&Candidate| {
            if mostly_vertical {
                candidate.vertical
            } else {
                candidate.horizontal
            }
        };
        let across = |candidate: &&Candidate| {
            if mostly_vertical {
                candidate.horizontal && moved_x
            } else {
                candidate.vertical && moved_y
            }
        };

        self.candidates
            .iter()
            .find(along)
            .or_else(|| self.candidates.iter().find(across))
            .map(|candidate| candidate.id)
    }
}

/// The touch being dragged in a window, shared by all its scroll views.
#[derive(Clone, Copy, PartialEq)]
struct WindowGesture(State<Option<Gesture>>);

/// Drag scrolling by touch of one scroll view.
///
/// A touch lands on every scroll view under it, and each of them hears every later movement, in no
/// given order. Left to themselves they would all drag at once, and the one that hears a movement
/// first cancels it for the others. So the scroll views of a window agree on one of them per touch:
/// the innermost that can scroll the way the touch moves.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct TouchDrag {
    id: usize,
    gesture: State<Option<Gesture>>,
    /// Where the touch was when this scroll view last followed it.
    last: State<Option<CursorPoint>>,
}

pub(crate) fn use_touch_drag() -> TouchDrag {
    let id = use_id::<WindowGesture>();
    let gesture = use_hook(|| match try_consume_root_context::<WindowGesture>() {
        Some(WindowGesture(gesture)) => gesture,
        None => {
            let gesture = State::create_in_scope(None, ScopeId::ROOT);
            provide_context_for_scope_id(WindowGesture(gesture), ScopeId::ROOT);
            gesture
        }
    });
    let last = use_state(|| None);
    TouchDrag { id, gesture, last }
}

impl TouchDrag {
    /// A touch landed on this scroll view, whose content can move along the given axes.
    pub(crate) fn press(&mut self, origin: CursorPoint, horizontal: bool, vertical: bool) {
        // `pointerdown` bubbles through the scroll views under the touch one after the other: the first
        // to hear it starts the gesture and the rest join it. A gesture that already has an owner, or
        // that started elsewhere, is a previous one that ended without this being told.
        let joined = (*self.gesture.peek())
            .clone()
            .filter(|gesture| gesture.owner.is_none() && gesture.origin == origin);
        let mut gesture = joined.unwrap_or(Gesture {
            origin,
            candidates: Vec::new(),
            owner: None,
        });
        if !gesture.includes(self.id) {
            gesture.candidates.push(Candidate {
                id: self.id,
                horizontal,
                vertical,
            });
        }
        self.gesture.set(Some(gesture));
        self.last.set(None);
    }

    /// The touch moved. Returns how far, when this scroll view is the one being dragged by it.
    pub(crate) fn movement(&mut self, coords: CursorPoint) -> Option<Vector2D> {
        let (origin, owner, decided) = {
            let gesture = self.gesture.peek();
            let gesture = gesture
                .as_ref()
                .filter(|gesture| gesture.includes(self.id))?;
            match gesture.owner {
                Some(owner) => (gesture.origin, owner, true),
                None => (gesture.origin, gesture.owner_at(coords)?, false),
            }
        };
        if !decided && let Some(gesture) = self.gesture.write().as_mut() {
            gesture.owner = Some(owner);
        }
        if owner != self.id {
            return None;
        }

        let from = self.last.peek().unwrap_or(origin);
        self.last.set(Some(coords));
        Some((from - coords).to_f32())
    }

    /// The touch was lifted. Returns whether this scroll view was being dragged by it.
    pub(crate) fn release(&mut self) -> bool {
        let (was_dragged, ends_gesture) = match self.gesture.peek().as_ref() {
            Some(gesture) if gesture.includes(self.id) => {
                let was_dragged = gesture.owner == Some(self.id);
                // The scroll view that was dragged cancels the lift for the others, so it is the one
                // to end the gesture. A touch that dragged nothing is heard lifted by all of them.
                (was_dragged, was_dragged || gesture.owner.is_none())
            }
            _ => (false, false),
        };
        if ends_gesture {
            self.gesture.set(None);
        }
        if self.last.peek().is_some() {
            self.last.set(None);
        }
        was_dragged
    }
}

/// Whether content of size `content` can be moved in a viewport of size `viewport`, by a pixel or more.
pub(crate) fn can_scroll(content: f32, viewport: f32) -> bool {
    content - viewport >= 1.0
}
