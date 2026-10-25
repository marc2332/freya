#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(600., 500.)))
}

#[derive(Clone, PartialEq)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    fn includes(self, task: &Task) -> bool {
        match self {
            Self::All => true,
            Self::Active => !task.completed,
            Self::Completed => task.completed,
        }
    }
}

fn app() -> impl IntoElement {
    use_init_theme(light_theme);
    let mut tasks = use_state(Vec::<Task>::new);
    let mut next_id = use_state(|| 0usize);
    let mut filter = use_state(|| Filter::All);

    let remaining = tasks.read().iter().filter(|task| !task.completed).count();
    let selected_filter = *filter.read();

    rect()
        .expanded()
        .content(Content::Flex)
        .theme_background()
        .theme_color()
        .padding(20.)
        .spacing(12.)
        .child(label().text("My tasks").font_size(28.))
        .child(TaskComposer::new(move |title| {
            let id = *next_id.read();
            *next_id.write() += 1;
            tasks.write().push(Task {
                id,
                title,
                completed: false,
            });
        }))
        .child(
            SegmentedButton::new().children(
                [
                    (Filter::All, "All"),
                    (Filter::Active, "Active"),
                    (Filter::Completed, "Completed"),
                ]
                .into_iter()
                .map(|(value, title)| {
                    ButtonSegment::new()
                        .selected(selected_filter == value)
                        .on_press(move |_| filter.set(value))
                        .child(title)
                }),
            ),
        )
        .child(format!("{remaining} tasks remaining"))
        .child(
            ScrollView::new().height(Size::flex(1.)).children(
                tasks
                    .read()
                    .iter()
                    .filter(|task| selected_filter.includes(task))
                    .map(|task| {
                        let id = task.id;
                        TaskRow::new(
                            task.clone(),
                            move |()| {
                                if let Some(task) =
                                    tasks.write().iter_mut().find(|task| task.id == id)
                                {
                                    task.completed = !task.completed;
                                }
                            },
                            move |()| {
                                tasks.write().retain(|task| task.id != id);
                            },
                        )
                        .key(id)
                    }),
            ),
        )
}

#[derive(PartialEq)]
struct TaskComposer {
    on_add: EventHandler<String>,
}

impl TaskComposer {
    fn new(on_add: impl Into<EventHandler<String>>) -> Self {
        Self {
            on_add: on_add.into(),
        }
    }
}

impl Component for TaskComposer {
    fn render(&self) -> impl IntoElement {
        let mut draft = use_state(String::new);

        let can_add = !draft.read().trim().is_empty();

        rect()
            .horizontal()
            .spacing(8.)
            .child(Input::new(draft).placeholder("What needs doing?"))
            .child(
                Button::new()
                    .enabled(can_add)
                    .on_press({
                        let on_add = self.on_add.clone();
                        move |_| {
                            let title = draft.read().trim().to_owned();
                            if !title.is_empty() {
                                on_add.call(title);
                                draft.write().clear();
                            }
                        }
                    })
                    .child("Add task"),
            )
    }
}

#[derive(PartialEq)]
struct TaskRow {
    task: Task,
    on_toggle: EventHandler<()>,
    on_remove: EventHandler<()>,
    key: DiffKey,
}

impl TaskRow {
    fn new(
        task: Task,
        on_toggle: impl Into<EventHandler<()>>,
        on_remove: impl Into<EventHandler<()>>,
    ) -> Self {
        Self {
            task,
            on_toggle: on_toggle.into(),
            on_remove: on_remove.into(),
            key: DiffKey::None,
        }
    }
}

impl KeyExt for TaskRow {
    fn key_mut(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for TaskRow {
    fn render(&self) -> impl IntoElement {
        rect()
            .width(Size::fill())
            .content(Content::Flex)
            .horizontal()
            .spacing(8.)
            .cross_align(Alignment::Center)
            .child(label().text(self.task.title.clone()).width(Size::flex(1.)))
            .child(
                Button::new()
                    .on_press({
                        let on_toggle = self.on_toggle.clone();
                        move |_| on_toggle.call(())
                    })
                    .child(if self.task.completed { "Reopen" } else { "Complete" }),
            )
            .child(
                Button::new()
                    .on_press({
                        let on_remove = self.on_remove.clone();
                        move |_| on_remove.call(())
                    })
                    .child("Delete"),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[cfg(test)]
mod tests {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    use crate::app;

    fn click_label(runner: &mut TestingRunner, text: &str) {
        let point = runner
            .find(|node, element| {
                Label::try_downcast(element)
                    .filter(|label| label.text == text)
                    .map(|_| node.layout().area.center().to_f64())
            })
            .unwrap_or_else(|| panic!("Missing label: {text}"));
        runner.click_cursor(point);
    }

    #[test]
    fn add_and_complete_task() {
        let mut runner = launch_test(app);
        runner.sync_and_update();
        // Focus the input.
        runner.click_cursor((30., 80.));
        runner.write_text("Learn Freya");
        click_label(&mut runner, "Add task");
        assert!(
            runner.find(|_, element| {
                Label::try_downcast(element).filter(|label| label.text == "Learn Freya")
            }).is_some()
        );

        click_label(&mut runner, "Complete");
        assert!(
            runner.find(|_, element| {
                Label::try_downcast(element).filter(|label| label.text == "0 tasks remaining")
            }).is_some()
        );
    }
}
