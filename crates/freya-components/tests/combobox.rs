use freya::prelude::*;
use freya_testing::prelude::*;

fn app() -> impl IntoElement {
    ComboBox::new(
        use_consume::<State<String>>(),
        vec![
            "Rust🦀".to_string(),
            "Ruby".to_string(),
            "Python".to_string(),
        ],
    )
    .placeholder("Language")
}

fn option_area(runner: &TestingRunner, text: &str) -> Option<Area> {
    runner.find(|node, element| {
        Label::try_downcast(element)
            .filter(|label| label.text == text)
            .map(|_| node.layout().area)
    })
}

fn assert_cursor_at_end(runner: &TestingRunner, text: &str) {
    let cursor = runner.find(|_, element| {
        Paragraph::try_downcast(element)
            .filter(|paragraph| paragraph.spans.iter().any(|span| span.text == text))
            .and_then(|paragraph| paragraph.cursor_index)
    });
    assert_eq!(cursor, Some(text.encode_utf16().count()));
}

#[test]
fn combobox_uses_its_own_input_and_dropdown_colors() {
    let (mut runner, _) = TestingRunner::new(
        app,
        (400., 400.).into(),
        |runner| {
            runner.provide_root_context(|| {
                let mut theme = light_theme();
                let mut combobox = theme
                    .get_theme::<ComboBoxThemePreference>("combobox")
                    .unwrap()
                    .clone();
                combobox.background = Color::RED.into();
                combobox.list_background = Color::BLUE.into();
                theme.set_theme("combobox", combobox);
                State::create(theme)
            });
            runner.provide_root_context(|| State::create(String::new()))
        },
        1.,
    );
    let has_background = |runner: &TestingRunner, color| {
        runner
            .find(
                |_, element| match &Rect::try_downcast(element)?.style.background {
                    Fill::Color(background) if *background == color => Some(()),
                    _ => None,
                },
            )
            .is_some()
    };

    assert!(has_background(&runner, Color::RED));
    assert!(!has_background(&runner, Color::BLUE));

    runner.click_cursor((15., 15.));
    for _ in 0..4 {
        runner.sync_and_update();
    }
    assert!(has_background(&runner, Color::BLUE));
}

#[test]
fn keyboard_filters_and_commits_a_suggestion() {
    let (mut runner, mut value) = TestingRunner::new(
        app,
        (400., 400.).into(),
        |runner| runner.provide_root_context(|| State::create(String::new())),
        1.,
    );
    runner.sync_and_update();
    runner.press_key(Key::Named(NamedKey::Tab));
    for _ in 0..4 {
        runner.sync_and_update();
    }
    assert!(option_area(&runner, "Python").is_some());
    runner.press_key(Key::Named(NamedKey::Escape));
    assert!(option_area(&runner, "Python").is_none());

    runner.write_text("rU");

    assert_eq!(*value.peek(), "");
    assert!(option_area(&runner, "Rust🦀").is_some());
    assert!(option_area(&runner, "Ruby").is_some());
    assert!(option_area(&runner, "Python").is_none());

    runner.press_key(Key::Named(NamedKey::ArrowDown));
    runner.press_key(Key::Named(NamedKey::Enter));
    runner.sync_and_update();
    assert_eq!(*value.peek(), "Rust🦀");
    assert!(option_area(&runner, "Rust🦀").is_none());
    assert_cursor_at_end(&runner, "Rust🦀");

    runner.write_text(" unmatched");
    assert_eq!(*value.peek(), "Rust🦀");
    runner.press_key(Key::Named(NamedKey::Escape));
    runner.sync_and_update();
    assert_cursor_at_end(&runner, "Rust🦀");

    value.set("Ruby".to_string());
    runner.sync_and_update();
    runner.press_key(Key::Named(NamedKey::ArrowDown));
    assert!(option_area(&runner, "Ruby").is_some());
    assert!(option_area(&runner, "Rust🦀").is_none());
}

#[test]
fn pointer_selects_a_suggestion_and_dismisses_the_dropdown() {
    let (mut runner, value) = TestingRunner::new(
        app,
        (400., 400.).into(),
        |runner| runner.provide_root_context(|| State::create(String::new())),
        1.,
    );
    runner.sync_and_update();
    assert!(option_area(&runner, "Ruby").is_none());

    runner.click_cursor((15., 15.));
    for _ in 0..4 {
        runner.sync_and_update();
    }
    let ruby = option_area(&runner, "Ruby").unwrap();
    runner.click_cursor((
        f64::from(ruby.min_x() + ruby.width() / 2.),
        f64::from(ruby.min_y() + ruby.height() / 2.),
    ));
    assert_eq!(*value.peek(), "Ruby");
    assert!(option_area(&runner, "Ruby").is_none());
    assert_cursor_at_end(&runner, "Ruby");

    runner.click_cursor((245., 20.));
    for _ in 0..4 {
        runner.sync_and_update();
    }
    assert!(option_area(&runner, "Ruby").is_some());
    runner.click_cursor((350., 350.));
    assert!(option_area(&runner, "Ruby").is_none());
    assert_eq!(*value.peek(), "Ruby");
}
