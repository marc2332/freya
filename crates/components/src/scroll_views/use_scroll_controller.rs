use dioxus::prelude::{
    use_hook,
    use_signal,
    CopyValue,
    Readable,
    Signal,
    Writable,
};
use freya_common::NodeReferenceLayout;

#[derive(Default, PartialEq, Eq)]
pub enum ScrollPosition {
    #[default]
    Top,
    Bottom,
    // Specific
}

pub struct ScrollConfig {
    pub direction: String,
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
    pub fn apply_container_size(&mut self, size: &NodeReferenceLayout) {
        let config = self.config.peek();
        if config.initial == ScrollPosition::Bottom
            && !size.inner.is_empty()
            && !*self.applied_init_scroll.peek()
        {
            match config.direction.as_str() {
                "vertical" if *self.y.peek() == 0 => {
                    *self.y.write() = -size.inner.height as i32;
                    self.applied_init_scroll.set(true);
                }
                "horizontal" if *self.x.peek() == 0 => {
                    *self.x.write() = -size.inner.width as i32;
                    self.applied_init_scroll.set(true);
                }
                _ => {}
            }
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
