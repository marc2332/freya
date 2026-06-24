use freya_core::prelude::*;
use torin::prelude::Direction;

#[derive(Default, PartialEq, Eq)]
pub enum ScrollPosition {
    #[default]
    Start,
    End,
}

#[derive(Default)]
pub struct ScrollConfig {
    pub default_vertical_position: ScrollPosition,
    pub default_horizontal_position: ScrollPosition,
}

pub struct ScrollRequest {
    pub(crate) position: ScrollPosition,
    pub(crate) direction: Direction,
    pub(crate) init: bool,
}

impl ScrollRequest {
    pub fn new(position: ScrollPosition, direction: Direction) -> ScrollRequest {
        ScrollRequest {
            position,
            direction,
            init: false,
        }
    }
}

pub enum ScrollEvent {
    X(i32),
    Y(i32),
}

#[derive(PartialEq, Clone, Copy)]
pub struct ScrollController {
    notifier: State<()>,
    requests: State<Vec<ScrollRequest>>,
    on_scroll: State<Callback<ScrollEvent, bool>>,
    get_scroll: State<Callback<(), (i32, i32)>>,
}

impl From<ScrollController> for (i32, i32) {
    fn from(val: ScrollController) -> Self {
        val.get_scroll.read().call(())
    }
}

impl ScrollController {
    pub fn new(x: i32, y: i32, initial_requests: Vec<ScrollRequest>) -> Self {
        let mut scroll = State::create((x, y));
        Self {
            notifier: State::create(()),
            requests: State::create(initial_requests),
            on_scroll: State::create(Callback::new(move |ev| {
                let current = *scroll.read();
                match ev {
                    ScrollEvent::X(x) => {
                        scroll.write().0 = x;
                    }
                    ScrollEvent::Y(y) => {
                        scroll.write().1 = y;
                    }
                }
                current != *scroll.read()
            })),
            get_scroll: State::create(Callback::new(move |_| *scroll.read())),
        }
    }
    pub fn managed(
        notifier: State<()>,
        requests: State<Vec<ScrollRequest>>,
        on_scroll: State<Callback<ScrollEvent, bool>>,
        get_scroll: State<Callback<(), (i32, i32)>>,
    ) -> Self {
        Self {
            notifier,
            requests,
            on_scroll,
            get_scroll,
        }
    }

    pub fn use_apply(&mut self, width: f32, height: f32) {
        let _ = self.notifier.read();
        for request in self.requests.write().drain(..) {
            match request {
                ScrollRequest {
                    position: ScrollPosition::Start,
                    direction: Direction::Vertical,
                    ..
                } => {
                    self.on_scroll.write().call(ScrollEvent::Y(0));
                }
                ScrollRequest {
                    position: ScrollPosition::Start,
                    direction: Direction::Horizontal,
                    ..
                } => {
                    self.on_scroll.write().call(ScrollEvent::X(0));
                }
                ScrollRequest {
                    position: ScrollPosition::End,
                    direction: Direction::Vertical,
                    init,
                    ..
                } => {
                    if init && height == 0. {
                        continue;
                    }
                    let (_x, y) = self.get_scroll.read().call(());
                    self.on_scroll
                        .write()
                        .call(ScrollEvent::Y(y - height as i32));
                }
                ScrollRequest {
                    position: ScrollPosition::End,
                    direction: Direction::Horizontal,
                    init,
                    ..
                } => {
                    if init && width == 0. {
                        continue;
                    }

                    let (x, _y) = self.get_scroll.read().call(());
                    self.on_scroll
                        .write()
                        .call(ScrollEvent::X(x - width as i32));
                }
            }
        }
    }

    pub fn scroll_to_x(&mut self, to: i32) -> bool {
        self.on_scroll.write().call(ScrollEvent::X(to))
    }

    pub fn scroll_to_y(&mut self, to: i32) -> bool {
        self.on_scroll.write().call(ScrollEvent::Y(to))
    }

    pub fn scroll_to(&mut self, scroll_position: ScrollPosition, scroll_direction: Direction) {
        self.requests
            .write()
            .push(ScrollRequest::new(scroll_position, scroll_direction));
        self.notifier.write();
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
                    direction: Direction::Vertical,
                    init: true,
                },
                ScrollRequest {
                    position: config.default_horizontal_position,
                    direction: Direction::Horizontal,
                    init: true,
                },
            ],
        )
    })
}
