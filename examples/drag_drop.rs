#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fmt::Debug, time::Duration};

use freya::prelude::*;

fn main() {
    launch(app);
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
        DragProvider::<&'static str> {
            rect {
                direction: "horizontal",
                width: "fill",
                height: "fill",
                spacing: "20",
                padding: "20",
                Column {
                    data,
                    state: FoodState::ReallyBad
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Column(data: Signal<Vec<Food>>, state: FoodState) -> Element {
    let move_food = move |food_name: &'static str| {
        let idx = data.iter().enumerate().find_map(|(i, food)| if food.name == food_name { Some(i) } else { None }).unwrap();
        let mut food = data.write().remove(idx);
        food.state = state;
        println!("{idx}");
        data.write().insert(0, food);
    };

    println!("{:?}",  data.read().iter().filter(|food| food.state == FoodState::ReallyBad).collect::<Vec<_>>());

    rsx!(
        DropZone{
            ondrop: move_food,
            rect {
                height: "100%",
                background: "rgb(235, 235, 235)",
                corner_radius: "8",
                padding: "10",
                spacing: "10",
                width: "200",
                for food in data.read().iter() {
                    rect {
                        key: "{food.name}",
                        width: "fill",
                        height: "70",
                        DragZone {
                            hide_while_dragging: true,
                            data: food.name,
                            drag_element: rsx!(
                                rect {
                                    width: "200",
                                    background: "rgb(210, 210, 210)",
                                    corner_radius: "8",
                                    padding: "10",
                                    layer: "-999",
                                    shadow: "0 2 10 2 rgb(0,0,0,0.2)",
                                    label {
                                        "{food.quantity} of {food.name} in {food.state:?} state."
                                    }
                                }
                            ),
                            rect {
                                width: "fill",
                                height: "fill",
                                background: "rgb(210, 210, 210)",
                                corner_radius: "8",
                                padding: "10",
                                label {
                                    "{food.quantity} of {food.name} in {food.state:?} state."
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
