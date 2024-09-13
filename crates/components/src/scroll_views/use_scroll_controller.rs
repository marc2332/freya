use std::collections::HashSet;

use dioxus::prelude::{
    current_scope_id,
    schedule_update_any,
    use_drop,
    use_hook,
    Readable,
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

pub struct ScrollRequest {
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
    pub fn new(x: i32, y: i32, initial_requests: Vec<ScrollRequest>) -> Self {
        Self {
            x: Signal::new(x),
            y: Signal::new(y),
            requests_subscribers: Signal::new(HashSet::new()),
            requests: Signal::new(initial_requests),
        }
    }

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
        self.requests
            .push(ScrollRequest::new(scroll_position, scroll_direction));
        let schedule = schedule_update_any();
        for scope_id in self.requests_subscribers.read().iter() {
            schedule(*scope_id);
        }
    }
}

pub fn use_scroll_controller(init: impl FnOnce() -> ScrollConfig) -> ScrollController {
    use_hook(|| {
        let config = init();
        ScrollController::new(
            0,
            0,
            vec![
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
            ],
        )
    })
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn controlled_scroll_view() {
        fn scroll_view_app() -> Element {
            let mut scroll_controller = use_scroll_controller(|| ScrollConfig {
                default_vertical_position: ScrollPosition::End,
                ..Default::default()
            });

            rsx!(
                ScrollView {
                    scroll_controller,
                    Button {
                        onclick: move |_| {
                            scroll_controller.scroll_to(ScrollPosition::End, ScrollDirection::Vertical);
                        },
                        label {
                            "Scroll Down"
                        }
                    }
                    rect {
                        height: "200",
                        width: "200",
                    },
                    rect {
                        height: "200",
                        width: "200",
                    },
                    rect {
                        height: "200",
                        width: "200",
                    }
                    rect {
                        height: "200",
                        width: "200",
                    }
                    Button {
                        onclick: move |_| {
                            scroll_controller.scroll_to(ScrollPosition::Start, ScrollDirection::Vertical);
                        },
                        label {
                            "Scroll up"
                        }
                    }
                }
            )
        }

        let mut utils = launch_test(scroll_view_app);
        let root = utils.root();
        let content = root.get(0).get(0).get(0);
        utils.wait_for_update().await;

        // Only the last three items are visible
        assert!(!content.get(1).is_visible());
        assert!(content.get(2).is_visible());
        assert!(content.get(3).is_visible());
        assert!(content.get(4).is_visible());

        // Click on the button to scroll up
        utils.click_cursor((15., 480.)).await;

        // Only the first three items are visible
        assert!(content.get(1).is_visible());
        assert!(content.get(2).is_visible());
        assert!(content.get(3).is_visible());
        assert!(!content.get(4).is_visible());

        // Click on the button to scroll down
        utils.click_cursor((15., 15.)).await;

        // Only the first three items are visible
        assert!(!content.get(1).is_visible());
        assert!(content.get(2).is_visible());
        assert!(content.get(3).is_visible());
        assert!(content.get(4).is_visible());
    }
}
