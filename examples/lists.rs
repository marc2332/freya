#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;
use rand::Rng;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut state = use_state(|| {
        let mut rng = rand::rng();
        vec![rng.random::<i32>(), rng.random::<i32>()]
    });

    let add = move |_| {
        let mut rng = rand::rng();
        state.write().push(rng.random::<i32>());
    };

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .horizontal()
        .spacing(24.0)
        .background((0, 0, 0))
        .children_iter(
            state
                .read()
                .iter()
                .enumerate()
                .map(|(i, id)| List { i, id: *id, state }.into_element()),
        )
        .child(Button::new().on_press(add).child("Create"))
}

#[derive(PartialEq, Clone)]
struct List {
    pub i: usize,
    pub id: i32,
    pub state: State<Vec<i32>>,
}

impl RenderOwned for List {
    fn render_key(&self) -> DiffKey {
        DiffKey::from(&self.id)
    }

    fn render(mut self) -> impl IntoElement {
        let mut count = use_state(|| 2);

        rect().children_iter(
            (0..count())
                .map(|i| {
                    rect()
                        .on_press(move |_| {
                            *count.write() += 1;
                        })
                        .children([format!("Element {i}").into()])
                        .into()
                })
                .chain([Button::new()
                    .on_press(move |_| {
                        self.state.write().remove(self.i);
                    })
                    .child("Remove")
                    .into()]),
        )
    }
}
