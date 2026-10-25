//! # Building your first app
//!
//! Build and test a todo app with task creation, completion, deletion, and filtering.
//!
//! Start with [Development Setup](crate::_docs::development_setup), then create a project.
//!
//! ```sh
//! cargo new task-app
//! cd task-app
//! cargo add freya
//! ```
//!
//! Start with section 1 in `src/main.rs`, then apply each section's changes.
//! The [finished example](https://github.com/marc2332/freya/blob/main/examples/learn_building_app.rs)
//! includes tests.
//!
//! If you cloned the Freya repository, run it like this:
//!
//! ```sh
//! cargo run --example learn_building_app
//! ```
//!
//! ## 1. Your first window
//!
//! `main` launches the window and `app` declares its contents. Keep `main` unchanged
//! and run `cargo run` after each step.
//!
//! ```rust,no_run
//! use freya::prelude::*;
//!
//! fn main() {
//!     launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(600., 500.)))
//! }
//!
//! fn app() -> impl IntoElement {
//!     use_init_theme(light_theme);
//!
//!     rect()
//!         .expanded()
//!         .theme_background()
//!         .theme_color()
//!         .padding(20.)
//!         .child(label().text("My tasks").font_size(28.))
//! }
//! ```
//!
//! See [UI and Components](crate::_docs::ui_and_components).
//!
//! ## 2. Build the screen
//!
//! Replace `app` with a text input and an Add task button. The input is editable,
//! but the button does nothing yet.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! fn app() -> impl IntoElement {
//!     use_init_theme(light_theme);
//!     // Keep the draft between renders. (1)
//!     let draft = use_state(String::new);
//!
//!     rect()
//!         .expanded()
//!         .theme_background()
//!         .theme_color()
//!         .padding(20.)
//!         .spacing(12.)
//!         .child(label().text("My tasks").font_size(28.))
//!         .child(
//!             rect()
//!                 .horizontal()
//!                 .spacing(8.)
//!                 .child(
//!                     // Bind the input to the string state. (2)
//!                     Input::new(draft).placeholder("What needs doing?"),
//!                 )
//!                 .child(Button::new().child("Add task")),
//!         )
//!         .child("Your tasks will appear here")
//! }
//! ```
//!
//! - (1) See [`use_state`](crate::prelude::use_state).
//! - (2) See [`Input`](crate::components::Input).
//!
//! ## 3. Make it interactive
//!
//! Add `Task` and replace `app` to store submitted tasks.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! #[derive(Clone, PartialEq)]
//! struct Task {
//!     id: usize,
//!     title: String,
//!     completed: bool,
//! }
//!
//! fn app() -> impl IntoElement {
//!     use_init_theme(light_theme);
//!     let mut draft = use_state(String::new);
//!     let mut tasks = use_state(Vec::<Task>::new);
//!     let mut next_id = use_state(|| 0usize);
//!
//!     // Subscribe to draft changes. (1)
//!     let can_add = !draft.read().trim().is_empty();
//!
//!     rect()
//!         .expanded()
//!         // Enable sharing remaining space between flex-sized children. (2)
//!         .content(Content::Flex)
//!         .theme_background()
//!         .theme_color()
//!         .padding(20.)
//!         .spacing(12.)
//!         .child(label().text("My tasks").font_size(28.))
//!         .child(
//!             rect()
//!                 .horizontal()
//!                 .spacing(8.)
//!                 .child(Input::new(draft).placeholder("What needs doing?"))
//!                 .child(
//!                     Button::new()
//!                         .enabled(can_add)
//!                         .on_press(move |_| {
//!                             let title = draft.read().trim().to_owned();
//!                             if !title.is_empty() {
//!                                 let id = *next_id.read();
//!                                 *next_id.write() += 1;
//!
//!                                 tasks.write().push(Task {
//!                                     id,
//!                                     title,
//!                                     completed: false,
//!                                 });
//!
//!                                 draft.write().clear();
//!                             }
//!                         })
//!                         .child("Add task"),
//!                 ),
//!         )
//!         // Give the list the remaining height. (2)
//!         .child(
//!             ScrollView::new().height(Size::flex(1.)).children(
//!                 tasks
//!                     .read()
//!                     .iter()
//!                     .map(|task| label().key(task.id).text(task.title.clone())),
//!             ),
//!         )
//! }
//! ```
//!
//! - (1) See [State Management](crate::_docs::state_management).
//! - (2) See [`Content::Flex`](crate::prelude::Content::Flex) and [`Size::flex`](crate::prelude::Size::flex).
//!
//! ## 4. Extract components
//!
//! Add `TaskComposer` alongside `Task`. It owns the draft and submits titles through
//! `on_add`. Typing updates the composer, while the parent owns tasks and assigns IDs.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! // PartialEq lets Freya compare component props between renders. (1)
//! #[derive(PartialEq)]
//! struct TaskComposer {
//!     // Report submitted titles to the parent. (2)
//!     on_add: EventHandler<String>,
//! }
//!
//! impl TaskComposer {
//!     fn new(on_add: impl Into<EventHandler<String>>) -> Self {
//!         Self {
//!             on_add: on_add.into(),
//!         }
//!     }
//! }
//!
//! impl Component for TaskComposer {
//!     fn render(&self) -> impl IntoElement {
//!         let mut draft = use_state(String::new);
//!
//!         let can_add = !draft.read().trim().is_empty();
//!
//!         rect()
//!             .horizontal()
//!             .spacing(8.)
//!             .child(Input::new(draft).placeholder("What needs doing?"))
//!             .child(
//!                 Button::new()
//!                     .enabled(can_add)
//!                     .on_press({
//!                         let on_add = self.on_add.clone();
//!                         move |_| {
//!                             let title = draft.read().trim().to_owned();
//!                             if !title.is_empty() {
//!                                 on_add.call(title);
//!                                 draft.write().clear();
//!                             }
//!                         }
//!                     })
//!                     .child("Add task"),
//!             )
//!     }
//! }
//! ```
//!
//! - (1) See [`Component`](crate::prelude::Component).
//! - (2) See [`EventHandler`](crate::prelude::EventHandler).
//!
//! Remove `draft` and `can_add` from `app`, then replace the input row with this child.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # #[derive(PartialEq)]
//! # struct TaskComposer { on_add: EventHandler<String> }
//! # impl TaskComposer {
//! #     fn new(on_add: impl Into<EventHandler<String>>) -> Self {
//! #         Self { on_add: on_add.into() }
//! #     }
//! # }
//! # impl Component for TaskComposer {
//! #     fn render(&self) -> impl IntoElement { rect() }
//! # }
//! # struct Task { id: usize, title: String, completed: bool }
//! # fn app() -> impl IntoElement {
//! # let mut tasks = use_state(Vec::<Task>::new);
//! # let mut next_id = use_state(|| 0usize);
//! #
//! # rect()
//! .child(TaskComposer::new(move |title| {
//!     let id = *next_id.read();
//!     *next_id.write() += 1;
//!     tasks.write().push(Task { id, title, completed: false });
//! }))
//! # }
//! ```
//!
//! ## 5. Complete and remove tasks
//!
//! Add `TaskRow` to display a task and request changes through callbacks.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # #[derive(Clone, PartialEq)]
//! # struct Task { id: usize, title: String, completed: bool }
//! #[derive(PartialEq)]
//! struct TaskRow {
//!     task: Task,
//!     on_toggle: EventHandler<()>,
//!     on_remove: EventHandler<()>,
//!     key: DiffKey,
//! }
//!
//! impl TaskRow {
//!     fn new(
//!         task: Task,
//!         on_toggle: impl Into<EventHandler<()>>,
//!         on_remove: impl Into<EventHandler<()>>,
//!     ) -> Self {
//!         Self {
//!             task,
//!             on_toggle: on_toggle.into(),
//!             on_remove: on_remove.into(),
//!             key: DiffKey::None,
//!         }
//!     }
//! }
//!
//! impl KeyExt for TaskRow {
//!     fn key_mut(&mut self) -> &mut DiffKey {
//!         &mut self.key
//!     }
//! }
//!
//! impl Component for TaskRow {
//!     fn render(&self) -> impl IntoElement {
//!         rect()
//!             .width(Size::fill())
//!             .content(Content::Flex)
//!             .horizontal()
//!             .spacing(8.)
//!             .cross_align(Alignment::Center)
//!             .child(label().text(self.task.title.clone()).width(Size::flex(1.)))
//!             .child(
//!                 Button::new()
//!                     .on_press({
//!                         let on_toggle = self.on_toggle.clone();
//!                         move |_| on_toggle.call(())
//!                     })
//!                     .child(if self.task.completed {
//!                         "Reopen"
//!                     } else {
//!                         "Complete"
//!                     }),
//!             )
//!             .child(
//!                 Button::new()
//!                     .on_press({
//!                         let on_remove = self.on_remove.clone();
//!                         move |_| on_remove.call(())
//!                     })
//!                     .child("Delete"),
//!             )
//!     }
//!
//!     fn render_key(&self) -> DiffKey {
//!         self.key.clone().or(self.default_key())
//!     }
//! }
//! ```
//!
//! Replace the scroll view in `app` with rows and their callbacks.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # #[derive(Clone, PartialEq)]
//! # struct Task { id: usize, title: String, completed: bool }
//! # #[derive(PartialEq)]
//! # struct TaskRow {
//! #     task: Task,
//! #     on_toggle: EventHandler<()>,
//! #     on_remove: EventHandler<()>,
//! #     key: DiffKey,
//! # }
//! # impl TaskRow {
//! #     fn new(task: Task, on_toggle: impl Into<EventHandler<()>>, on_remove: impl Into<EventHandler<()>>) -> Self {
//! #         Self { task, on_toggle: on_toggle.into(), on_remove: on_remove.into(), key: DiffKey::None }
//! #     }
//! # }
//! # impl KeyExt for TaskRow { fn key_mut(&mut self) -> &mut DiffKey { &mut self.key } }
//! # impl Component for TaskRow {
//! #     fn render(&self) -> impl IntoElement { rect() }
//! #     fn render_key(&self) -> DiffKey { self.key.clone().or(self.default_key()) }
//! # }
//! # fn app() -> impl IntoElement {
//! # let mut tasks = use_state(Vec::<Task>::new);
//! #
//! # rect()
//! .child(ScrollView::new().height(Size::flex(1.)).children(
//!     tasks.read().iter().map(|task| {
//!         let id = task.id;
//!         TaskRow::new(
//!             task.clone(),
//!             move |()| {
//!                 if let Some(task) = tasks.write().iter_mut().find(|task| task.id == id) {
//!                     task.completed = !task.completed;
//!                 }
//!             },
//!             move |()| {
//!                 tasks.write().retain(|task| task.id != id);
//!             },
//!         )
//!         // Key by stable ID to preserve row identity after deletions or reordering. (1)
//!         .key(id)
//!     }),
//! ))
//! # }
//! ```
//!
//! - (1) See [Lists and Keys](crate::_docs::ui_and_components#lists-and-keys).
//!
//! ## 6. Filter the list
//!
//! Choose whether to show all, active, or completed tasks. This version also shows how many
//! tasks remain unfinished.
//!
//! ```rust,no_run
//! # use freya::prelude::*;
//! # #[derive(Clone, PartialEq)]
//! # struct Task { id: usize, title: String, completed: bool }
//! # #[derive(PartialEq)]
//! # struct TaskComposer { on_add: EventHandler<String> }
//! # impl TaskComposer {
//! #     fn new(on_add: impl Into<EventHandler<String>>) -> Self { Self { on_add: on_add.into() } }
//! # }
//! # impl Component for TaskComposer { fn render(&self) -> impl IntoElement { rect() } }
//! # #[derive(PartialEq)]
//! # struct TaskRow { task: Task, on_toggle: EventHandler<()>, on_remove: EventHandler<()>, key: DiffKey }
//! # impl TaskRow {
//! #     fn new(task: Task, on_toggle: impl Into<EventHandler<()>>, on_remove: impl Into<EventHandler<()>>) -> Self {
//! #         Self { task, on_toggle: on_toggle.into(), on_remove: on_remove.into(), key: DiffKey::None }
//! #     }
//! # }
//! # impl KeyExt for TaskRow { fn key_mut(&mut self) -> &mut DiffKey { &mut self.key } }
//! # impl Component for TaskRow {
//! #     fn render(&self) -> impl IntoElement { rect() }
//! #     fn render_key(&self) -> DiffKey { self.key.clone().or(self.default_key()) }
//! # }
//! #[derive(Clone, Copy, PartialEq)]
//! enum Filter { All, Active, Completed }
//!
//! impl Filter {
//!     fn includes(self, task: &Task) -> bool {
//!         match self {
//!             Self::All => true,
//!             Self::Active => !task.completed,
//!             Self::Completed => task.completed,
//!         }
//!     }
//! }
//!
//! fn app() -> impl IntoElement {
//!     // ...
//! #     use_init_theme(light_theme);
//! #     let mut tasks = use_state(Vec::<Task>::new);
//! #     let mut next_id = use_state(|| 0usize);
//!     let mut filter = use_state(|| Filter::All);
//!
//!     let selected_filter = *filter.read();
//!     // Derive the count from all tasks, not just the visible ones.
//!     let remaining = tasks.read().iter().filter(|task| !task.completed).count();
//!
//!     rect()
//!         // ...
//! #         .expanded()
//! #         .content(Content::Flex)
//! #         .theme_background()
//! #         .theme_color()
//! #         .padding(20.)
//! #         .spacing(12.)
//! #         .child(label().text("My tasks").font_size(28.))
//! #         .child(TaskComposer::new(move |title| {
//! #             let id = *next_id.read();
//! #             *next_id.write() += 1;
//! #             tasks.write().push(Task { id, title, completed: false });
//! #         }))
//!         .child(SegmentedButton::new().children(
//!             [(Filter::All, "All"), (Filter::Active, "Active"), (Filter::Completed, "Completed")]
//!                 .into_iter().map(|(value, title)| {
//!                     ButtonSegment::new().selected(selected_filter == value)
//!                         .on_press(move |_| filter.set(value)).child(title)
//!                 }),
//!         ))
//!         .child(format!("{remaining} tasks remaining"))
//!         .child(ScrollView::new().height(Size::flex(1.)).children(
//!             tasks.read().iter()
//!                 .filter(|task| selected_filter.includes(task))
//!                 .map(|task| {
//!                     // ...
//! #                     let id = task.id;
//! #                     TaskRow::new(
//! #                         task.clone(),
//! #                         move |()| {
//! #                             if let Some(task) = tasks.write().iter_mut().find(|task| task.id == id) {
//! #                                 task.completed = !task.completed;
//! #                             }
//! #                         },
//! #                         move |()| {
//! #                             tasks.write().retain(|task| task.id != id);
//! #                         },
//! #                     ).key(id)
//!                 }),
//!         ))
//! }
//! ```
//!
//! The [finished example](https://github.com/marc2332/freya/blob/main/examples/learn_building_app.rs)
//! combines these pieces.
//!
//! ## 7. Test the app
//!
//! Add `freya-testing` as a development dependency.
//!
//! ```sh
//! cargo add freya-testing --dev
//! ```
//!
//! Add this test module to `src/main.rs`. `launch_test(app)` runs without a window.
//! The test adds a task and completes it, checking its title and the remaining count.
//!
//! ```rust,no_run
//! #[cfg(test)]
//! mod tests {
//!     use freya::prelude::*;
//!     use freya_testing::prelude::*;
//!
//!     use crate::app;
//!
//!     fn click_label(runner: &mut TestingRunner, text: &str) {
//!         let point = runner
//!             .find(|node, element| {
//!                 Label::try_downcast(element)
//!                     .filter(|label| label.text == text)
//!                     .map(|_| node.layout().area.center().to_f64())
//!             })
//!             .unwrap_or_else(|| panic!("Missing label: {text}"));
//!         runner.click_cursor(point);
//!     }
//!
//!     #[test]
//!     fn add_and_complete_task() {
//!         // Run the app without a window. (1)
//!         let mut runner = launch_test(app);
//!         runner.sync_and_update();
//!         // Focus the input.
//!         runner.click_cursor((30., 80.));
//!         runner.write_text("Learn Freya");
//!         click_label(&mut runner, "Add task");
//!         assert!(
//!             runner
//!                 .find(|_, element| {
//!                     Label::try_downcast(element).filter(|label| label.text == "Learn Freya")
//!                 })
//!                 .is_some()
//!         );
//!
//!         click_label(&mut runner, "Complete");
//!         assert!(
//!             runner
//!                 .find(|_, element| {
//!                     Label::try_downcast(element)
//!                         .filter(|label| label.text == "0 tasks remaining")
//!                 })
//!                 .is_some()
//!         );
//!     }
//! }
//! ```
//!
//! Run `cargo test` in your project, or target the example in a Freya checkout.
//!
//! ```sh
//! cargo test --example learn_building_app
//! ```
//!
//! - (1) See [freya_testing] for the testing APIs.
//!
//! ## Where to go next
//!
//! - Read [State Management](crate::_docs::state_management) for props, context, Radio, and reusable state interfaces.
//! - Read [Testing](freya_testing) for headless testing APIs.
//! - Browse [Examples](https://github.com/marc2332/freya/tree/main/examples) for focused API demonstrations and larger apps.
