use std::{
    ops::Deref,
    time::{
        Duration,
        Instant,
    },
};

use async_io::Timer;
use freya_core::prelude::*;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct AnimConfiguration {
    on_finish: OnFinish,
    on_creation: OnCreation,
    on_change: OnChange,
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

    pub fn on_change(&mut self, on_change: OnChange) -> &mut Self {
        self.on_change = on_change;
        self
    }
}

/// Controls the direction of the animation.
#[derive(Clone, Copy, PartialEq)]
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
#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub enum OnFinish {
    /// Does nothing at all.
    #[default]
    Nothing,
    /// Runs the animation in reverse direction.
    Reverse {
        /// Delay before reversing.
        delay: Duration,
    },
    /// Runs the animation in the same direction again.
    Restart {
        /// Delay before restarting.
        delay: Duration,
    },
}

impl OnFinish {
    /// Creates a new [OnFinish::Nothing] variant.
    pub fn nothing() -> Self {
        Self::Nothing
    }

    /// Creates a new [OnFinish::Reverse] variant with no delay.
    pub fn reverse() -> Self {
        Self::Reverse {
            delay: Duration::ZERO,
        }
    }

    /// Creates a new [OnFinish::Reverse] variant with a delay.
    pub fn reverse_with_delay(delay: Duration) -> Self {
        Self::Reverse { delay }
    }

    /// Creates a new [OnFinish::Restart] variant with no delay.
    pub fn restart() -> Self {
        Self::Restart {
            delay: Duration::ZERO,
        }
    }

    /// Creates a new [OnFinish::Restart] variant with a delay.
    pub fn restart_with_delay(delay: Duration) -> Self {
        Self::Restart { delay }
    }
}

/// What to do once the animation gets created.
///
/// By default it is [OnCreation::Nothing]
#[derive(PartialEq, Clone, Copy, Default, Debug)]
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
/// Defaults to [OnChange::Reset].
#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub enum OnChange {
    /// Reset to the initial state.
    #[default]
    Reset,
    /// Set the values to the end of the animation.
    Finish,
    /// Reruns the animation.
    Rerun,
    /// Does nothing at all.
    Nothing,
}

pub trait ReadAnimatedValue: Clone + 'static {
    type Output;
    fn value(&self) -> Self::Output;
}

pub trait AnimatedValue: Clone + Default + 'static {
    fn prepare(&mut self, direction: AnimDirection);

    fn is_finished(&self, index: u128, direction: AnimDirection) -> bool;

    fn advance(&mut self, index: u128, direction: AnimDirection);

    fn finish(&mut self, direction: AnimDirection);

    fn into_reversed(self) -> Self;
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Ease {
    In,
    #[default]
    Out,
    InOut,
}

/// Animate your elements. Use [`use_animation`] to use this.
#[derive(Clone, PartialEq)]
pub struct UseAnimation<Animated: AnimatedValue> {
    pub(crate) animated_value: State<Animated>,
    pub(crate) config: State<AnimConfiguration>,
    pub(crate) is_running: State<bool>,
    pub(crate) has_run_yet: State<bool>,
    pub(crate) task: State<Option<TaskHandle>>,
    pub(crate) last_direction: State<AnimDirection>,
}
impl<T: AnimatedValue> Copy for UseAnimation<T> {}

impl<Animated: AnimatedValue> Deref for UseAnimation<Animated> {
    type Target = State<Animated>;
    fn deref(&self) -> &Self::Target {
        &self.animated_value
    }
}

