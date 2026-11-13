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
struct PendingGesture {
    origin: CursorPoint,
    // Inner scroll views come first.
    candidates: Vec<Candidate>,
}

impl PendingGesture {
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

#[derive(Clone, PartialEq)]
enum Gesture {
    Pending(PendingGesture),
    Scrolling(AccessibilityId),
    TakenOver,
}

#[derive(Clone, Copy, PartialEq)]
struct ScrollTouchGesture(State<Option<Gesture>>);

impl ScrollTouchGesture {
    fn get() -> Self {
        match try_consume_root_context() {
            Some(gesture) => gesture,
            None => {
                let gesture = Self(State::create_in_scope(None, ScopeId::ROOT));
                provide_context_for_scope_id(gesture, ScopeId::ROOT);
                gesture
            }
        }
    }
}

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
    use_hook(|| TouchDrag {
        id,
        gesture: ScrollTouchGesture::get().0,
        last: State::create(None),
    })
}

impl TouchDrag {
    /// Stop touch scrolling for the current gesture until release or cancellation.
    /// Call from a touch handler or long-press callback, without a hook or scroll-view ID.
    pub fn take_over() {
        if let Some(ScrollTouchGesture(mut gesture)) = try_consume_root_context() {
            gesture.set_if_modified(Some(Gesture::TakenOver));
        }
    }

    /// Start a touch using the content and view sizes to find which directions can scroll.
    pub fn press(&mut self, origin: CursorPoint, content: Size2D, viewport: Size2D) {
        let candidate = Candidate {
            id: self.id,
            horizontal: content.width - viewport.width >= 1.0,
            vertical: content.height - viewport.height >= 1.0,
        };
        {
            let mut gesture = self.gesture.write();
            match gesture.as_mut() {
                Some(Gesture::TakenOver) => {}
                Some(Gesture::Pending(pending)) if pending.origin == origin => {
                    if !pending
                        .candidates
                        .iter()
                        .any(|candidate| candidate.id == self.id)
                    {
                        pending.candidates.push(candidate);
                    }
                }
                _ => {
                    *gesture = Some(Gesture::Pending(PendingGesture {
                        origin,
                        candidates: vec![candidate],
                    }));
                }
            }
        }
        self.last.set_if_modified(Some(origin));
    }

    /// Return how far to scroll, or `None` if this view does not have the drag.
    pub fn movement(&mut self, coords: CursorPoint) -> Option<Vector2D> {
        let owner = {
            let mut gesture = self.gesture.write();
            match gesture.as_ref()? {
                Gesture::Pending(pending) => {
                    if !pending
                        .candidates
                        .iter()
                        .any(|candidate| candidate.id == self.id)
                    {
                        return None;
                    }
                    let owner = pending.owner_at(coords)?;
                    *gesture = Some(Gesture::Scrolling(owner));
                    owner
                }
                Gesture::Scrolling(owner) => *owner,
                Gesture::TakenOver => return None,
            }
        };
        if owner != self.id {
            return None;
        }

        let from = (*self.last.peek())?;
        self.last.set_if_modified(Some(coords));
        Some((from - coords).to_f32())
    }

    /// End the touch and return whether this view should release its scrolling drag.
    pub fn release(&mut self) -> bool {
        let (was_dragged, ends_gesture) = match self.gesture.peek().as_ref() {
            Some(Gesture::Scrolling(owner)) => {
                let was_dragged = *owner == self.id;
                // Only the scrolling owner clears it, since other views may not receive the release.
                (was_dragged, was_dragged)
            }
            Some(Gesture::Pending(_) | Gesture::TakenOver) => (false, true),
            None => (false, false),
        };
        if ends_gesture {
            self.gesture.set_if_modified(None);
        }
        self.last.set_if_modified(None);
        was_dragged
    }
}
