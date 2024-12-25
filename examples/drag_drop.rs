#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    fmt::Debug,
    time::Duration,
};

use freya::prelude::*;

fn main() {
    launch_with_props(app, "Drag and Drop", (800., 600.));
}

#[derive(PartialEq, Clone, Copy)]
pub enum FoodState {
    ReallyBad,
    Meh,
    Normal,
    Amazing,
}

impl Debug for FoodState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FoodState::ReallyBad => f.write_str("really bad"),
            FoodState::Meh => f.write_str("meh"),
            FoodState::Normal => f.write_str("normal"),
            FoodState::Amazing => f.write_str("amazing"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Food {
    name: &'static str,
    state: FoodState,
    quantity: usize,
}

impl Food {
    pub fn new(name: &'static str, quantity: usize, state: FoodState) -> Self {
        Self {
            name,
            state,
            quantity,
        }
    }
}

fn app() -> Element {
    let data = use_signal(|| {
        vec![
            Food::new("Bananas", 7, FoodState::Amazing),
            Food::new("Apples", 12, FoodState::Meh),
            Food::new("Kiwis", 5, FoodState::Normal),
            Food::new("Strawberries", 25, FoodState::Amazing),
            Food::new("Pineapples", 2, FoodState::ReallyBad),
            Food::new("Cherries", 44, FoodState::Meh),
            Food::new("Coconuts", 1, FoodState::ReallyBad),
            Food::new("Blueberries", 70, FoodState::Normal),
            Food::new("Mangos", 9, FoodState::Normal),
            Food::new("Grapes", 57, FoodState::Normal),
            Food::new("Mandarin", 57, FoodState::Meh),
            Food::new("Papaya", 18, FoodState::ReallyBad),
        ]
    });

    rsx!(
        GlobalAnimatedPositionProvider::<&'static str> {
            DragProvider::<&'static str> {
                rect {
                    direction: "horizontal",
                    width: "fill",
                    height: "fill",
                    spacing: "20",
                    padding: "20",
                    content: "flex",
                    Column {
                        data,
                        state: FoodState::ReallyBad
                    }
                    Column {
                        data,
                        state: FoodState::Meh
                    }
                    Column {
                        data,
                        state: FoodState::Normal
                    }
                    Column {
                        data,
                        state: FoodState::Amazing
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Column(data: Signal<Vec<Food>>, state: FoodState) -> Element {
    let move_food = move |food_name: &'static str| {
        let (idx, food) = data
            .iter()
            .enumerate()
            .find_map(|(i, food)| {
                if food.name == food_name {
                    Some((i, food.clone()))
                } else {
                    None
                }
            })
            .unwrap();
        if food.state != state {
            let mut food = data.write().remove(idx);
            food.state = state;
            data.write().insert(0, food);
        }
    };

    rsx!(
        DropZone {
            ondrop: move_food,
            width: "flex(1)",
            height: "fill",
            rect {
                background: "rgb(235, 235, 235)",
                corner_radius: "8",
                padding: "10",
                spacing: "8",
                width: "fill",
                height: "fill",
                for food in data.read().iter().filter(|food| food.state == state) {
                    DragZone {
                        key: "{food.name}",
                        hide_while_dragging: false,
                        data: food.name,
                        drag_element: rsx!(
                            rect {
                                width: "190",
                                height: "auto",
                                background: "rgb(220, 220, 220)",
                                corner_radius: "99",
                                padding: "10",
                                layer: "-999",
                                shadow: "0 2 5 1 rgb(0,0,0,0.10)",
                                label {
                                    "Dragging {food.name}"
                                }
                            }
                        ),
                        GlobalAnimatedPosition {
                            width: "fill",
                            height: "70",
                            function: Function::Elastic,
                            duration: Duration::from_secs(1),
                            id: food.name,
                            Card {
                                food: food.clone()
                            }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn Card(food: Food) -> Element {
    let animation = use_animation(move |ctx| {
        ctx.auto_start(true);
        ctx.with(
            AnimNum::new(0.7, 1.)
                .time(1000)
                .function(Function::Elastic)
                .ease(Ease::Out),
        )
    });

    let scale = animation.get();
    let scale = scale.read().as_f32();

    rsx!(
        rect {
            width: "fill",
            height: "fill",
            background: "rgb(210, 210, 210)",
            corner_radius: "8",
            padding: "10",
            scale: "{scale}",
            label {
                "{food.quantity} of {food.name} in {food.state:?} state."
            }
        }
    )
}
