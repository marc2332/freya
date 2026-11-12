use freya_core::prelude::*;
use torin::geometry::{
    CursorPoint,
    Size2D,
    Vector2D,
};

const DRAG_THRESHOLD: f64 = 2.0;

#[derive(Clone, Copy, PartialEq)]
struct Candidate {
    id: AccessibilityId,
    horizontal: bool,
    vertical: bool,
}

#[derive(Clone, PartialEq)]
struct Gesture {
    origin: CursorPoint,
    // Inner scroll views come first.
    candidates: Vec<Candidate>,
    owner: Option<AccessibilityId>,
}

impl Gesture {
    fn includes(&self, id: AccessibilityId) -> bool {
        self.candidates.iter().any(|candidate| candidate.id == id)
    }

    // Try the main drag direction first, then the other direction.
    fn owner_at(&self, coords: CursorPoint) -> Option<AccessibilityId> {
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

// The current touch, shared by the window's scroll views.
#[derive(Clone, Copy, PartialEq)]
struct ScrollTouchGesture(State<Option<Gesture>>);

/// Tracks touch scrolling. When views are nested, the inner view gets the drag
/// if it can scroll in the drag direction.
#[derive(Clone, Copy, PartialEq)]
pub struct TouchDrag {
    id: AccessibilityId,
    gesture: State<Option<Gesture>>,
    last: State<Option<CursorPoint>>,
}

/// Track touch scrolling with the scroll view's accessibility ID.
pub fn use_touch_drag(id: AccessibilityId) -> TouchDrag {
    use_hook(|| TouchDrag::create(id))
}

impl TouchDrag {
    fn create(id: AccessibilityId) -> Self {
        let gesture = match try_consume_root_context::<ScrollTouchGesture>() {
            Some(ScrollTouchGesture(gesture)) => gesture,
            None => {
                let gesture = State::create_in_scope(None, ScopeId::ROOT);
                provide_context_for_scope_id(ScrollTouchGesture(gesture), ScopeId::ROOT);
                gesture
            }
        };
        Self {
            id,
            gesture,
            last: State::create(None),
        }
    }

    /// Start a touch using the content and view sizes to find which directions can scroll.
    pub fn press(&mut self, origin: CursorPoint, content: Size2D, viewport: Size2D) {
        let joined = self
            .gesture
            .peek()
            .as_ref()
            .filter(|gesture| gesture.owner.is_none() && gesture.origin == origin)
            .cloned();
        let mut gesture = joined.unwrap_or(Gesture {
            origin,
            candidates: Vec::new(),
            owner: None,
        });
        if !gesture.includes(self.id) {
            gesture.candidates.push(Candidate {
                id: self.id,
                horizontal: content.width - viewport.width >= 1.0,
                vertical: content.height - viewport.height >= 1.0,
            });
        }
        self.gesture.set(Some(gesture));
        self.last.set(None);
    }

    /// Return how far to scroll, or `None` if this view does not have the drag.
    pub fn movement(&mut self, coords: CursorPoint) -> Option<Vector2D> {
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

    /// End the touch and return whether this view had the drag.
    pub fn release(&mut self) -> bool {
        let (was_dragged, ends_gesture) = match self.gesture.peek().as_ref() {
            Some(gesture) if gesture.includes(self.id) => {
                let was_dragged = gesture.owner == Some(self.id);
                // The view with the drag clears it. Other views may not receive the release.
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
