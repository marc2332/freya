use dioxus_core::prelude::{
    consume_context,
    spawn,
    use_hook,
    Task,
};
use dioxus_hooks::{
    use_memo,
    use_reactive,
    use_signal,
    Dependency,
};
use dioxus_signals::{
    CopyValue,
    Memo,
    ReadOnlySignal,
    Readable,
    Signal,
    Writable,
};
use freya_core::animation_clock::AnimationClock;
use tokio::time::Instant;

use super::AnimatedValue;
use crate::{
    use_platform,
    UsePlatform,
};

#[derive(Default, PartialEq, Clone)]
pub struct AnimConfiguration {
    on_finish: OnFinish,
    auto_start: bool,
    on_deps_change: OnDepsChange,
}

impl AnimConfiguration {
    pub fn on_finish(&mut self, on_finish: OnFinish) -> &mut Self {
        self.on_finish = on_finish;
        self
    }

    pub fn auto_start(&mut self, auto_start: bool) -> &mut Self {
        self.auto_start = auto_start;
        self
    }

    pub fn on_deps_change(&mut self, on_deps_change: OnDepsChange) -> &mut Self {
        self.on_deps_change = on_deps_change;
        self
    }
}

#[derive(Clone)]
pub struct AnimationContext<Animated: AnimatedValue> {
    value: Signal<Animated>,
    conf: AnimConfiguration,
}

impl<Animated: AnimatedValue> PartialEq for AnimationContext<Animated> {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value) && self.conf.eq(&other.conf)
    }
}

/// Controls the direction of the animation.
#[derive(Clone, Copy)]
pub enum AnimDirection {
    Forward,
    Reverse,
}

impl AnimDirection {
    pub fn toggle(&mut self) {
        match self {
            Self::Forward => *self = Self::Reverse,
            Self::Reverse => *self = Self::Forward,
        }
    }
}

/// What to do once the animation finishes. By default it is [`Stop`](OnFinish::Stop)
#[derive(PartialEq, Clone, Copy, Default)]
pub enum OnFinish {
    #[default]
    Stop,
    Reverse,
    Restart,
}

/// What to do once the animation dependencies change. By default it is [`Reset`](OnDepsChange::Reset)
#[derive(PartialEq, Clone, Copy, Default)]
pub enum OnDepsChange {
    #[default]
    Reset,
    Finish,
    Rerun,
}

/// Animate your elements. Use [`use_animation`] to use this.
#[derive(Clone)]
pub struct UseAnimation<Animated: AnimatedValue> {
    pub(crate) context: Memo<AnimationContext<Animated>>,
    pub(crate) platform: UsePlatform,
    pub(crate) animation_clock: CopyValue<AnimationClock>,
    pub(crate) is_running: Signal<bool>,
    pub(crate) has_run_yet: Signal<bool>,
    pub(crate) task: Signal<Option<Task>>,
    pub(crate) last_direction: Signal<AnimDirection>,
}

impl<T: AnimatedValue> PartialEq for UseAnimation<T> {
    fn eq(&self, other: &Self) -> bool {
        self.context.eq(&other.context)
            && self.platform.eq(&other.platform)
            && self.is_running.eq(&other.is_running)
            && self.has_run_yet.eq(&other.has_run_yet)
            && self.task.eq(&other.task)
            && self.last_direction.eq(&other.last_direction)
    }
}

impl<T: AnimatedValue> Copy for UseAnimation<T> {}

impl<Animated: AnimatedValue> UseAnimation<Animated> {
    /// Get the animated value.
    pub fn get(&self) -> ReadOnlySignal<Animated> {
        self.context.read().value.into()
    }

    /// Reset the animation to the default state.
    pub fn reset(&self) {
        let mut has_run_yet = self.has_run_yet;
        let mut task = self.task;

        has_run_yet.set(false);

        if let Some(task) = task.write().take() {
            task.cancel();
        }

        self.context
            .peek()
            .value
            .write_unchecked()
            .prepare(AnimDirection::Forward);
    }

    /// Finish the animation with the final state.
    pub fn finish(&self) {
        let mut task = self.task;

        if let Some(task) = task.write().take() {
            task.cancel();
        }

        self.context
            .peek()
            .value
            .write_unchecked()
            .finish(*self.last_direction.peek());
    }

    /// Checks if there is any animation running.
    pub fn is_running(&self) -> bool {
        *self.is_running.read()
    }

    /// Checks if it has run yet, by subscribing.
    pub fn has_run_yet(&self) -> bool {
        *self.has_run_yet.read()
    }

