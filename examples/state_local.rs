use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> Element {
    rect()
        .expanded()
        .center()
        .horizontal()
        .child(CoolComponent(2))
        .child(CoolComponent(23))
        .child(CoolComponent(34))
        .into()
}

#[derive(PartialEq)]
struct CoolComponent(i32);
impl Render for CoolComponent {
    fn render(&self) -> Element {
        let mut state = use_state(|| self.0);

        let increase = move |_| {
            *state.write() += 1;
        };

        Button::new()
            .on_press(increase)
            .child(format!("Value: {}", state.read()))
            .into()
    }
}
