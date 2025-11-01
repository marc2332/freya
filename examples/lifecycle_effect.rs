use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    let mut count = use_state(|| 4);

    use_side_effect(move || {
        println!("{}", count.read());
    });

    Button::new()
        .on_press(move |_| {
            *count.write() += 1;
        })
        .child("Increase")
        .into()
}
