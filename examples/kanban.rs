#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::{
    animation::*,
    prelude::*,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(900., 600.)))
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(PartialEq, Clone, Debug)]
struct Task {
    id: usize,
    title: String,
    status: TaskStatus,
}

impl Task {
    pub fn new(id: usize, title: String, status: TaskStatus) -> Self {
        Self { id, title, status }
    }
}

#[derive(PartialEq)]
struct Card(Task);
impl Component for Card {
    fn render(&self) -> impl IntoElement {
        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Run);
            AnimNum::new(0.8, 1.)
                .time(500)
                .function(Function::Expo)
                .ease(Ease::Out)
        });

        let scale = animation.read().value();

        rect()
            .background((255, 255, 255))
            .border(
                Border::new()
                    .fill((200, 200, 200))
                    .width(1.0)
                    .alignment(BorderAlignment::Inner),
            )
            .corner_radius(4.0)
            .padding(12.0)
            .width(Size::px(200.))
            .height(Size::px(60.))
            .scale(scale)
            .shadow((0., 2., 4., 0., (0, 0, 0, 25)))
            .child(label().text(self.0.title.clone()))
    }
}

fn app() -> impl IntoElement {
    let tasks = use_state(|| {
        vec![
            Task::new(1, "Design UI mockups".to_string(), TaskStatus::Todo),
            Task::new(2, "Implement drag and drop".to_string(), TaskStatus::Todo),
            Task::new(3, "Write unit tests".to_string(), TaskStatus::InProgress),
            Task::new(4, "Deploy to production".to_string(), TaskStatus::Done),
            Task::new(5, "Setup CI/CD".to_string(), TaskStatus::Done),
            Task::new(6, "Add animations".to_string(), TaskStatus::InProgress),
        ]
    });

    rect().expanded().center().child(
        rect()
            .direction(Direction::Horizontal)
            .width(Size::px(800.))
            .height(Size::fill())
            .content(Content::Flex)
            .spacing(12.0)
            .padding(12.0)
            .child(column(tasks, TaskStatus::Todo, "To Do".to_string()))
            .child(column(
                tasks,
                TaskStatus::InProgress,
                "In Progress".to_string(),
            ))
            .child(column(tasks, TaskStatus::Done, "Done".to_string())),
    )
}

fn column(mut tasks: State<Vec<Task>>, status: TaskStatus, title: String) -> impl IntoElement {
    rect()
        .direction(Direction::Vertical)
        .width(Size::flex(1.))
        .height(Size::fill())
        .child(DropZone::<usize>::new(
            rect()
                .direction(Direction::Vertical)
                .expanded()
                .padding(16.0)
                .spacing(8.0)
                .background((240, 240, 240))
                .corner_radius(8.0)
                .child(
                    label()
                        .text(title.clone())
                        .font_size(18.0)
                        .font_weight(FontWeight::BOLD),
                )
                .children_iter(
                    tasks
                        .read()
                        .iter()
                        .filter(|t| t.status == status)
                        .map(|task| {
                            DragZone::<usize>::new(
                                task.id,
                                Portal::new(task.id)
                                    .height(Size::px(60.))
                                    .width(Size::fill())
                                    .function(Function::Expo)
                                    .duration(Duration::from_millis(500))
                                    .child(Card(task.clone())),
                            )
                            .drag_element(
                                Portal::new(task.id)
                                    .height(Size::px(60.))
                                    .width(Size::fill())
                                    .function(Function::Expo)
                                    .duration(Duration::from_millis(500))
                                    .child(
                                        rect()
                                            .interactive(false)
                                            .background((255, 255, 255))
                                            .layer(999)
                                            .border(
                                                Border::new()
                                                    .fill((200, 200, 200))
                                                    .width(1.0)
                                                    .alignment(BorderAlignment::Inner),
                                            )
                                            .corner_radius(4.0)
                                            .padding(12.0)
                                            .width(Size::px(200.))
                                            .height(Size::px(60.))
                                            .shadow((0., 2., 4., 0., (0, 0, 0, 25)))
                                            .child(label().text(task.title.clone())),
                                    ),
                            )
                            .show_while_dragging(false)
                            .key(task.id)
                            .into()
                        }),
                ),
            move |task_id: usize| {
                let mut task = tasks
                    .read()
                    .iter()
                    .find(|t| t.id == task_id)
                    .unwrap()
                    .clone();
                if task.status != status {
                    tasks.write().retain(|t| t.id != task_id);
                    task.status = status;
                    tasks.write().push(task);
                }
            },
        ))
}