impl<Animated: AnimatedValue> UseAnimation<Animated> {
    /// Get the animated value.
    pub fn get(&self) -> ReadRef<'static, Animated> {
        self.animated_value.read()
    }

    /// Runs the animation normally.
    pub fn start(&mut self) {
        self.run(AnimDirection::Forward)
    }

    /// Runs the animation in reverse direction.
    pub fn reverse(&mut self) {
        self.run(AnimDirection::Reverse)
    }

    /// Reset the animation with the initial state.
    pub fn reset(&mut self) {
        if let Some(task) = self.task.write().take() {
            task.cancel();
        }

        self.animated_value
            .write()
            .prepare(*self.last_direction.peek());

        *self.has_run_yet.write() = true;
    }

    /// Finish the animation with the final state.
    pub fn finish(&mut self) {
        if let Some(task) = self.task.write().take() {
            task.cancel();
        }

        self.animated_value
            .write()
            .finish(*self.last_direction.peek());

        *self.has_run_yet.write() = true;
    }

    pub fn is_running(&self) -> State<bool> {
        self.is_running
    }

    pub fn has_run_yet(&self) -> State<bool> {
        self.has_run_yet
    }

    /// Run the animation with a given [`AnimDirection`]
    pub fn run(&self, mut direction: AnimDirection) {
        let mut is_running = self.is_running;
        let mut has_run_yet = self.has_run_yet;
        let mut task = self.task;
        let mut last_direction = self.last_direction;

        let on_finish = self.config.peek().on_finish;
        let mut animated_value = self.animated_value;

        last_direction.set(direction);

        // Cancel previous animations
        if let Some(task) = task.write().take() {
            task.cancel();
        }

        let peek_has_run_yet = *self.has_run_yet.peek();

        let mut ticker = RenderingTicker::get();
        let platform = Platform::get();
        let animation_clock = AnimationClock::get();

        let animation_task = spawn(async move {
            platform.send(UserEvent::RequestRedraw);

            let mut index = 0u128;
            let mut prev_frame = Instant::now();

            // Prepare the animations with the the proper direction
            animated_value.write().prepare(direction);

            if !peek_has_run_yet {
                *has_run_yet.write() = true;
            }
            is_running.set(true);

            loop {
                // Wait for the event loop to tick
                ticker.tick().await;

                // Request another redraw to move the animation forward
                platform.send(UserEvent::RequestRedraw);

                let elapsed = animation_clock.correct_elapsed_duration(prev_frame.elapsed());

                index += elapsed.as_millis();

                let is_finished = {
                    let mut animated_value = animated_value.write();
                    let is_finished = animated_value.is_finished(index, direction);
                    // Advance the animations
                    animated_value.advance(index, direction);

                    is_finished
                };

                if is_finished {
                    let delay = match on_finish {
                        OnFinish::Reverse { delay } => {
                            // Toggle direction
                            direction.toggle();
                            delay
                        }
                        OnFinish::Restart { delay } => delay,
                        OnFinish::Nothing => {
                            // Stop if all the animations are finished
                            break;
                        }
                    };

                    if !delay.is_zero() {
                        Timer::after(delay).await;
                    }

                    index = 0;

                    // Restart/reverse the animation
                    animated_value.write().prepare(direction);
                }

                prev_frame = Instant::now();
            }

            is_running.set(false);
            task.write().take();
        });

        // Cancel previous animations
        task.write().replace(animation_task);
    }
}
/// Animate your UI easily.
///
/// [`use_animation`] takes a callback to initialize the animated values and related configuration.
///
/// To animate a group of values at once you can just return a tuple of them.
///
/// Currently supports animating:
/// - Numeric values (e.g width): [AnimNum](crate::prelude::AnimNum)
/// - Colors using [AnimColor](crate::prelude::AnimColor)
/// - Sequential animations: [AnimSequential](crate::prelude::AnimSequential)
/// - Anything as long as you implement the [AnimatedValue] trait.
///
/// For each animated value you will need to specify the duration, optionally an ease function or what type of easing you want.
///
/// # Example
///
/// Here is an example that animates a value from `0.0` to `100.0` in `50` milliseconds.
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use freya::animation::*;
/// fn app() -> impl IntoElement {
///     let animation = use_animation(|conf| {
///         conf.on_creation(OnCreation::Run);
///         AnimNum::new(0., 100.).time(50)
///     });
///
///     let width = animation.get().value();
///
///     rect()
///         .width(Size::px(width))
///         .height(Size::fill())
///         .background(Color::BLUE)
/// }
/// ```
///
/// You are not limited to just one animation per call, you can have as many as you want.
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use freya::animation::*;
/// fn app() -> impl IntoElement {
///     let animation = use_animation(|conf| {
///         conf.on_creation(OnCreation::Run);
///         (
///             AnimNum::new(0., 100.).time(50),
///             AnimColor::new(Color::RED, Color::BLUE).time(50),
///         )
///     });
///
///     let (width, color) = animation.get().value();
///
///     rect()
///         .width(Size::px(width))
///         .height(Size::fill())
///         .background(color)
/// }
/// ```
///
/// You can also tweak what to do once the animation has finished with [`AnimConfiguration::on_finish`].
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use freya::animation::*;
/// fn app() -> impl IntoElement {
///     let animation = use_animation(|conf| {
///         conf.on_finish(OnFinish::restart());
///         // ...
///         # AnimNum::new(0., 1.)
///     });
///
///     // ...
///     # rect()
/// }
/// ```
///
/// You can subscribe your animation to reactive [state](freya_core::prelude::use_state) values, these are considered dependencies.
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use freya::animation::*;
/// fn app() -> impl IntoElement {
///     let value = use_state(|| 100.);
///
///     let animation = use_animation(move |conf| {
///         conf.on_change(OnChange::Rerun);
///         AnimNum::new(0., value()).time(50)
///     });
///
///     // ...
///     # rect()
/// }
/// ```
///
/// You may also use [use_animation_with_dependencies] to pass non-reactive dependencies as well.
pub fn use_animation<Animated: AnimatedValue>(
    mut run: impl 'static + FnMut(&mut AnimConfiguration) -> Animated,
) -> UseAnimation<Animated> {
    use_hook(|| {
        let mut config = State::create(AnimConfiguration::default());
        let mut animated_value = State::create(Animated::default());
        let is_running = State::create(false);
        let has_run_yet = State::create(false);
        let task = State::create(None);
        let last_direction = State::create(AnimDirection::Forward);

        let mut animation = UseAnimation {
            animated_value,
            config,
            is_running,
            has_run_yet,
            task,
            last_direction,
        };

        Effect::create_sync(move || {
            let mut anim_conf = AnimConfiguration::default();
            animated_value.set(run(&mut anim_conf));
            *config.write() = anim_conf;
        });

        Effect::create_sync_with_gen(move |current_gen| match config.read().on_change {
            OnChange::Finish if current_gen > 0 => {
                animation.finish();
            }
            OnChange::Rerun if current_gen > 0 => {
                let last_direction = *animation.last_direction.peek();
                animation.run(last_direction);
            }
            OnChange::Reset if current_gen > 0 => {
                animation.reset();
            }
            _ => {}
        });

        match config.peek().on_creation {
            OnCreation::Run => {
                animation.run(AnimDirection::Forward);
            }
            OnCreation::Finish => {
                animation.finish();
            }
            _ => {}
        }

        animation
    })
}

