use freya::{
    helpers::from_fn_owned,
    prelude::*,
};
use rand::Rng;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
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
                .map(|(i, id)| from_fn_owned(id, (state, i), list)),
        )
        .child(Button::new().on_press(add).child("Create"))
        .into()
}

fn list((mut state, i): (State<Vec<i32>>, usize)) -> Element {
    let mut count = use_state(|| 2);

    rect()
        .children_iter(
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
                        state.write().remove(i);
                    })
                    .child("Remove")
                    .into()]),
        )
        .into()
}
