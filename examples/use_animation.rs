#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{cell::RefCell, time::Duration};

use freya::prelude::*;
use freya_node_state::Parse;
use skia_safe::Color;
use tokio::time::Instant;
use easer::functions::{Linear, Easing};

fn main() {
    launch_with_props(app, "Animation", (400.0, 350.0));
}

fn apply_value(origin: f32, destination: f32,index: i32, time: Duration, ease: Ease, function: Function) -> f32 {
    let (t, b, c, d) = (index as f32, origin, destination - origin, time.as_millis() as f32);
    match function {
        Function::Linear => {
            match ease {
                Ease::In => {
                    Linear::ease_in(t, b, c, d)
                }
                Ease::InOut => {
                    Linear::ease_in_out(t, b, c, d)
                }
                Ease::Out => {
                    Linear::ease_out(t, b, c, d)
                }
            }
        }
    }
}


#[derive(Default, Clone, Copy)]
enum Function {
    #[default]
    Linear
}

#[derive(Default, Clone, Copy)]
enum Ease {
    #[default]
    In,
    Out,
    InOut,
}

struct AnimColor {
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

    // TODO: Accept anything that can be converted into a Duration
    pub fn time(mut self, time: u64) -> Self {
        self.time = Duration::from_millis(time);
        self
    }

    pub fn ease(mut self, ease: Ease) -> Self {
        self.ease = ease;
        self
    }

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
            AnimDirection::Forward => {
                self.value = self.origin
            }
            AnimDirection::Reverse => {
                self.value = self.destination;
            }
        }
    }

    fn is_finished(&self, index: i32, direction: AnimDirection) -> bool {
        match direction {
            AnimDirection::Forward => {
                index > self.time.as_millis() as i32 && self.value.r() >= self.destination.r()  && self.value.g() >= self.destination.g()  && self.value.b() >= self.destination.b()
            }
            AnimDirection::Reverse => {
                index > self.time.as_millis() as i32 && self.value.r() <= self.origin.r()  && self.value.g() <= self.origin.g()  && self.value.b() <= self.origin.b()
            }
        }
        
    }

    fn advance(&mut self, index: i32, direction: AnimDirection) {
        if !self.is_finished(index, direction) {
            let (origin, destination) = match direction {
                AnimDirection::Forward => (self.origin, self.destination),
                AnimDirection::Reverse => (self.destination, self.origin),
            };
            let r = apply_value(origin.r() as f32, destination.r() as f32, index.min(self.time.as_millis() as i32), self.time, self.ease, self.function);
            let g = apply_value(origin.g() as f32, destination.g() as f32, index.min(self.time.as_millis() as i32), self.time, self.ease, self.function);
            let b = apply_value(origin.b() as f32, destination.b() as f32, index.min(self.time.as_millis() as i32), self.time, self.ease, self.function);
            self.value = Color::from_rgb(r as u8, g as u8, b as u8);
        }
    }
}

struct AnimNum {
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

    // TODO: Accept anything that can be converted into a Duration
    pub fn time(mut self, time: u64) -> Self {
        self.time = Duration::from_millis(time);
        self
    }

    pub fn ease(mut self, ease: Ease) -> Self {
        self.ease = ease;
        self
    }

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
            AnimDirection::Forward => {
                self.value = self.origin
            }
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
            self.value = apply_value(origin, destination, index.min(self.time.as_millis() as i32), self.time, self.ease, self.function)
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
struct Context {
    animated_values: Vec<Signal<Box<dyn AnimatedValue>>>
}

impl Context {
    pub fn with(&mut self, animated_value: impl AnimatedValue + 'static) -> ReadOnlySignal<Box<dyn AnimatedValue>> {
        let val: Box<dyn AnimatedValue> = Box::new(animated_value);
        let signal = Signal::new(val);
        self.animated_values.push(signal.clone());
        ReadOnlySignal::new(signal)
    }
}

#[derive(Clone, Copy)]
pub enum AnimDirection {
    Forward,
    Reverse
}

#[derive(PartialEq)]
struct UseAnimator<Animated> {
    value: Animated,
    ctx: Context,
    platform: UsePlatform,
    task: RefCell<Option<Task>>
}

impl<Animated> UseAnimator<Animated> {
    pub fn get(&self) -> &Animated {
        &self.value
    }

    pub fn reverse(&self) {
        self.run(AnimDirection::Reverse)
    }

    pub fn start(&self) {
        self.run(AnimDirection::Forward)
    }

    pub fn run(&self, direction: AnimDirection) {
        let platform = self.platform;
        let mut ticker = platform.new_ticker();
        let mut values = self.ctx.animated_values.clone();

        if let Some(task) = self.task.borrow_mut().take() {
            task.cancel();
        }

        let task = spawn(async move {
            platform.request_animation_frame();

            let mut index = 0;
            let mut prev_frame = Instant::now();

            for value in values.iter_mut() {
                value.write().prepare(direction);
            }

            loop {
                // Wait for the event loop to tick
                ticker.tick().await;
                platform.request_animation_frame();

                index += prev_frame.elapsed().as_millis() as i32;

                if values.iter().all(|value| value.peek().is_finished(index, direction)) {
                    break;
                }

                for value in values.iter_mut() {
                    value.write().advance(index, direction);
                }

                prev_frame = Instant::now();
            }
        });

        *self.task.borrow_mut() = Some(task);
    }
}

fn use_animation<Animated: PartialEq + 'static>(run: impl Fn(&mut Context) -> Animated + 'static) -> ReadOnlySignal<UseAnimator<Animated>> {
    use_memo(move || {
        let mut ctx = Context::default();
        let value = run(&mut ctx);

        UseAnimator {
            value,
            ctx,
            platform: UsePlatform::new(),
            task: RefCell::new(None)
        }
    }).clone()
}

fn app() -> Element {
    let mut toggle = use_signal(|| true);
    let animator = use_animation(|ctx| {
        (
            ctx.with(AnimNum::new(100., 200.).time(500).ease(Ease::InOut)),
            ctx.with(AnimColor::new("rgb(131, 111, 255)", "rgb(255, 167, 50)").time(170).ease(Ease::InOut)),
            ctx.with(AnimNum::new(0., 360.).time(550).ease(Ease::InOut)),
            ctx.with(AnimNum::new(50., 0.).time(550).ease(Ease::InOut))
        )
    });

    let animations = animator.read();
    let (size, color, rotate, radius) = animations.get();

    rsx!(
        rect {
            main_align: "center",
            cross_align: "center",
            height: "100%",
            width: "100%",
            onclick: move |_| {
                if *toggle.peek() {
                    animator.read().start();
                } else {
                    animator.read().reverse();
                }
                toggle.toggle();
            },
            rect {
                width: "{size.read().as_f32()}",
                rotate: "{rotate.read().as_f32()}deg",
                height: "50%",
                background: "{color.read().as_string()}",
                corner_radius: "{radius.read().as_f32()}"
            }
        }
    )
}
