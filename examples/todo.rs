#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    fmt::Debug,
    sync::atomic::{
        AtomicI64,
        Ordering,
    },
    time::Duration,
};

use freya::prelude::*;

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_min_size(600., 400.)
            .with_size(1000., 700.)
            .with_title("To Do"),
    );
}

#[derive(PartialEq, Clone, Copy)]
pub enum Priority {
    Low,
    Mid,
    High,
    Urgent,
}

impl Debug for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => f.write_str("Low 💆‍♂️"),
            Self::Mid => f.write_str("Mid 👍"),
            Self::High => f.write_str("High 😨"),
            Self::Urgent => f.write_str("Urgent 🔥"),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    Todo,
    Progress,
    Done,
    Discarded,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Todo => f.write_str("To Do"),
            Self::Progress => f.write_str("Progress"),
            Self::Done => f.write_str("Done"),
            Self::Discarded => f.write_str("Discarded"),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Task {
    id: i64,
    description: String,
    state: State,
    priority: Priority,
}

static ID: AtomicI64 = AtomicI64::new(0);

impl Task {
    pub fn new(description: String, state: State, priority: Priority) -> Self {
        Self {
            id: ID.fetch_add(1, Ordering::Relaxed),
            description,
            state,
            priority,
        }
    }
}

fn app() -> Element {
    use_init_theme(|| DARK_THEME);
    let data = use_signal::<Vec<Task>>(|| {
        vec![
            Task::new(
                "Make a To Do example".to_string(),
                State::Progress,
                Priority::Mid,
            ),
            Task::new(
                "Release Freya v0.3.0".to_string(),
                State::Todo,
                Priority::High,
            ),
            Task::new("Improve docs".to_string(), State::Progress, Priority::High),
            Task::new(
                "Release Freya v0.3.0".to_string(),
                State::Progress,
                Priority::Urgent,
            ),
            Task::new(
                "Release Freya v0.3.0-rc.0".to_string(),
                State::Done,
                Priority::High,
            ),
            Task::new(
                "Rewrite Freya in Java".to_string(),
                State::Discarded,
                Priority::Low,
            ),
            Task::new("Pet my dog".to_string(), State::Todo, Priority::High),
        ]
    });

    rsx!(
        Body {
            DragProvider::<i64> {
                ScrollView {
                    direction: "horizontal",
                    width: "fill",
                    height: "fill",
                    spacing: "20",
                    padding: "20",
                    Column {
                        data,
                        state: State::Todo
                    }
                    Column {
                        data,
                        state: State::Progress
                    }
                    Column {
                        data,
                        state: State::Done
                    }
                    Column {
                        data,
                        state: State::Discarded
                    }
                }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Column(data: Signal<Vec<Task>>, state: State) -> Element {
    let move_task = move |task_id: i64| {
        let (idx, task) = data
            .iter()
            .enumerate()
            .find_map(|(i, task)| {
                if task.id == task_id {
                    Some((i, task.clone()))
                } else {
                    None
                }
            })
            .unwrap();
        if task.state != state {
            let mut task = data.write().remove(idx);
            task.state = state;
            data.write().insert(0, task);
        }
    };

    rsx!(
        DropZone{
            ondrop: move_task,
            width: "225",
            height: "fill",
            rect {
                content: "flex",
                direction: "horizontal",
                margin: "8 0",
                label {
                    width: "flex",
                    font_size: "22",
                    "{state:?}"
                }
                AddTask {
                    data,
                    state
                }
            }
            rect {
                background: "rgb(30, 30, 30)",
                corner_radius: "8",
                padding: "10",
                spacing: "8",
                width: "fill",
                height: "fill",
                for task in data.read().iter().filter(|task| task.state == state) {
                    DragZone {
                        key: "{task.id}",
                        hide_while_dragging: true,
                        data: task.id,
                        drag_element: rsx!(
                            rect {
                                layer: "-999",
                                width: "200",
                                height: "70",
                                Card {
                                    task: task.clone(),
                                }
                            }
                        ),
                        AnimatedPosition {
                            width: "fill",
                            height: "70",
                            function: Function::Elastic,
                            duration: Duration::from_secs(1),
                            Card {
                                task: task.clone(),
                            }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn Card(task: Task) -> Element {
    let animation = use_animation(move |conf| {
        conf.auto_start(true);
        AnimNum::new(0.7, 1.)
            .time(1000)
            .function(Function::Elastic)
            .ease(Ease::Out)
    });

    let scale = animation.get();
    let scale = scale.read();

    rsx!(
        rect {
            width: "fill",
            height: "fill",
            background: "rgb(45, 45, 45)",
            corner_radius: "8",
            padding: "10",
            scale: "{scale.read()}",
            direction: "horizontal",
            content: "flex",
            label {
                width: "flex",
                "{task.description}"
            }
            rect {
                main_align: "center",
                PriorityPill {
                    priority: task.priority
                }
            }
        }
    )
}

#[component]
fn AddTask(data: Signal<Vec<Task>>, state: State) -> Element {
    let mut show_popup = use_signal(|| false);
    let mut description = use_signal(String::new);
    let mut priority = use_signal(|| Priority::Low);

    rsx!(
        if show_popup() {
            Popup {
                theme: theme_with!(PopupTheme {
                    height: "auto".into()
                }),
                oncloserequest: move |_| {
                    show_popup.set(false)
                },
                PopupTitle {
                    text: "Add New Task to '{state:?}'"
                }
                PopupContent {
                    rect {
                        padding: "0 0 20 0",
                        spacing: "10",
                        Input {
                            value: description,
                            placeholder: "Description",
                            width: "fill",
                            onchange: move |txt| {
                                description.set(txt);
                            }
                        }
                        for p in [Priority::Low, Priority::Mid, Priority::High, Priority::Urgent] {
                            Tile {
                                onselect: move |_| priority.set(p),
                                leading: rsx!(
                                    Radio {
                                        selected: *priority.read() == p,
                                    }
                                ),
                                label { "{p:?}" }
                            }
                        }
                        Button {
                            theme: theme_with!(ButtonTheme {
                                width: "fill".into()
                            }),
                            onpress: move |_| {
                                description.write().clear();
                                show_popup.set(false);
                            },
                            label {
                                "Discard"
                            }
                        }
                        FilledButton {
                            theme: theme_with!(ButtonTheme {
                                width: "fill".into()
                            }),
                            onpress: move |_| {
                                let add_description = description();
                                if add_description.trim().is_empty() {
                                    return;
                                }
                                data.write().push(Task::new(add_description, state, priority()));
                                description.write().clear();
                                priority.set(Priority::Low);
                                show_popup.set(false);
                            },
                            label {
                                "Add"
                            }
                        }
                    }
                }
            }
        }
        Button {
            onpress: move |_| show_popup.set(true),
            label {
                "New Task"
            }
        }
    )
}

#[component]
fn PriorityPill(priority: Priority) -> Element {
    rsx!(
        rect {
            background: "rgb(30, 30, 30)",
            corner_radius: "99",
            padding: "4 8",
            label {
                "{priority:?}"
            }
        }
    )
}
