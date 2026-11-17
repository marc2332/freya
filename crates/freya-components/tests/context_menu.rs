use freya::prelude::*;
use freya_testing::prelude::*;

fn context_menu_app() -> impl IntoElement {
    rect().expanded().child(ContextMenuViewer::new()).child(
        rect()
            .expanded()
            .on_secondary_down(|_| {
                ContextMenu::open_from_down(Menu::new().child(MenuButton::new().child("Item")));
            })
            .child("Area"),
    )
}

fn menu_is_open(test: &mut TestingRunner) -> bool {
    test.find(|_, element| Label::try_downcast(element).filter(|l| l.text.as_ref() == "Item"))
        .is_some()
}

fn right_button(test: &mut TestingRunner, name: MouseEventName, cursor: (f64, f64)) {
    test.send_event(PlatformEvent::Mouse {
        name,
        cursor: cursor.into(),
        button: Some(MouseButton::Right),
    });
}

fn right_click(test: &mut TestingRunner, cursor: (f64, f64)) {
    right_button(test, MouseEventName::MouseDown, cursor);
    test.sync_and_update();
    right_button(test, MouseEventName::MouseUp, cursor);
    test.sync_and_update();
}

#[test]
pub fn context_menu_closes_with_a_single_click() {
    let mut test = launch_test(context_menu_app);
    test.sync_and_update();

    right_click(&mut test, (100.0, 100.0));
    assert!(menu_is_open(&mut test));

    test.click_cursor((400.0, 400.0));
    assert!(!menu_is_open(&mut test));

    right_click(&mut test, (100.0, 100.0));
    right_click(&mut test, (300.0, 200.0));
    assert!(menu_is_open(&mut test));

    test.click_cursor((400.0, 400.0));
    assert!(!menu_is_open(&mut test));
}

#[test]
pub fn context_menu_closes_with_a_single_click_after_a_fast_open() {
    let mut test = launch_test(context_menu_app);
    test.sync_and_update();

    // A fast click's release is processed before the menu has mounted
    right_button(&mut test, MouseEventName::MouseDown, (100.0, 100.0));
    right_button(&mut test, MouseEventName::MouseUp, (100.0, 100.0));
    test.sync_and_update();
    assert!(menu_is_open(&mut test));

    test.click_cursor((400.0, 400.0));
    assert!(!menu_is_open(&mut test));
}

#[test]
fn custom_viewer_removal_restores_fallback() {
    fn app() -> impl IntoElement {
        let custom_viewer = use_consume::<State<bool>>();
        let mut menu_font = use_consume::<State<Option<f32>>>();

        rect()
            .expanded()
            .font_size(30.)
            .maybe(*custom_viewer.read(), |el| el.child(ContextMenuViewer::new()))
            .child(
                rect()
                    .expanded()
                    .on_secondary_down(move |_| {
                        ContextMenu::open_from_down(Menu::new().child(
                            MenuButton::new().child(label().text("Item").on_styled(
                                move |event: Event<StyledEventData>| {
                                    menu_font.set_if_modified(Some(event.text_style.font_size.into()));
                                },
                            )),
                        ));
                    })
                    .child("Area"),
            )
    }

    let (mut test, (mut custom_viewer, menu_font)) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| {
            (
                runner.provide_root_context(|| State::create(true)),
                runner.provide_root_context(|| State::create(None::<f32>)),
            )
        },
        1.,
    );

    right_click(&mut test, (100., 100.));
    assert_eq!(*menu_font.peek(), Some(30.));

    for visible in [false, true, false] {
        custom_viewer.set(visible);
        test.sync_and_update();
        assert_eq!(
            test.find_many(|_, element| {
                Label::try_downcast(element).filter(|label| label.text.as_ref() == "Item")
            })
            .len(),
            1
        );
        if visible {
            assert_eq!(*menu_font.peek(), Some(30.));
        } else {
            assert!(menu_font.peek().is_some_and(|font_size| font_size < 30.));
        }
    }

    test.click_cursor((400., 400.));
    assert!(!menu_is_open(&mut test));
}

#[test]
pub fn context_menu_closes_with_escape() {
    let mut test = launch_test(context_menu_app);
    test.sync_and_update();

    right_click(&mut test, (100.0, 100.0));
    assert!(menu_is_open(&mut test));

    test.press_key(Key::Named(NamedKey::Escape));
    test.sync_and_update();
    assert!(!menu_is_open(&mut test));
}
