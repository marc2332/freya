#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use freya_testing::TestingRunner;
use torin::prelude::Size2D;

fn main() {
    fn app() -> impl IntoElement {
        rect()
            .expanded()
            .spacing(24.0)
            .child(CoolComponent(20))
            .child(CoolComponent(60))
    }

    #[derive(PartialEq)]
    struct CoolComponent(u8);
    impl Component for CoolComponent {
        fn render(&self) -> impl IntoElement {
            let mut value = use_state(|| 40);
            rect()
                .on_press(move |_| {
                    *value.write() += 150;
                })
                .content(Content::Flex)
                .direction(Direction::Horizontal)
                .width(Size::fill())
                .height(Size::percent(50.))
                .spacing(24.0)
                .background(Color::BLACK)
                .children([
                    rect()
                        .width(Size::flex(1.))
                        .height(Size::fill())
                        .background((self.0 + value(), 180, 100))
                        .into(),
                    rect()
                        .width(Size::flex(1.))
                        .height(Size::fill())
                        .background((self.0 + value(), 180, 100))
                        .into(),
                    rect()
                        .width(Size::flex(1.))
                        .height(Size::fill())
                        .background((self.0 + value(), 180, 100))
                        .into(),
                    Button::new().on_press(|_| {}).child("hi").into(),
                ])
        }
    }

    let (mut runner, _) = TestingRunner::new(app, Size2D::new(500., 500.), |_| {}, 1.);
    runner.render_to_file("./demo-1.png");
    runner.click_cursor((270., 100.));
    runner.render_to_file("./demo-2.png");
    runner.handle_events_immediately();
}
