use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn relative_sizes_follow_the_base_and_fixed_sizes_do_not() {
    let mut preference = TypographyThemePreference::default();
    let typography = preference.resolve(&LIGHT_COLORS);
    assert_eq!(typography.font_size(TextRole::Control), 14.);
    assert_eq!(typography.font_size(TextRole::Content), 16.);

    preference.base_size = 24.0.into();
    preference.title = TypographySize::Fixed(30.).into();
    let typography = preference.resolve(&LIGHT_COLORS);
    assert_eq!(typography.font_size(TextRole::Control), 21.);
    assert_eq!(typography.font_size(TextRole::Content), 24.);
    assert_eq!(typography.font_size(TextRole::Title), 30.);
}

#[test]
fn changing_typography_updates_inherited_sizes_but_not_overrides() {
    fn app() -> impl IntoElement {
        rect()
            .child(FloatingTab::new().child("Tab"))
            .child(label().font_size(14.).text("Tab"))
            .child(label().font_size(21.).text("Tab"))
            .child(Button::new().child(label().font_size(30.).text("Override")))
    }

    let (mut runner, mut theme) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(light_theme())),
        1.,
    );

    let label_widths = |runner: &TestingRunner, text: &str| {
        runner.find_many(|node, element| {
            Label::try_downcast(element)
                .filter(|label| label.text == text)
                .map(|_| node.layout().area.width())
        })
    };

    let before = label_widths(&runner, "Tab");
    assert_eq!(before.len(), 3);
    assert_eq!(before[0], before[1]);
    assert!(before[0] < before[2]);
    let override_before = label_widths(&runner, "Override");
    assert_eq!(override_before.len(), 1);

    let mut updated = runner.run_in(light_theme);
    updated.set(
        "typography",
        TypographyThemePreference {
            base_size: 24.0.into(),
            ..TypographyThemePreference::default()
        },
    );
    theme.set_if_modified(updated);
    runner.sync_and_update();

    let after = label_widths(&runner, "Tab");
    assert_eq!(after, [before[2], before[1], before[2]]);
    assert_eq!(label_widths(&runner, "Override"), override_before);
}
