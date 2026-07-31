use freya_core::{
    helpers::from_fn_standalone_borrowed_keyed,
    prelude::*,
};
use freya_testing::prelude::*;
use torin::prelude::Size;

#[derive(PartialEq)]
struct CompA {
    key: DiffKey,
}

impl KeyExt for CompA {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CompA {
    fn render(&self) -> impl IntoElement {
        label().text("A")
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(PartialEq)]
struct CompB {
    key: DiffKey,
}

impl KeyExt for CompB {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CompB {
    fn render(&self) -> impl IntoElement {
        let text = use_state(|| "B");
        label().text(*text.read())
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[test]
pub fn same_key_different_component_type() {
    fn app() -> impl IntoElement {
        let mut toggle = use_state(|| false);
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .on_mouse_up(move |_| toggle.toggle())
            .child(if toggle() {
                (CompB { key: DiffKey::None }).key(1).into_element()
            } else {
                (CompA { key: DiffKey::None }).key(1).into_element()
            })
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    let label = test
        .find(|_, element| Label::try_downcast(element).filter(|label| label.text.as_ref() == "A"));
    assert!(label.is_some());

    test.click_cursor((50., 50.));
    test.sync_and_update();

    let label = test
        .find(|_, element| Label::try_downcast(element).filter(|label| label.text.as_ref() == "B"));
    assert!(label.is_some());
}

#[test]
pub fn same_key_different_from_fn_component() {
    fn comp_a(props: &u8) -> Element {
        label().text(format!("A{props}")).into_element()
    }

    fn comp_b(props: &String) -> Element {
        label().text(format!("B{props}")).into_element()
    }

    fn app() -> impl IntoElement {
        let mut toggle = use_state(|| false);
        rect()
            .width(Size::px(100.))
            .height(Size::px(100.))
            .on_mouse_up(move |_| toggle.toggle())
            .child(if toggle() {
                from_fn_standalone_borrowed_keyed(1, "hey".to_string(), comp_b)
            } else {
                from_fn_standalone_borrowed_keyed(1, 5u8, comp_a)
            })
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "A5")
    });
    assert!(label.is_some());

    test.click_cursor((50., 50.));
    test.sync_and_update();

    let label = test.find(|_, element| {
        Label::try_downcast(element).filter(|label| label.text.as_ref() == "Bhey")
    });
    assert!(label.is_some());
}
