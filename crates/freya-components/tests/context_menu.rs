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

        rect()
            .expanded()
            .font_size(30.)
            .maybe(*custom_viewer.read(), |el| {
                el.child(ContextMenuViewer::new())
            })
            .child(
                rect()
                    .expanded()
                    .on_secondary_down(|_| {
                        ContextMenu::open_from_down(Menu::new().child("Item"));
                    })
                    .child("Area"),
            )
    }

    let (mut test, mut custom_viewer) = TestingRunner::new(
        app,
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(true)),
        1.,
    );
    let item_width = |test: &TestingRunner| {
        let items = test.find_many(|node, element| {
            Label::try_downcast(element)
                .filter(|label| label.text.as_ref() == "Item")
                .map(|_| node.layout().area.width())
        });
        assert_eq!(items.len(), 1);
        items[0]
    };

    right_click(&mut test, (100., 100.));
    let custom_width = item_width(&test);

    for visible in [false, true, false] {
        custom_viewer.set(visible);
        test.sync_and_update();
        test.sync_and_update();
        let width = item_width(&test);
        if visible {
            assert_eq!(width, custom_width);
        } else {
            assert!(width < custom_width);
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
