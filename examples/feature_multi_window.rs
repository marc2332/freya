use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    let count = use_state(|| 0);

    let on_press = move |_| {
        EventNotifier::get().launch_window(WindowConfig::new(move || sub_app(count)));
    };

    rect()
        .expanded()
        .center()
        .child(Button::new().on_press(on_press).child("Open"))
        .into()
}

fn sub_app(mut count: State<i32>) -> Element {
    let on_press = move |_| {
        *count.write() += 1;
    };

    rect()
        .expanded()
        .center()
        .spacing(6.)
        .child(format!("Value is {}", count.read()))
        .child(Button::new().on_press(on_press).child("Increase"))
        .into()
}
