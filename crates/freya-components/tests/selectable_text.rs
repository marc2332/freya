use std::collections::HashMap;

use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn selectable_text_drag_selects_text() {
    let mut utils = launch_test(|| SelectableText::new("Hello Rustaceans\nHello Rustaceans"));

    utils.set_fonts(HashMap::from_iter([(
        "NotoSans",
        include_bytes!("../../freya-edit/tests/NotoSans-Regular.ttf").as_slice(),
    )]));
    utils.set_default_fonts(&["NotoSans".into()]);

    // Press down inside the selectable text
    utils.press_cursor((35.0, 3.0));
    utils.sync_and_update();

    // Drag to another global position
    utils.move_cursor((80.0, 25.0));
    utils.sync_and_update();

    // Release the mouse button
    utils.release_cursor((80.0, 25.0));
    utils.sync_and_update();

    // The selection should match the expected highlight (same expectation as other editor tests)
    let highlights = utils.find(|_, e| Some(Paragraph::try_downcast(e)?.highlights.clone()));
    assert_eq!(highlights, Some(vec![(5, 27)]));
}