    /// Checks if it has run yet, doesn't subscribe. Useful for when you just mounted your component.
    pub fn peek_has_run_yet(&self) -> bool {
        *self.has_run_yet.peek()
    }

    /// Runs the animation in reverse direction.
    pub fn reverse(&self) {
        self.run(AnimDirection::Reverse)
    }

    /// Runs the animation normally.
    pub fn start(&self) {
        self.run(AnimDirection::Forward)
    }

    /// Run the animation with a given [`AnimDirection`]
    pub fn run(&self, mut direction: AnimDirection) {
        let context = &self.context.peek();
        let platform = self.platform;
        let mut is_running = self.is_running;
        let mut has_run_yet = self.has_run_yet;
        let mut task = self.task;
        let mut last_direction = self.last_direction;
        let animation_clock = self.animation_clock;

        let on_finish = context.conf.on_finish;
        let mut value = context.value;

        last_direction.set(direction);

        // Cancel previous animations
        if let Some(task) = task.write().take() {
            task.cancel();
        }

        let peek_has_run_yet = self.peek_has_run_yet();
        let mut ticker = platform.new_ticker();

        let animation_task = spawn(async move {
            platform.request_animation_frame();

            let mut index = 0u128;
            let mut prev_frame = Instant::now();

            // Prepare the animations with the the proper direction
            value.write().prepare(direction);

            if !peek_has_run_yet {
                *has_run_yet.write() = true;
            }
            is_running.set(true);

            loop {
                // Wait for the event loop to tick
                ticker.tick().await;

                // Its okay to stop this animation if the value has been dropped
                if value.try_peek().is_err() {
                    break;
                }

                platform.request_animation_frame();

                let elapsed = animation_clock
                    .peek()
                    .correct_elapsed_duration(prev_frame.elapsed());

                index += elapsed.as_millis();

                let is_finished = value.peek().is_finished(index, direction);

                // Advance the animations
                value.write().advance(index, direction);

                prev_frame = Instant::now();

                if is_finished {
                    if OnFinish::Reverse == on_finish {
                        // Toggle direction
                        direction.toggle();
                    }
                    match on_finish {
                        OnFinish::Restart | OnFinish::Reverse => {
                            index = 0;

                            // Restart the animation
                            value.write().prepare(direction);
                        }
                        OnFinish::Stop => {
                            // Stop if all the animations are finished
                            break;
                        }
                    }
                }
            }

            is_running.set(false);
            task.write().take();
        });

        // Cancel previous animations
        task.write().replace(animation_task);
    }
}

