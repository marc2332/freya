use dioxus_core::prelude::{
    spawn,
    use_hook,
    Task,
};
use dioxus_hooks::{
    use_effect,
    use_memo,
    use_reactive,
    use_signal,
    Dependency,
};
use dioxus_signals::{
    MappedSignal,
    Readable,
    Signal,
    Writable,
};
use tokio::time::Instant;

use super::AnimatedValue;
use crate::{
    use_platform,
    UsePlatform,
};

#[derive(Default, PartialEq, Clone)]
pub struct AnimConfiguration {
    on_finish: OnFinish,
    on_creation: OnCreation,
    on_deps_change: OnDepsChange,
}

impl AnimConfiguration {
    pub fn on_finish(&mut self, on_finish: OnFinish) -> &mut Self {
        self.on_finish = on_finish;
        self
    }

    pub fn on_creation(&mut self, on_creation: OnCreation) -> &mut Self {
        self.on_creation = on_creation;
        self
    }

    pub fn on_deps_change(&mut self, on_deps_change: OnDepsChange) -> &mut Self {
        self.on_deps_change = on_deps_change;
        self
    }
}

#[derive(Clone)]
pub struct AnimationContext<Animated: AnimatedValue> {
    value: Animated,
    conf: AnimConfiguration,
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

/// What to do once the animation finishes.
///
/// By default it is [OnFinish::Nothing].
#[derive(PartialEq, Clone, Copy, Default)]
pub enum OnFinish {
    /// Does nothing at all.
    #[default]
    Nothing,
    /// Runs the animation in reverse direction.
    Reverse,
    /// Runs the animation in the same direction again.
    Restart,
}

/// What to do once the animation gets created.
///
/// By default it is [OnCreation::Nothing]
#[derive(PartialEq, Clone, Copy, Default)]
pub enum OnCreation {
    /// Does nothing at all.
    #[default]
    Nothing,
    /// Runs the animation.
    Run,
    /// Set the values to the end of the animation. As if it had actually run.
    Finish,
}

/// What to do once the animation dependencies change.
///
/// By default it is [OnDepsChange::Reset]
#[derive(PartialEq, Clone, Copy, Default)]
pub enum OnDepsChange {
    /// Reset to the initial state.
    #[default]
    Reset,
    /// Set the values to the end of the animation.
    Finish,
    /// Reruns the animation.
    Rerun,
}

/// Animate your elements. Use [`use_animation`] to use this.
#[derive(Clone, PartialEq)]
pub struct UseAnimation<Animated: AnimatedValue> {
    pub(crate) context: Signal<Option<AnimationContext<Animated>>>,
    pub(crate) platform: UsePlatform,
    pub(crate) is_running: Signal<bool>,
    pub(crate) has_run_yet: Signal<bool>,
    pub(crate) task: Signal<Option<Task>>,
    pub(crate) last_direction: Signal<AnimDirection>,
}
impl<T: AnimatedValue> Copy for UseAnimation<T> {}

impl<Animated: AnimatedValue> UseAnimation<Animated> {
    /// Get the animated value.
    pub fn get(&self) -> MappedSignal<Animated> {
        self.context.map(|a| &a.as_ref().unwrap().value)
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
            .write_unchecked()
            .as_mut()
            .unwrap()
            .value
            .prepare(AnimDirection::Forward);
    }

    /// Finish the animation with the final state.
    pub fn finish(&self) {
        let mut task = self.task;

        if let Some(task) = task.write().take() {
            task.cancel();
        }

        self.context
            .write_unchecked()
            .as_mut()
            .unwrap()
            .value
            .finish(*self.last_direction.peek());

        *self.has_run_yet.write_unchecked() = true;
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
        let platform = self.platform;
        let mut is_running = self.is_running;
        let mut has_run_yet = self.has_run_yet;
        let mut task = self.task;
        let mut last_direction = self.last_direction;

        let on_finish = self.context.peek().as_ref().unwrap().conf.on_finish;
        let mut context = self.context;

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
            context.write().as_mut().unwrap().value.prepare(direction);

            if !peek_has_run_yet {
                *has_run_yet.write() = true;
            }
            is_running.set(true);

            loop {
                // Wait for the event loop to tick
                ticker.tick().await;

                let Ok(mut context) = context.try_write() else {
                    // Its okay to stop this animation if the context has been dropped
                    break;
                };
                let context = context.as_mut().unwrap();

                platform.request_animation_frame();

                index += prev_frame.elapsed().as_millis();

                let is_finished = context.value.is_finished(index, direction);

                // Advance the animations
                context.value.advance(index, direction);

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
                            context.value.prepare(direction);
                        }
                        OnFinish::Nothing => {
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
///         conf.on_creation(OnCreation::Run);
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
///         conf.on_creation(OnCreation::Run);
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
    let is_running = use_signal(|| false);
    let has_run_yet = use_signal(|| false);
    let task = use_signal(|| None);
    let last_direction = use_signal(|| AnimDirection::Reverse);
    let mut context = use_signal(|| None);

    use_memo(move || {
        let mut conf = AnimConfiguration::default();
        let value = run(&mut conf);
        context.replace(Some(AnimationContext { value, conf }));
    });

    let animation = UseAnimation {
        context,
        platform,
        is_running,
        has_run_yet,
        task,
        last_direction,
    };

    use_effect(move || {
        if *has_run_yet.peek() {
            match context.read().as_ref().unwrap().conf.on_deps_change {
                OnDepsChange::Finish => animation.finish(),
                OnDepsChange::Rerun => {
                    let last_direction = *animation.last_direction.peek();
                    animation.run(last_direction);
                }
                _ => {}
            }
        }
    });

    use_hook(
        move || match animation.context.read().as_ref().unwrap().conf.on_creation {
            OnCreation::Run => {
                animation.run(AnimDirection::Forward);
            }
            OnCreation::Finish => {
                animation.finish();
            }
            _ => {}
        },
    );

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
    let is_running = use_signal(|| false);
    let has_run_yet = use_signal(|| false);
    let task = use_signal(|| None);
    let last_direction = use_signal(|| AnimDirection::Reverse);

    let mut context = use_signal(|| None);

    use_memo(use_reactive(deps, move |deps| {
        let mut conf = AnimConfiguration::default();
        let value = run(&mut conf, deps);
        context.replace(Some(AnimationContext { value, conf }));
    }));

    let animation = UseAnimation {
        context,
        platform,
        is_running,
        has_run_yet,
        task,
        last_direction,
    };

    use_effect(move || {
        if *has_run_yet.peek() {
            match context.read().as_ref().unwrap().conf.on_deps_change {
                OnDepsChange::Finish => animation.finish(),
                OnDepsChange::Rerun => {
                    let last_direction = *animation.last_direction.peek();
                    animation.run(last_direction);
                }
                _ => {}
            }
        }
    });

    use_hook(
        move || match animation.context.peek().as_ref().unwrap().conf.on_creation {
            OnCreation::Run => {
                animation.run(AnimDirection::Forward);
            }
            OnCreation::Finish => {
                animation.finish();
            }
            _ => {}
        },
    );

    animation
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
