use freya_core::prelude::*;
use torin::geometry::CursorPoint;

use crate::scrollviews::{
    ScrollPoint,
    ScrollSize,
    ScrollVector,
};

const DRAG_THRESHOLD: f64 = 2.0;

#[derive(Clone, Copy, PartialEq)]
struct Candidate {
    id: AccessibilityId,
    // Room to scroll towards each end.
    to_start: ScrollVector,
    to_end: ScrollVector,
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

    // Try views with room in the drag direction before falling back to either axis.
    fn owner_at(&self, coords: CursorPoint) -> Option<AccessibilityId> {
        let delta = self.origin - coords;
        let distance = delta.abs();
        let moved_x = distance.x > DRAG_THRESHOLD;
        let moved_y = distance.y > DRAG_THRESHOLD;
        if !moved_x && !moved_y {
            return None;
        }

        let mostly_vertical = distance.y >= distance.x;
        let towards_end = if mostly_vertical {
            delta.y > 0.0
        } else {
            delta.x > 0.0
        };
        let towards = |candidate: &&Candidate| {
            let room = if towards_end {
                candidate.to_end
            } else {
                candidate.to_start
            };
            (if mostly_vertical { room.y } else { room.x }) >= 1.0
        };
        let along = |candidate: &&Candidate| {
            let room = candidate.to_start + candidate.to_end;
            (if mostly_vertical { room.y } else { room.x }) >= 1.0
        };
        let across = |candidate: &&Candidate| {
            let room = candidate.to_start + candidate.to_end;
            if mostly_vertical {
                room.x >= 1.0 && moved_x
            } else {
                room.y >= 1.0 && moved_y
            }
        };

        self.candidates
            .iter()
            .find(towards)
            .or_else(|| self.candidates.iter().find(along))
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
    // This scroll view's ID.
    id: AccessibilityId,
    // The touch shared by the window's scroll views.
    gesture: State<Option<Gesture>>,
    // The last touch position used by this view.
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

    /// Start a touch using the scroll position and sizes to check where this view can scroll.
    pub fn press(
        &mut self,
        origin: CursorPoint,
        position: ScrollPoint,
        content: ScrollSize,
        viewport: ScrollSize,
    ) {
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
            let room = content - viewport;
            gesture.candidates.push(Candidate {
                id: self.id,
                to_start: (position.x, position.y).into(),
                to_end: (room.width - position.x, room.height - position.y).into(),
            });
        }
        self.gesture.set_if_modified(Some(gesture));
        self.last.set_if_modified(None);
    }

    /// Return how far to scroll, or `None` if this view does not have the drag.
    pub fn movement(&mut self, coords: CursorPoint) -> Option<ScrollVector> {
        let (origin, owner) = {
            let mut gesture = self.gesture.write();
            let gesture = gesture
                .as_mut()
                .filter(|gesture| gesture.includes(self.id))?;
            let owner = match gesture.owner {
                Some(owner) => owner,
                None => {
                    let owner = gesture.owner_at(coords)?;
                    gesture.owner = Some(owner);
                    owner
                }
            };
            (gesture.origin, owner)
        };
        if owner != self.id {
            return None;
        }

        let from = self.last.peek().unwrap_or(origin);
        self.last.set_if_modified(Some(coords));
        Some(from - coords)
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
            self.gesture.set_if_modified(None);
        }
        self.last.set_if_modified(None);
        was_dragged
    }
}
