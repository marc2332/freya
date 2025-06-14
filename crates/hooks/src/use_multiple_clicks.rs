use std::time::{
    Duration,
    Instant,
};

use dioxus_hooks::use_signal;
use dioxus_signals::{
    ReadOnlySignal,
    Readable,
    Signal,
    Writable,
};

#[derive(Clone, Copy)]
pub struct UseMultipleClicks {
    stack: Signal<usize>,
    started: Signal<Instant>,

    /// After how many seconds since the last click should a group of clicks be discarded.
    time_treshold: Duration,
    /// At what amount of clicks should a group of clicks be discarded.
    amount_treshold: usize,
}

impl UseMultipleClicks {
    pub fn clicked(&mut self) {
        if self.started.read().elapsed() >= self.time_treshold
            || *self.stack.read() >= self.amount_treshold
        {
            *self.stack.write() = 0;
            *self.started.write() = Instant::now()
        }

        *self.stack.write() += 1;
    }

    pub fn len(&self) -> ReadOnlySignal<usize> {
        self.stack.into()
    }
}

/// Easily count an amount of clicks in a given time frame or amount quantity.
///
/// ### Example:
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let mut clicks = use_multiple_clicks(Duration::from_secs(1), 3);
///
///     rsx!(
///         rect {
///             onclick: move |_| {
///                 clicks.clicked();
///             },
///             height: "100%",
///             width: "100%",
///             main_align: "center",
///             cross_align: "center",
///             background: "rgb(0, 119, 182)",
///             color: "white",
///             label {
///                 font_size: "75",
///                 "{clicks.len()}"
///             }
///         }
///     )
/// }
/// ```
pub fn use_multiple_clicks(time_treshold: Duration, amount_treshold: usize) -> UseMultipleClicks {
    let stack = use_signal(|| 0);
    let started = use_signal(Instant::now);

    UseMultipleClicks {
        stack,
        started,
        time_treshold,
        amount_treshold,
    }
}
