use std::time::Duration;

use freya::{
    animation::*,
    prelude::*,
};

use crate::showcases::heading;

#[derive(PartialEq, Clone, Copy, Hash)]
enum TaskStatus {
    Todo,
    Done,
}

#[derive(PartialEq, Clone)]
struct Task {
    id: usize,
    title: String,
    status: TaskStatus,
}

impl Task {
    fn new(id: usize, title: &str, status: TaskStatus) -> Self {
        Self {
            id,
            title: title.to_string(),
            status,
        }
    }
}

#[derive(PartialEq)]
struct TaskCard(Task);

impl Component for TaskCard {
    fn render(&self) -> impl IntoElement {
        let colors = use_theme().read().colors.clone();
        let animation = use_animation(|conf| {
            conf.on_creation(OnCreation::Finish);
            AnimNum::new(0.85, 1.)
                .time(400)
                .function(Function::Expo)
                .ease(Ease::Out)
        });

        rect()
            .background(colors.background)
            .corner_radius(10.)
            .padding(12.)
            .width(Size::fill())
            .height(Size::px(58.))
            .scale(animation.read().value())
            .shadow((0., 2., 6., 0., (0, 0, 0, 25)))
            .child(
                label()
                    .text(self.0.title.clone())
                    .max_lines(1)
                    .text_overflow(TextOverflow::Ellipsis),
            )
    }
}

#[derive(PartialEq)]
pub struct KanbanShowcase;

impl Component for KanbanShowcase {
    fn render(&self) -> impl IntoElement {
        let tasks = use_state(|| {
            vec![
                Task::new(1, "Buy milk", TaskStatus::Todo),
                Task::new(2, "Fix the bike", TaskStatus::Todo),
                Task::new(3, "Call the dentist", TaskStatus::Todo),
                Task::new(4, "Water the plants", TaskStatus::Done),
            ]
        });

        rect()
            .spacing(16.)
            .expanded()
            .child(heading(
                "Kanban",
                "Pick up a card and drop it somewhere else",
            ))
            .child(
                rect()
                    .horizontal()
                    .expanded()
                    .content(Content::Flex)
                    .spacing(12.)
                    .child(Column {
                        tasks,
                        status: TaskStatus::Todo,
                        title: "To Do",
                    })
                    .child(Column {
                        tasks,
                        status: TaskStatus::Done,
                        title: "Done",
                    }),
            )
    }
}

#[derive(PartialEq)]
struct Column {
    tasks: State<Vec<Task>>,
    status: TaskStatus,
    title: &'static str,
}

impl Component for Column {
    fn render(&self) -> impl IntoElement {
        let mut tasks = self.tasks;
        let status = self.status;
        let dragging = use_drag::<usize>().read().is_some();
        let colors = use_theme().read().colors.clone();
        let mut card_width = use_state(|| 0.);

        rect().width(Size::flex(1.)).height(Size::fill()).child(
            DropZone::new(move |task_id: usize| {
                let Some(mut task) = tasks.read().iter().find(|task| task.id == task_id).cloned()
                else {
                    return;
                };
                if task.status != status {
                    tasks.write().retain(|task| task.id != task_id);
                    task.status = status;
                    tasks.write().push(task);
                }
            })
            .child(
                rect()
                    .expanded()
                    .padding(12.)
                    .spacing(8.)
                    .background(colors.surface_secondary)
                    .corner_radius(12.)
                    .child(
                        label()
                            .text(self.title)
                            .font_size(16.)
                            .font_weight(FontWeight::BOLD),
                    )
                    .child(
                        rect()
                            .width(Size::fill())
                            .spacing(8.)
                            .on_sized(move |event: Event<SizedEventData>| {
                                card_width.set_if_modified(event.area.width())
                            })
                            .children(
                                tasks
                                    .read()
                                    .iter()
                                    .filter(|task| task.status == status)
                                    .enumerate()
                                    .map(|(index, task)| {
                                        let portal = || {
                                            Portal::new(task.id)
                                                .height(Size::px(58.))
                                                .width(Size::fill())
                                                .function(Function::Expo)
                                                .duration(Duration::from_millis(400))
                                        };

                                        DragZone::new(task.id)
                                            .drag_element(
                                                portal().child(
                                                    rect()
                                                        .interactive(false)
                                                        .background(colors.background)
                                                        .layer(999)
                                                        .corner_radius(10.)
                                                        .padding(12.)
                                                        .width(Size::px(card_width()))
                                                        .height(Size::px(58.))
                                                        .shadow((0., 6., 16., 0., (0, 0, 0, 45)))
                                                        .child(
                                                            label()
                                                                .text(task.title.clone())
                                                                .max_lines(1)
                                                                .text_overflow(
                                                                    TextOverflow::Ellipsis,
                                                                ),
                                                        ),
                                                ),
                                            )
                                            .show_while_dragging(false)
                                            .child(
                                                portal()
                                                    .animation_dependency((status, index, dragging))
                                                    .child(TaskCard(task.clone())),
                                            )
                                            .key(task.id)
                                    }),
                            ),
                    ),
            ),
        )
    }
}
