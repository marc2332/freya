use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn theme_references_follow_variable_updates_but_fixed_overrides_do_not() {
    const BUTTON_BACKGROUND: &str = "app.button_background";

    fn custom_theme(background: Color) -> Theme {
        let mut theme = light_theme();
        theme.set_var(BUTTON_BACKGROUND, background);
        let mut colors = theme
            .get_theme::<ButtonColorsThemePreference>("filled_button")
            .unwrap()
            .clone();
        colors.background = Preference::Reference(BUTTON_BACKGROUND);
        theme.set_theme("filled_button", colors);
        theme
    }

    fn app() -> impl IntoElement {
        rect().child(Button::new().filled().child("Themed")).child(
            Button::new()
                .filled()
                .background(Color::BLUE)
                .child("Fixed"),
        )
    }

    let (mut runner, mut theme) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(custom_theme(Color::RED))),
        1.,
    );

    let backgrounds = |runner: &TestingRunner| {
        runner.find_many(
            |_, element| match &Rect::try_downcast(element)?.style.background {
                Fill::Color(color) if *color != Color::TRANSPARENT => Some(*color),
                _ => None,
            },
        )
    };

    assert_eq!(backgrounds(&runner), [Color::RED, Color::BLUE]);
    let updated = runner.run_in(|| custom_theme(Color::GREEN));
    theme.set_if_modified(updated);
    runner.sync_and_update();
    assert_eq!(backgrounds(&runner), [Color::GREEN, Color::BLUE]);
}
