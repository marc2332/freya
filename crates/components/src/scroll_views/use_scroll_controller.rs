use dioxus::prelude::{
    use_hook,
    use_signal,
    CopyValue,
    Readable,
    Signal,
    Writable,
};

#[derive(Default, PartialEq, Eq)]
pub enum ScrollPosition {
    #[default]
    Top,
    Bottom,
    // Specific
}

pub struct ScrollConfig {
    pub initial: ScrollPosition,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ScrollController {
    config: CopyValue<ScrollConfig>,
    x: Signal<i32>,
    y: Signal<i32>,
    applied_init_scroll: Signal<bool>,
}

impl From<ScrollController> for (Signal<i32>, Signal<i32>) {
    fn from(val: ScrollController) -> Self {
        (val.x, val.y)
    }
}

impl ScrollController {
    fn should_apply(&self, size: f32) -> bool {
        let config = self.config.peek();
        config.initial == ScrollPosition::Bottom && size > 0. && !*self.applied_init_scroll.peek()
    }

    pub fn apply_vertical(&mut self, size: f32) {
        if self.should_apply(size) {
            *self.y.write() = -size as i32;
            self.applied_init_scroll.set(true);
        }
    }

    pub fn apply_horizontal(&mut self, size: f32) {
        if self.should_apply(size) {
            *self.x.write() = -size as i32;
            self.applied_init_scroll.set(true);
        }
    }
}

pub fn use_scroll_controller(init: impl FnOnce() -> ScrollConfig) -> ScrollController {
    let config = use_hook(|| CopyValue::new(init()));
    let x = use_signal(|| 0);
    let y = use_signal(|| 0);
    let applied_init_scroll = use_signal(|| false);

    ScrollController {
        config,
        x,
        y,
        applied_init_scroll,
    }
}