/// Like [use_animation] but supports passing manual dependencies.
///
/// ```rust, no_run
/// # use freya::prelude::*;
/// # use freya::animation::*;
/// fn app() -> impl IntoElement {
///     # let other_value = 1.;
///     let animation = use_animation_with_dependencies(&other_value, |conf, other_value| {
///         conf.on_change(OnChange::Rerun);
///         AnimNum::new(0., *other_value).time(50)
///     });
///
///     // ...
///     # rect()
/// }
/// ```
pub fn use_animation_with_dependencies<Animated: AnimatedValue, D: 'static + Clone + PartialEq>(
    dependencies: &D,
    mut run: impl 'static + FnMut(&mut AnimConfiguration, &D) -> Animated,
) -> UseAnimation<Animated> {
    let dependencies = use_reactive(dependencies);
    use_hook(|| {
        let mut config = State::create(AnimConfiguration::default());
        let mut animated_value = State::create(Animated::default());
        let is_running = State::create(false);
        let has_run_yet = State::create(false);
        let task = State::create(None);
        let last_direction = State::create(AnimDirection::Forward);

        let mut animation = UseAnimation {
            animated_value,
            config,
            is_running,
            has_run_yet,
            task,
            last_direction,
        };

        Effect::create_sync(move || {
            let dependencies = dependencies.read();
            let mut anim_conf = AnimConfiguration::default();
            animated_value.set(run(&mut anim_conf, &dependencies));
            *config.write() = anim_conf;
        });

        Effect::create_sync_with_gen(move |current_gen| match config.read().on_change {
            OnChange::Finish if current_gen > 0 => {
                animation.finish();
            }
            OnChange::Rerun if current_gen > 0 => {
                let last_direction = *animation.last_direction.peek();
                animation.run(last_direction);
            }
            OnChange::Reset if current_gen > 0 => {
                animation.reset();
            }
            _ => {}
        });

        match config.peek().on_creation {
            OnCreation::Run => {
                animation.run(AnimDirection::Forward);
            }
            OnCreation::Finish => {
                animation.finish();
            }
            _ => {}
        }

        animation
    })
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

               fn into_reversed(self) -> Self {
                    #[allow(non_snake_case)]
                    let ($($type,)*) = self;
                    (
                        $(
                            $type.into_reversed(),
                        )*
                    )
                }
            }
            impl<$($type,)*> ReadAnimatedValue for  ($($type,)*)
            where
                $($type: ReadAnimatedValue,)*
            {
                type Output = (
                    $(
                        <$type as ReadAnimatedValue>::Output,
                    )*
                );
                fn value(&self) -> Self::Output {
                    #[allow(non_snake_case)]
                    let ($($type,)*) = self;
                    (
                        $(
                            $type.value(),
                        )*
                    )
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
    (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
);
