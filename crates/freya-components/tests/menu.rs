use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
pub fn menu_basic_render() {
    fn menu_app() -> impl IntoElement {
        let mut show_menu = use_state(|| true);

        rect().maybe_child(show_menu().then(|| {
            Menu::new()
                .on_close(move |_| show_menu.set(false))
                .child(MenuButton::new().child("Open"))
                .child(MenuButton::new().child("Save"))
                .child(MenuButton::new().child("Close"))
        }))
    }

    let mut test = launch_test(menu_app);
    test.sync_and_update();

    // Find all menu button labels
    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    assert_eq!(labels.len(), 3);
    assert_eq!(
        Label::try_downcast(&*labels[0].element()).unwrap().text,
        "Open"
    );
    assert_eq!(
        Label::try_downcast(&*labels[1].element()).unwrap().text,
        "Save"
    );
    assert_eq!(
        Label::try_downcast(&*labels[2].element()).unwrap().text,
        "Close"
    );
}

#[test]
pub fn menu_button_click() {
    fn menu_app() -> impl IntoElement {
        let mut show_menu = use_state(|| true);
        let mut clicked = use_state(|| false);

        rect()
            .child(label().text(format!("Clicked: {}", clicked())))
            .maybe_child(show_menu().then(|| {
                Menu::new()
                    .on_close(move |_| show_menu.set(false))
                    .child(
                        MenuButton::new()
                            .child("Open")
                            .on_press(move |_| clicked.set(true)),
                    )
                    .child(MenuButton::new().child("Save"))
            }))
    }

    let mut test = launch_test(menu_app);
    test.sync_and_update();

    let status_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Clicked:"))
                .map(|_| node)
        })
        .unwrap();

    assert_eq!(
        Label::try_downcast(&*status_label.element()).unwrap().text,
        "Clicked: false"
    );

    // Click the "Open" button
    test.click_cursor((50.0, 35.0));

    let status_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Clicked:"))
                .map(|_| node)
        })
        .unwrap();

    assert_eq!(
        Label::try_downcast(&*status_label.element()).unwrap().text,
        "Clicked: true"
    );
}

#[test]
pub fn menu_close_on_global_click() {
    fn menu_app() -> impl IntoElement {
        let mut show_menu = use_state(|| true);

        rect()
            .child(label().text(format!("Menu Open: {}", show_menu())))
            .maybe_child(show_menu().then(|| {
                Menu::new()
                    .on_close(move |_| show_menu.set(false))
                    .child(MenuButton::new().child("Item"))
            }))
    }

    let mut test = launch_test(menu_app);
    test.sync_and_update();

    let status_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Menu Open:"))
                .map(|_| node)
        })
        .unwrap();

    assert_eq!(
        Label::try_downcast(&*status_label.element()).unwrap().text,
        "Menu Open: true"
    );

    // Click outside the menu (global mouse up should close it)
    test.move_cursor((400.0, 400.0));
    test.sync_and_update();
    test.press_cursor((400.0, 400.0));
    test.sync_and_update();
    test.release_cursor((400.0, 400.0));
    test.sync_and_update();

    let status_label = test
        .find(|node, element| {
            Label::try_downcast(element)
                .filter(|l| l.text.starts_with("Menu Open:"))
                .map(|_| node)
        })
        .unwrap();

    assert_eq!(
        Label::try_downcast(&*status_label.element()).unwrap().text,
        "Menu Open: false"
    );
}

#[test]
pub fn menu_with_submenu() {
    fn menu_app() -> impl IntoElement {
        let mut show_menu = use_state(|| true);

        rect().maybe_child(show_menu().then(|| {
            Menu::new()
                .on_close(move |_| show_menu.set(false))
                .child(MenuButton::new().child("Open"))
                .child(
                    SubMenu::new()
                        .label("Export")
                        .child(MenuButton::new().child("PDF"))
                        .child(MenuButton::new().child("PNG")),
                )
                .child(MenuButton::new().child("Close"))
        }))
    }

    let mut test = launch_test(menu_app);
    test.sync_and_update();

    // Initially, submenu items should not be visible
    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    let label_texts: Vec<String> = labels
        .iter()
        .map(|l| Label::try_downcast(&*l.element()).unwrap().text.to_string())
        .collect();

    assert!(label_texts.contains(&"Open".to_string()));
    assert!(label_texts.contains(&"Export".to_string()));
    assert!(label_texts.contains(&"Close".to_string()));

    // PDF and PNG should not be visible initially
    assert!(!label_texts.contains(&"PDF".to_string()));
    assert!(!label_texts.contains(&"PNG".to_string()));

    // Hover over the "Export" submenu to open it
    test.move_cursor((50.0, 50.0));
    test.sync_and_update();

    // Now submenu items should be visible
    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    let label_texts: Vec<String> = labels
        .iter()
        .map(|l| Label::try_downcast(&*l.element()).unwrap().text.to_string())
        .collect();

    assert!(label_texts.contains(&"PDF".to_string()));
    assert!(label_texts.contains(&"PNG".to_string()));

    // Move mouse away from submenu
    test.move_cursor((50.0, 20.0));
    test.sync_and_update();

    // Submenu items should disappear
    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    let label_texts: Vec<String> = labels
        .iter()
        .map(|l| Label::try_downcast(&*l.element()).unwrap().text.to_string())
        .collect();

    assert!(!label_texts.contains(&"PDF".to_string()));
    assert!(!label_texts.contains(&"PNG".to_string()));
}

