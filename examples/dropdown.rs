use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let values = use_hook(|| {
        vec![
            "First Option".to_string(),
            "Second Option".to_string(),
            "Rust".to_string(),
        ]
    });
    let mut selected_dropdown = use_signal(|| "First Option".to_string());
    rsx!(
        Dropdown {
            value: selected_dropdown.read().clone(),
            for ch in values {
                DropdownItem {
                    value: ch.clone(),
                    onpress: {
                        to_owned![ch];
                        move |_| selected_dropdown.set(ch.clone())
                    },
                    label { "{ch}" }
                }
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
        MyButton {
            onpress: |_| println!("button pressed"),
            label {
                "This is a button. Apparently, buttons can be pressed."
            }
        }
    )
}

pub struct Hoverable<Animated: AnimatedValue + PartialEq + Clone + 'static> {
    #[allow(
        unused,
        reason = "Consumers don't always need to know the hover state."
    )]
    pub hovered: Signal<bool>,
    pub animation: UseAnimation<Animated>,
    pub onmouseenter: Box<dyn FnMut(Event<MouseData>)>,
    pub onmouseleave: Box<dyn FnMut(Event<MouseData>)>,
}

macro_rules! hoverable {
    ($anim:expr) => {{
        use freya::prelude::*;

        let mut hovered = use_signal(|| false);
        let animation = use_animation($anim);

        let onmouseenter = move |_: Event<MouseData>| {
            if !hovered() {
                animation.run(AnimDirection::Forward);
                hovered.set(true);
            }
        };

        let onmouseleave = move |_: Event<MouseData>| {
            if hovered() {
                hovered.set(false);
                animation.run(AnimDirection::Reverse);
            }
        };

        Hoverable {
            hovered,
            animation,
            onmouseenter: Box::new(onmouseenter),
            onmouseleave: Box::new(onmouseleave),
        }
    }};
}

#[component]
pub fn MyButton(children: Element, onpress: Option<EventHandler<PressEvent>>) -> Element {
    // in the component:

    let mut animation =
        hoverable!(move |_conf| { AnimColor::new("white", "black").ease(Ease::InOut).time(100) });

    rsx!(rect {
        Button {
            onpress: onpress,
           rect {
            background: animation.animation.get().read().read(),
            onmouseenter: animation.onmouseenter,
            onmouseleave: animation.onmouseleave,

             {children}
            }
        }
    })
}
