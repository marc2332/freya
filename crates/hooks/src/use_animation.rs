use std::time::Duration;

use dioxus_core::prelude::{spawn, use_hook, Task};
use dioxus_hooks::{use_memo, use_reactive, use_signal, Dependency};
use dioxus_signals::{Memo, ReadOnlySignal, Readable, Signal, Writable};
use easer::functions::*;
use freya_engine::prelude::Color;
use freya_node_state::Parse;
use tokio::time::Instant;

use crate::{use_platform, UsePlatform};

pub fn apply_value(
    origin: f32,
    destination: f32,
    index: i32,
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

#[derive(Default, Clone, Copy)]
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

#[derive(Default, Clone, Copy)]
pub enum Ease {
    #[default]
    In,
    Out,
    InOut,
}

/// Animate a color.
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
}

impl AnimatedValue for AnimColor {
    fn time(&self) -> Duration {
        self.time
    }

    fn as_f32(&self) -> f32 {
        panic!("This is not a f32.")
    }

    fn as_string(&self) -> String {
        format!(
            "rgb({}, {}, {})",
            self.value.r(),
            self.value.g(),
            self.value.b()
        )
    }

    fn prepare(&mut self, direction: AnimDirection) {
        match direction {
            AnimDirection::Forward => self.value = self.origin,
            AnimDirection::Reverse => {
                self.value = self.destination;
            }
        }
    }

    fn is_finished(&self, index: i32, direction: AnimDirection) -> bool {
        match direction {
            AnimDirection::Forward => {
                index > self.time.as_millis() as i32
                    && self.value.r() >= self.destination.r()
                    && self.value.g() >= self.destination.g()
                    && self.value.b() >= self.destination.b()
            }
            AnimDirection::Reverse => {
                index > self.time.as_millis() as i32
                    && self.value.r() <= self.origin.r()
                    && self.value.g() <= self.origin.g()
                    && self.value.b() <= self.origin.b()
            }
        }
    }

    fn advance(&mut self, index: i32, direction: AnimDirection) {
        if !self.is_finished(index, direction) {
            let (origin, destination) = match direction {
                AnimDirection::Forward => (self.origin, self.destination),
                AnimDirection::Reverse => (self.destination, self.origin),
            };
            let r = apply_value(
                origin.r() as f32,
                destination.r() as f32,
                index.min(self.time.as_millis() as i32),
                self.time,
                self.ease,
                self.function,
            );
            let g = apply_value(
                origin.g() as f32,
                destination.g() as f32,
                index.min(self.time.as_millis() as i32),
                self.time,
                self.ease,
                self.function,
            );
            let b = apply_value(
                origin.b() as f32,
                destination.b() as f32,
                index.min(self.time.as_millis() as i32),
                self.time,
                self.ease,
                self.function,
            );
            self.value = Color::from_rgb(r as u8, g as u8, b as u8);
        }
    }
}

/// Animate a numeric value.
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
}

impl AnimatedValue for AnimNum {
    fn time(&self) -> Duration {
        self.time
    }

    fn as_f32(&self) -> f32 {
        self.value
    }

    fn as_string(&self) -> String {
        panic!("This is not a String");
    }

    fn prepare(&mut self, direction: AnimDirection) {
        match direction {
            AnimDirection::Forward => self.value = self.origin,
            AnimDirection::Reverse => {
                self.value = self.destination;
            }
        }
    }

    fn is_finished(&self, index: i32, direction: AnimDirection) -> bool {
        match direction {
            AnimDirection::Forward => {
                index > self.time.as_millis() as i32 && self.value >= self.destination
            }
            AnimDirection::Reverse => {
                index > self.time.as_millis() as i32 && self.value <= self.origin
            }
        }
    }

    fn advance(&mut self, index: i32, direction: AnimDirection) {
        if !self.is_finished(index, direction) {
            let (origin, destination) = match direction {
                AnimDirection::Forward => (self.origin, self.destination),
                AnimDirection::Reverse => (self.destination, self.origin),
            };
            self.value = apply_value(
                origin,
                destination,
                index.min(self.time.as_millis() as i32),
                self.time,
                self.ease,
                self.function,
            )
        }
    }
}

pub trait AnimatedValue {
    fn time(&self) -> Duration;

    fn as_f32(&self) -> f32;

    fn as_string(&self) -> String;

    fn prepare(&mut self, direction: AnimDirection);

    fn is_finished(&self, index: i32, direction: AnimDirection) -> bool;

    fn advance(&mut self, index: i32, direction: AnimDirection);
}

#[derive(Default, PartialEq, Clone)]
pub struct Context {
    animated_values: Vec<Signal<Box<dyn AnimatedValue>>>,
    on_finish: OnFinish,
    auto_start: bool,
}

impl Context {
    pub fn with(
        &mut self,
        animated_value: impl AnimatedValue + 'static,
    ) -> ReadOnlySignal<Box<dyn AnimatedValue>> {
        let val: Box<dyn AnimatedValue> = Box::new(animated_value);
        let signal = Signal::new(val);
        self.animated_values.push(signal);
        ReadOnlySignal::new(signal)
    }

    pub fn on_finish(&mut self, on_finish: OnFinish) -> &mut Self {
        self.on_finish = on_finish;
        self
    }