#[test]
pub fn menu_nested_submenus() {
    fn menu_app() -> impl IntoElement {
        let mut show_menu = use_state(|| true);

        rect().maybe_child(show_menu().then(|| {
            Menu::new()
                .on_close(move |_| show_menu.set(false))
                .child(
                    SubMenu::new()
                        .label("Options")
                        .child(MenuButton::new().child("Option 1"))
                        .child(
                            SubMenu::new()
                                .label("More Options")
                                .child(MenuButton::new().child("Option 2"))
                                .child(MenuButton::new().child("Option 3")),
                        ),
                )
                .child(MenuButton::new().child("Close"))
        }))
    }

    let mut test = launch_test(menu_app);
    test.sync_and_update();

    // Initially, only "Options" should be visible
    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    let label_texts: Vec<String> = labels
        .iter()
        .map(|l| Label::try_downcast(&*l.element()).unwrap().text.to_string())
        .collect();

    assert!(label_texts.contains(&"Options".to_string()));
    assert!(!label_texts.contains(&"Option 1".to_string()));
    assert!(!label_texts.contains(&"More Options".to_string()));

    // Hover over "Options" to reveal first submenu
    test.move_cursor((15.0, 15.0));
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    let label_texts: Vec<String> = labels
        .iter()
        .map(|l| Label::try_downcast(&*l.element()).unwrap().text.to_string())
        .collect();

    assert!(label_texts.contains(&"Option 1".to_string()));
    assert!(label_texts.contains(&"More Options".to_string()));
    assert!(!label_texts.contains(&"Option 2".to_string()));
    assert!(!label_texts.contains(&"Option 3".to_string()));

    // Hover over "More Options" to reveal nested submenu
    test.move_cursor((150.0, 45.0));
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    let label_texts: Vec<String> = labels
        .iter()
        .map(|l| Label::try_downcast(&*l.element()).unwrap().text.to_string())
        .collect();

    assert!(label_texts.contains(&"Option 2".to_string()));
    assert!(label_texts.contains(&"Option 3".to_string()));

    // Move mouse away to close nested submenu
    test.move_cursor((150.0, 20.0));
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    let label_texts: Vec<String> = labels
        .iter()
        .map(|l| Label::try_downcast(&*l.element()).unwrap().text.to_string())
        .collect();

    // Nested submenu items should disappear
    assert!(!label_texts.contains(&"Option 2".to_string()));
    assert!(!label_texts.contains(&"Option 3".to_string()));
    // But first level should still be visible
    assert!(label_texts.contains(&"Option 1".to_string()));

    // Move completely away to close all submenus
    test.move_cursor((20.0, 50.0));
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    let label_texts: Vec<String> = labels
        .iter()
        .map(|l| Label::try_downcast(&*l.element()).unwrap().text.to_string())
        .collect();

    // All submenu items should be hidden
    assert!(!label_texts.contains(&"Option 1".to_string()));
    assert!(!label_texts.contains(&"More Options".to_string()));
}

#[test]
pub fn menu_container_standalone() {
    fn menu_app() -> impl IntoElement {
        MenuContainer::new()
            .child(MenuItem::new().child(label().text("Item 1")))
            .child(MenuItem::new().child(label().text("Item 2")))
            .child(MenuItem::new().child(label().text("Item 3")))
    }

    let mut test = launch_test(menu_app);
    test.sync_and_update();

    let labels = test.find_many(|node, element| Label::try_downcast(element).map(|_| node));

    assert_eq!(labels.len(), 3);
    assert_eq!(
        Label::try_downcast(&*labels[0].element()).unwrap().text,
        "Item 1"
    );
    assert_eq!(
        Label::try_downcast(&*labels[1].element()).unwrap().text,
        "Item 2"
    );
    assert_eq!(
        Label::try_downcast(&*labels[2].element()).unwrap().text,
        "Item 3"
    );
}
