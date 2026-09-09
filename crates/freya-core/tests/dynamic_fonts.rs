use std::collections::HashMap;

use freya::prelude::*;
use freya_testing::prelude::*;

const NOTO_SANS: &[u8] = include_bytes!("../../freya-edit/tests/NotoSans-Regular.ttf");
const SAMUEL_MORSE: &[u8] = include_bytes!("../../../examples/SamuelMorse.otf");

fn app() -> impl IntoElement {
    rect()
        .expanded()
        .on_mouse_up(|_| {
            Platform::get().load_font("Samuel Morse", SAMUEL_MORSE);
        })
        .child(label().font_family("Samuel Morse").text("Hello, World!"))
}

fn launch<const N: usize>(fonts: [(&str, &[u8]); N]) -> TestingRunner {
    let mut test = launch_test(app);
    test.set_fonts(HashMap::from_iter(fonts));
    test.set_default_fonts(&["NotoSans".into()]);
    test
}

#[test]
fn load_font_at_runtime() {
    let label_width = |test: &TestingRunner| {
        test.find(|node, element| Label::try_downcast(element).map(|_| node.layout().area.width()))
            .expect("the label should be in the tree")
    };

    let preloaded = launch([("NotoSans", NOTO_SANS), ("Samuel Morse", SAMUEL_MORSE)]);
    let with_font = label_width(&preloaded);

    let mut test = launch([("NotoSans", NOTO_SANS)]);
    assert_ne!(
        label_width(&test),
        with_font,
        "both fonts measure the same, so this test proves nothing"
    );

    test.click_cursor((250., 250.));

    assert_eq!(label_width(&test), with_font);
}
