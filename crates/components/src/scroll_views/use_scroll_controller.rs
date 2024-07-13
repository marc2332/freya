use std::collections::HashSet;

use dioxus::prelude::{
    current_scope_id,
    schedule_update_any,
    use_drop,
    use_hook,
    use_on_unmount,
    use_signal,
    CopyValue,
    Readable,
    ReadableVecExt,
    ScopeId,
    Signal,
    Writable,
    WritableVecExt,
};

#[derive(Default, PartialEq, Eq)]
pub enum ScrollPosition {
    #[default]
    Start,
    End,
    // Specific
}

#[derive(Default, PartialEq, Eq)]
pub enum ScrollDirection {
    #[default]
    Vertical,
    Horizontal,
}

#[derive(Default)]
pub struct ScrollConfig {
    pub default_vertical_position: ScrollPosition,
    pub default_horizontal_position: ScrollPosition,
}

pub(crate) struct ScrollRequest {
    pub(crate) position: ScrollPosition,
    pub(crate) direction: ScrollDirection,
    pub(crate) init: bool,
    pub(crate) applied_by: HashSet<ScopeId>,
}

impl ScrollRequest {
    pub fn new(position: ScrollPosition, direction: ScrollDirection) -> ScrollRequest {
        ScrollRequest {
            position,
            direction,
            init: false,
            applied_by: HashSet::default(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ScrollController {
    requests_subscribers: Signal<HashSet<ScopeId>>,
    requests: Signal<Vec<ScrollRequest>>,
    x: Signal<i32>,
    y: Signal<i32>,
}

impl From<ScrollController> for (Signal<i32>, Signal<i32>) {
    fn from(val: ScrollController) -> Self {
        (val.x, val.y)
    }
}

impl ScrollController {
    pub fn use_apply(&mut self, width: f32, height: f32) {
        let scope_id = current_scope_id().unwrap();

        if !self.requests_subscribers.peek().contains(&scope_id) {
            self.requests_subscribers.write().insert(scope_id);
        }

        let mut requests_subscribers = self.requests_subscribers;
        use_drop(move || {
            requests_subscribers.write().remove(&scope_id);
        });

        self.requests.write().retain_mut(|request| {
            if request.applied_by.contains(&scope_id) {
                return true;
            }

            match request {
                ScrollRequest {
                    position: ScrollPosition::Start,
                    direction: ScrollDirection::Vertical,
                    ..
                } => {
                    *self.y.write() = 0;
                }
                ScrollRequest {
                    position: ScrollPosition::Start,
                    direction: ScrollDirection::Horizontal,
                    ..
                } => {
                    *self.x.write() = 0;
                }
                ScrollRequest {
                    position: ScrollPosition::End,
                    direction: ScrollDirection::Vertical,
                    init,
                    ..
                } => {
                    if *init && height == 0. {
                        return true;
                    }
                    *self.y.write() = -height as i32;
                }
                ScrollRequest {
                    position: ScrollPosition::End,
                    direction: ScrollDirection::Horizontal,
                    init,
                    ..
                } => {
                    if *init && width == 0. {
                        return true;
                    }
                    *self.x.write() = -width as i32;
                }
            }

            request.applied_by.insert(scope_id);

            *self.requests_subscribers.peek() != request.applied_by
        });
    }

    pub fn scroll_to_x(&mut self, to: i32) {
        self.x.set(to);
    }

    pub fn scroll_to_y(&mut self, to: i32) {
        self.y.set(to);
    }

    pub fn scroll_to(
        &mut self,
        scroll_position: ScrollPosition,
        scroll_direction: ScrollDirection,
    ) {
        self.requests.push(ScrollRequest {
            position: scroll_position,
            direction: scroll_direction,
            init: false,
            applied_by: HashSet::default(),
        });
        let schedule = schedule_update_any();
        for scope_id in self.requests_subscribers.read().iter() {
            schedule(*scope_id);
        }
    }
}

pub fn use_scroll_controller(init: impl FnOnce() -> ScrollConfig) -> ScrollController {
    use_hook(|| {
        let config = init();
        ScrollController {
            x: Signal::new(0),
            y: Signal::new(0),
            requests_subscribers: Signal::new(HashSet::new()),
            requests: Signal::new(vec![
                ScrollRequest {
                    position: config.default_vertical_position,
                    direction: ScrollDirection::Vertical,
                    init: true,
                    applied_by: HashSet::default(),
                },
                ScrollRequest {
                    position: config.default_horizontal_position,
                    direction: ScrollDirection::Horizontal,
                    init: true,
                    applied_by: HashSet::default(),
                },
            ]),
        }
    })
}
