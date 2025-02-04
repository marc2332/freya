use std::{
    fmt,
    ops::Deref,
    time::Duration,
};

use dioxus_core::prelude::{
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
    Memo,
    ReadOnlySignal,
    Readable,
    Signal,
    Writable,
};
use easer::functions::*;
use freya_core::parsing::Parse;
use freya_engine::prelude::Color;
use tokio::time::Instant;

use crate::{
    use_platform,
    UsePlatform,
};

pub fn apply_value(
    origin: f32,
    destination: f32,
    index: u128,
    time: Duration,
    ease: Ease,
    function: Function,
) -> f32 {
    let (t, b, c, d) = (
        index as f32,
        origin,
        destination - origin,
        time.as_millis() as f32,
    );
    match function {
        Function::Back => match ease {
            Ease::In => Back::ease_in(t, b, c, d),
            Ease::InOut => Back::ease_in_out(t, b, c, d),
            Ease::Out => Back::ease_out(t, b, c, d),
        },
        Function::Bounce => match ease {
            Ease::In => Bounce::ease_in(t, b, c, d),
            Ease::InOut => Bounce::ease_in_out(t, b, c, d),
            Ease::Out => Bounce::ease_out(t, b, c, d),
        },
        Function::Circ => match ease {
            Ease::In => Circ::ease_in(t, b, c, d),
            Ease::InOut => Circ::ease_in_out(t, b, c, d),
            Ease::Out => Circ::ease_out(t, b, c, d),
        },
        Function::Cubic => match ease {
            Ease::In => Cubic::ease_in(t, b, c, d),
            Ease::InOut => Cubic::ease_in_out(t, b, c, d),
            Ease::Out => Cubic::ease_out(t, b, c, d),
        },
        Function::Elastic => match ease {
            Ease::In => Elastic::ease_in(t, b, c, d),
            Ease::InOut => Elastic::ease_in_out(t, b, c, d),
            Ease::Out => Elastic::ease_out(t, b, c, d),
        },
        Function::Expo => match ease {
            Ease::In => Expo::ease_in(t, b, c, d),
            Ease::InOut => Expo::ease_in_out(t, b, c, d),
            Ease::Out => Expo::ease_out(t, b, c, d),
        },
        Function::Linear => match ease {
            Ease::In => Linear::ease_in(t, b, c, d),
            Ease::InOut => Linear::ease_in_out(t, b, c, d),
            Ease::Out => Linear::ease_out(t, b, c, d),
        },
        Function::Quad => match ease {
            Ease::In => Quad::ease_in(t, b, c, d),
            Ease::InOut => Quad::ease_in_out(t, b, c, d),
            Ease::Out => Quad::ease_out(t, b, c, d),
        },
        Function::Quart => match ease {
            Ease::In => Quart::ease_in(t, b, c, d),
            Ease::InOut => Quart::ease_in_out(t, b, c, d),
            Ease::Out => Quart::ease_out(t, b, c, d),
        },
        Function::Sine => match ease {
            Ease::In => Sine::ease_in(t, b, c, d),
            Ease::InOut => Sine::ease_in_out(t, b, c, d),
            Ease::Out => Sine::ease_out(t, b, c, d),
        },
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Function {
    Back,
    Bounce,
    Circ,
    Cubic,
    Elastic,
    Expo,
    #[default]
    Linear,
    Quad,
    Quart,
    Sine,
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Ease {
    In,
    #[default]
    Out,
    InOut,
}

/// Animate a color.
#[derive(Clone, PartialEq)]
pub struct AnimColor {
    origin: Color,
    destination: Color,
    time: Duration,
    ease: Ease,
    function: Function,

    value: Color,
}

impl AnimColor {
    pub fn new(origin: &str, destination: &str) -> Self {
        Self {
            origin: Color::parse(origin).unwrap(),
            destination: Color::parse(destination).unwrap(),
            time: Duration::default(),
            ease: Ease::default(),
            function: Function::default(),

            value: Color::parse(origin).unwrap(),
        }
    }

    /// Set the animation duration using milliseconds. Use `Self::duration` if you want to specify the duration in another form.
    pub fn time(mut self, time: u64) -> Self {
        self.time = Duration::from_millis(time);
        self
    }

    /// Set the animation duration using milliseconds.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.time = duration;
        self
    }

    /// Set the easing type. See `Ease` for all the types.
    pub fn ease(mut self, ease: Ease) -> Self {
        self.ease = ease;
        self
    }

    /// Set the easing function. See `Function` for all the types.
    pub fn function(mut self, function: Function) -> Self {
        self.function = function;
        self
    }

    /// Read the value of the [AnimColor] as a String.
    pub fn read(&self) -> String {
        format!(
            "rgb({}, {}, {}, {})",
            self.value.r(),
            self.value.g(),
            self.value.b(),
            self.value.a()
        )
    }
}

impl AnimatedValue for AnimColor {
    fn prepare(&mut self, direction: AnimDirection) {
        match direction {
            AnimDirection::Forward => self.value = self.origin,
            AnimDirection::Reverse => {
                self.value = self.destination;
            }
        }
    }

    fn is_finished(&self, index: u128, direction: AnimDirection) -> bool {
        match direction {
            AnimDirection::Forward => {
                index > self.time.as_millis()
                    && self.value.r() == self.destination.r()
                    && self.value.g() == self.destination.g()
                    && self.value.b() == self.destination.b()
                    && self.value.a() == self.destination.a()
            }
            AnimDirection::Reverse => {
                index > self.time.as_millis()
                    && self.value.r() == self.origin.r()
                    && self.value.g() == self.origin.g()
                    && self.value.b() == self.origin.b()
                    && self.value.a() == self.origin.a()
            }
        }
    }

    fn advance(&mut self, index: u128, direction: AnimDirection) {
        let (origin, destination) = match direction {
            AnimDirection::Forward => (self.origin, self.destination),
            AnimDirection::Reverse => (self.destination, self.origin),
        };
        let r = apply_value(
            origin.r() as f32,
            destination.r() as f32,
            index.min(self.time.as_millis()),
            self.time,
            self.ease,
            self.function,
        );
        let g = apply_value(
            origin.g() as f32,
            destination.g() as f32,
            index.min(self.time.as_millis()),
            self.time,
            self.ease,
            self.function,
        );
        let b = apply_value(
            origin.b() as f32,
            destination.b() as f32,
            index.min(self.time.as_millis()),
            self.time,
            self.ease,
            self.function,
        );
        let a = apply_value(
            origin.a() as f32,
            destination.a() as f32,
            index.min(self.time.as_millis()),
            self.time,
            self.ease,
            self.function,
        );
        self.value = Color::from_argb(a as u8, r as u8, g as u8, b as u8);
    }

    fn finish(&mut self, direction: AnimDirection) {
        self.advance(self.time.as_millis(), direction);
    }
}

/// Chain a sequence of animated values.
#[derive(Clone)]
pub struct AnimSequential<Animated: AnimatedValue, const N: usize> {
    values: [Animated; N],
    curr_value: usize,
    acc_index: u128,
}

impl<Animated: AnimatedValue, const N: usize> AnimSequential<Animated, N> {
    pub fn new(values: [Animated; N]) -> Self {
        Self {
            values,
            curr_value: 0,
            acc_index: 0,
        }
    }
}

impl<Animated: AnimatedValue, const N: usize> Deref for AnimSequential<Animated, N> {
    type Target = [Animated; N];

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<Animated: AnimatedValue, const N: usize> AnimatedValue for AnimSequential<Animated, N> {
    fn advance(&mut self, index: u128, direction: AnimDirection) {
        if let Some(value) = self.values.get_mut(self.curr_value) {
            let index = index - self.acc_index;
            value.advance(index, direction);

            if value.is_finished(index, direction) {
                self.curr_value += 1;
                self.acc_index += index;
            }
        }
    }

    fn is_finished(&self, index: u128, direction: AnimDirection) -> bool {
        if let Some(value) = self.values.get(self.curr_value) {
            value.is_finished(index, direction)
        } else {
            true
        }
    }

    fn prepare(&mut self, direction: AnimDirection) {
        self.acc_index = 0;
        self.curr_value = 0;
        for val in &mut self.values {
            val.prepare(direction);
        }
    }

    fn finish(&mut self, direction: AnimDirection) {
        for value in &mut self.values {
            value.finish(direction);
        }
    }
}

/// Animate a numeric value.
#[derive(Clone, PartialEq)]
pub struct AnimNum {
    origin: f32,
    destination: f32,
    time: Duration,
    ease: Ease,
    function: Function,

    value: f32,
}

impl AnimNum {
    pub fn new(origin: f32, destination: f32) -> Self {
        Self {
            origin,
            destination,
            time: Duration::default(),
            ease: Ease::default(),
            function: Function::default(),

            value: origin,
        }
    }

    /// Set the animation duration using milliseconds. Use `Self::duration` if you want to specify the duration in another form.
    pub fn time(mut self, time: u64) -> Self {
        self.time = Duration::from_millis(time);
        self
    }

    /// Set the animation duration using milliseconds.
    pub fn duration(mut self, duration: Duration) -> Self {
        self.time = duration;
        self
    }

    /// Set the easing type. See `Ease` for all the types.
    pub fn ease(mut self, ease: Ease) -> Self {
        self.ease = ease;
        self
    }

    /// Set the easing function. See `Function` for all the types.
    pub fn function(mut self, function: Function) -> Self {
        self.function = function;
        self
    }

    /// Read the value of the [AnimNum] as a f32.
    pub fn read(&self) -> f32 {
        self.value
    }
}

impl AnimatedValue for AnimNum {
    fn prepare(&mut self, direction: AnimDirection) {
        match direction {
            AnimDirection::Forward => self.value = self.origin,
            AnimDirection::Reverse => {
                self.value = self.destination;
            }
        }
    }

    fn is_finished(&self, index: u128, direction: AnimDirection) -> bool {
        match direction {
            AnimDirection::Forward => {
                index > self.time.as_millis() && self.value == self.destination
            }
            AnimDirection::Reverse => index > self.time.as_millis() && self.value == self.origin,
        }
    }

    fn advance(&mut self, index: u128, direction: AnimDirection) {
        let (origin, destination) = match direction {
            AnimDirection::Forward => (self.origin, self.destination),
            AnimDirection::Reverse => (self.destination, self.origin),
        };
        self.value = apply_value(
            origin,
            destination,
            index.min(self.time.as_millis()),
            self.time,
            self.ease,
            self.function,
        );
    }

    fn finish(&mut self, direction: AnimDirection) {
        self.advance(self.time.as_millis(), direction);
    }
}

pub trait AnimatedValue: Clone + 'static {
    fn prepare(&mut self, direction: AnimDirection);

    fn is_finished(&self, index: u128, direction: AnimDirection) -> bool;

    fn advance(&mut self, index: u128, direction: AnimDirection);

    fn finish(&mut self, direction: AnimDirection);
}

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

                index += prev_frame.elapsed().as_millis();

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
/// Currently supports animating numeric values (e.g width, padding, rotation, offsets) or also colors, you need specify the duration,
/// and optionally an ease function or what type of easing you want as well.
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