    pub fn auto_start(&mut self, auto_start: bool) -> &mut Self {
        self.auto_start = auto_start;
        self
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

/// Animate your elements. Use [`use_animation`] to use this.
#[derive(PartialEq, Clone)]
pub struct UseAnimator<Animated: PartialEq + Clone + 'static> {
    pub(crate) value_and_ctx: Memo<(Animated, Context)>,
    pub(crate) platform: UsePlatform,
    pub(crate) is_running: Signal<bool>,
    pub(crate) has_run_yet: Signal<bool>,
    pub(crate) task: Signal<Option<Task>>,
}

impl<T: PartialEq + Clone + 'static> Copy for UseAnimator<T> {}

impl<Animated: PartialEq + Clone + 'static> UseAnimator<Animated> {
    /// Get the animated value.
    pub fn get(&self) -> Animated {
        self.value_and_ctx.read().0.clone()
    }

    /// Reset the animation to the default state.
    pub fn reset(&self) {
        let mut task = self.task;

        if let Some(task) = task.write().take() {
            task.cancel();
        }

        for value in &self.value_and_ctx.read().1.animated_values {
            let mut value = *value;
            value.write().prepare(AnimDirection::Forward);
        }
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
        let ctx = &self.value_and_ctx.peek().1;
        let platform = self.platform;
        let mut is_running = self.is_running;
        let mut ticker = platform.new_ticker();
        let mut values = ctx.animated_values.clone();
        let mut has_run_yet = self.has_run_yet;
        let on_finish = ctx.on_finish;
        let mut task = self.task;

        // Cancel previous animations
        if let Some(task) = task.write().take() {
            task.cancel();
        }

        if !self.peek_has_run_yet() {
            *has_run_yet.write() = true;
        }
        is_running.set(true);

        let animation_task = spawn(async move {
            platform.request_animation_frame();

            let mut index = 0;
            let mut prev_frame = Instant::now();

            // Prepare the animations with the the proper direction
            for value in values.iter_mut() {
                value.write().prepare(direction);
            }

            loop {
                // Wait for the event loop to tick
                ticker.tick().await;
                platform.request_animation_frame();

                index += prev_frame.elapsed().as_millis() as i32;

                let is_finished = values
                    .iter()
                    .all(|value| value.peek().is_finished(index, direction));
                if is_finished {
                    if OnFinish::Reverse == on_finish {
                        // Toggle direction
                        direction.toggle();
                    }
                    match on_finish {
                        OnFinish::Restart | OnFinish::Reverse => {
                            index = 0;

                            // Restart the animation
                            for value in values.iter_mut() {
                                value.write().prepare(direction);
                            }
                        }
                        OnFinish::Stop => {
                            // Stop if all the animations are finished
                            break;
                        }
                    }
                }

                // Advance the animations
                for value in values.iter_mut() {
                    value.write().advance(index, direction);
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
///     let animation = use_animation(|ctx| {
///         ctx.auto_start(true);
///         ctx.with(AnimNum::new(0., 100.).time(50))
///     });
///
///     let width = animation.get().read().as_f32();
///
///     rsx!(
///         rect {
///             width: "{width}",
///             height: "100%",
///             background: "blue"
///         }
///     )
/// }
/// ```
///
/// You are not limited to just one animation per call, you can have as many as you want.
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let animation = use_animation(|ctx| {
///         ctx.auto_start(true);
///         (
///             ctx.with(AnimNum::new(0., 100.).time(50)),
///             ctx.with(AnimColor::new("red", "blue").time(50))
///         )
///     });
///
///     let (width, color) = animation.get();
///
///     rsx!(
///         rect {
///             width: "{width.read().as_f32()}",
///             height: "100%",
///             background: "{color.read().as_string()}"
///         }
///     )
/// }
/// ```
///
/// You can also tweak what to do once the animation has finished with [`Context::on_finish`].
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     let animation = use_animation(|ctx| {
///         ctx.on_finish(OnFinish::Restart);
///         (
///             ctx.with(AnimNum::new(0., 100.).time(50)),
///             ctx.with(AnimColor::new("red", "blue").time(50))
///         )
///     });
///
///     let (width, color) = animation.get();
///
///     rsx!(
///         rect {
///             width: "{width.read().as_f32()}",
///             height: "100%",
///             background: "{color.read().as_string()}"
///         }
///     )
/// }
/// ```
///
pub fn use_animation<Animated: PartialEq + Clone + 'static>(
    run: impl Fn(&mut Context) -> Animated + Clone + 'static,
) -> UseAnimator<Animated> {
    let platform = use_platform();
    let is_running = use_signal(|| false);
    let has_run_yet = use_signal(|| false);
    let task = use_signal(|| None);

    let value_and_ctx = use_memo(move || {
        let mut ctx = Context::default();
        (run(&mut ctx), ctx)
    });

    let animator = UseAnimator {
        value_and_ctx,
        platform,
        is_running,
        has_run_yet,
        task,
    };

    use_hook(move || {
        if animator.value_and_ctx.read().1.auto_start {
            animator.run(AnimDirection::Forward);
        }
    });

    animator
}

pub fn use_animation_with_dependencies<Animated: PartialEq + Clone + 'static, D: Dependency>(
    deps: D,
    run: impl Fn(&mut Context, D::Out) -> Animated + 'static,
) -> UseAnimator<Animated>
where
    D::Out: 'static + Clone,
{
    let platform = use_platform();
    let is_running = use_signal(|| false);
    let has_run_yet = use_signal(|| false);
    let task = use_signal(|| None);

    let value_and_ctx = use_memo(use_reactive(deps, move |vals| {
        let mut ctx = Context::default();
        (run(&mut ctx, vals), ctx)
    }));

    let animator = UseAnimator {
        value_and_ctx,
        platform,
        is_running,
        has_run_yet,
        task,
    };

    use_hook(move || {
        if animator.value_and_ctx.read().1.auto_start {
            animator.run(AnimDirection::Forward);
        }
    });

    animator
}
