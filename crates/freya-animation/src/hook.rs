use std::{
    ops::Deref,
    time::Instant,
};

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
    Reverse,
    /// Runs the animation in the same direction again.
    Restart,
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

                let mut animated_value = animated_value.write();

                let is_finished = animated_value.is_finished(index, direction);

                // Advance the animations
                animated_value.advance(index, direction);

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
                            animated_value.prepare(direction);
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

        Effect::create_with_gen(move |current_gen| match config.read().on_change {
            OnChange::Finish if current_gen > 0 => {
                animation.finish();
            }
            OnChange::Rerun if current_gen > 0 => {
                let last_direction = *animation.last_direction.peek();
                animation.run(last_direction);
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

        Effect::create_with_gen(move |current_gen| match config.read().on_change {
            OnChange::Finish if current_gen > 0 => {
                animation.finish();
            }
            OnChange::Rerun if current_gen > 0 => {
                let last_direction = *animation.last_direction.peek();
                animation.run(last_direction);
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