/// Animate your elements easily.
///
/// [`use_animation`] takes an callback to initialize the animated values and related configuration.
///
/// To animate a group of values at once you can just return a tuple of them.
///
/// Currently supports animating numeric values (e.g width, padding, rotation, offsets) using [crate::AnimNum] or colors using [crate::AnimColor].
/// For each animated value you will need specify the duration, optionally an ease function or what type of easing you want.
///
/// For animations where you want to animate a value after one another you may use [crate::AnimSequential].
///
/// # Example
///
/// Here is an example that animates a value from `0.0` to `100.0` in `50` milliseconds.
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// fn main() {
///     launch(app);
/// }
///
/// fn app() -> Element {
///     let animation = use_animation(|conf| {
///         conf.auto_start(true);
///         AnimNum::new(0., 100.).time(50)
///     });
///
///     let width = animation.get().read().read();
///
///     rsx!(rect {
///         width: "{width}",
///         height: "100%",
///         background: "blue"
///     })
/// }
/// ```
///
/// You are not limited to just one animation per call, you can have as many as you want.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let animation = use_animation(|conf| {
///         conf.auto_start(true);
///         (
///             AnimNum::new(0., 100.).time(50),
///             AnimColor::new("red", "blue").time(50),
///         )
///     });
///
///     let (width, color) = &*animation.get().read_unchecked();
///
///     rsx!(rect {
///         width: "{width.read()}",
///         height: "100%",
///         background: "{color.read()}"
///     })
/// }
/// ```
///
/// You can also tweak what to do once the animation has finished with [`AnimConfiguration::on_finish`].
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let animation = use_animation(|conf| {
///         conf.on_finish(OnFinish::Restart);
///         (
///             AnimNum::new(0., 100.).time(50),
///             AnimColor::new("red", "blue").time(50),
///         )
///     });
///
///     let (width, color) = &*animation.get().read_unchecked();
///
///     rsx!(rect {
///         width: "{width.read()}",
///         height: "100%",
///         background: "{color.read()}"
///     })
/// }
/// ```
pub fn use_animation<Animated: AnimatedValue>(
    run: impl 'static + Fn(&mut AnimConfiguration) -> Animated,
) -> UseAnimation<Animated> {
    let platform = use_platform();
    let animation_clock = use_animation_clock();
    let is_running = use_signal(|| false);
    let has_run_yet = use_signal(|| false);
    let task = use_signal(|| None);
    let last_direction = use_signal(|| AnimDirection::Reverse);
    let mut prev_value = use_signal::<Option<Signal<Animated>>>(|| None);

    let context = use_memo(move || {
        if let Some(prev_value) = prev_value.take() {
            prev_value.manually_drop();
        }
        let mut conf = AnimConfiguration::default();
        let value = run(&mut conf);
        let value = Signal::new(value);
        prev_value.set(Some(value));
        AnimationContext { value, conf }
    });

    let animation = UseAnimation {
        context,
        platform,
        is_running,
        has_run_yet,
        task,
        last_direction,
        animation_clock,
    };

    use_hook(move || {
        if animation.context.read().conf.auto_start {
            animation.run(AnimDirection::Forward);
        }
    });

    use_memo(move || {
        let context = context.read();
        if *has_run_yet.peek() {
            match context.conf.on_deps_change {
                OnDepsChange::Finish => animation.finish(),
                OnDepsChange::Rerun => {
                    let last_direction = *animation.last_direction.peek();
                    animation.run(last_direction);
                }
                _ => {}
            }
        }
    });

    animation
}

pub fn use_animation_with_dependencies<Animated: PartialEq + AnimatedValue, D: Dependency>(
    deps: D,
    run: impl 'static + Fn(&mut AnimConfiguration, D::Out) -> Animated,
) -> UseAnimation<Animated>
where
    D::Out: 'static + Clone,
{
    let platform = use_platform();
    let animation_clock = use_animation_clock();
    let is_running = use_signal(|| false);
    let has_run_yet = use_signal(|| false);
    let task = use_signal(|| None);
    let last_direction = use_signal(|| AnimDirection::Reverse);
    let mut prev_value = use_signal::<Option<Signal<Animated>>>(|| None);

    let context = use_memo(use_reactive(deps, move |deps| {
        if let Some(prev_value) = prev_value.take() {
            prev_value.manually_drop();
        }
        let mut conf = AnimConfiguration::default();
        let value = run(&mut conf, deps);
        let value = Signal::new(value);
        prev_value.set(Some(value));
        AnimationContext { value, conf }
    }));

    let animation = UseAnimation {
        context,
        platform,
        is_running,
        has_run_yet,
        task,
        last_direction,
        animation_clock,
    };

    use_memo(move || {
        let context = context.read();
        if *has_run_yet.peek() {
            match context.conf.on_deps_change {
                OnDepsChange::Finish => animation.finish(),
                OnDepsChange::Rerun => {
                    animation.run(*animation.last_direction.peek());
                }
                _ => {}
            }
        }
    });

    use_hook(move || {
        if animation.context.read().conf.auto_start {
            animation.run(AnimDirection::Forward);
        }
    });

    animation
}

pub fn use_animation_clock() -> CopyValue<AnimationClock> {
    use_hook(|| CopyValue::new(consume_context()))
}

macro_rules! impl_tuple_call {
    ($(($($type:ident),*)),*) => {
        $(
            impl<$($type,)*> AnimatedValue for ($($type,)*)
            where
                $($type: AnimatedValue,)*
            {
                fn prepare(&mut self, direction: AnimDirection) {
                    #[allow(non_snake_case)]
                    let ($($type,)*) = self;
                    $(
                        $type.prepare(direction);
                    )*
                }

                fn is_finished(&self, index: u128, direction: AnimDirection) -> bool {
                    #[allow(non_snake_case)]
                    let ($($type,)*) = self;
                    $(
                        if !$type.is_finished(index, direction) {
                            return false;
                        }
                    )*
                    true
                }

                fn advance(&mut self, index: u128, direction: AnimDirection) {
                    #[allow(non_snake_case)]
                    let ($($type,)*) = self;
                    $(
                        $type.advance(index, direction);
                    )*
                }

                fn finish(&mut self, direction: AnimDirection) {
                    #[allow(non_snake_case)]
                    let ($($type,)*) = self;
                    $(
                        $type.finish(direction);
                    )*
                }
            }
        )*
    };
}

impl_tuple_call!(
    (T1),
    (T1, T2),
    (T1, T2, T3),
    (T1, T2, T3, T4),
    (T1, T2, T3, T4, T5),
    (T1, T2, T3, T4, T5, T6),
    (T1, T2, T3, T4, T5, T6, T7),
    (T1, T2, T3, T4, T5, T6, T7, T8),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17),
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
);
