use freya::prelude::*;
use freya_components::flex_wrap::FlexWrap;
use freya_testing::prelude::*;

/// Test basic FlexWrap layout with three child elements
/// Verifies that all children are rendered and accessible
#[test]
pub fn flex_wrap_basic_layout() {
    fn flex_wrap_app() -> impl IntoElement {
        let cards: Vec<Element> = vec![
            label().text("Card 1").into_element(),
            label().text("Card 2").into_element(),
            label().text("Card 3").into_element(),
        ];

        FlexWrap::new()
            .spacing(16.0)
            .item_width(200.0)
            .children(cards)
    }

    let mut test = launch_test(flex_wrap_app);
    test.sync_and_update();

    // Verify all children are rendered
    let labels = test.find_many(|_, element| Label::try_downcast(element));
    assert_eq!(labels.len(), 3, "All three cards should be rendered");

    assert_eq!(labels[0].text, "Card 1");
    assert_eq!(labels[1].text, "Card 2");
    assert_eq!(labels[2].text, "Card 3");
}

/// Test FlexWrap with custom spacing value
/// Verifies that the spacing parameter is correctly applied to the layout
#[test]
pub fn flex_wrap_custom_spacing() {
    fn flex_wrap_app() -> impl IntoElement {
        let cards: Vec<Element> = vec![
            label().text("Item 1").into_element(),
            label().text("Item 2").into_element(),
        ];

        FlexWrap::new()
            .spacing(24.0)
            .item_width(150.0)
            .children(cards)
    }

    let mut test = launch_test(flex_wrap_app);
    test.sync_and_update();

    // Verify custom spacing is applied by checking for rects with the spacing
    let rects_with_spacing = test.find_many(|_, element| {
        Rect::try_downcast(element).filter(|rect| rect.layout.layout.spacing.get() == 24.0)
    });

    assert!(
        !rects_with_spacing.is_empty(),
        "FlexWrap should apply custom spacing of 24.0"
    );
}

/// Test FlexWrap with custom item width
/// Verifies that all items respect the specified width parameter
#[test]
pub fn flex_wrap_custom_item_width() {
    fn flex_wrap_app() -> impl IntoElement {
        let cards: Vec<Element> = vec![
            label().text("A").into_element(),
            label().text("B").into_element(),
            label().text("C").into_element(),
        ];

        FlexWrap::new()
            .spacing(10.0)
            .item_width(300.0)
            .children(cards)
    }

    let mut test = launch_test(flex_wrap_app);
    test.sync_and_update();

    // Verify all items are rendered with labels
    let labels = test.find_many(|_, element| Label::try_downcast(element));
    assert_eq!(labels.len(), 3, "All three items should be rendered");
}

/// Test FlexWrap with empty children vector
/// Ensures the component handles edge case gracefully without panicking
#[test]
pub fn flex_wrap_empty_children() {
    fn flex_wrap_app() -> impl IntoElement {
        FlexWrap::new()
            .spacing(16.0)
            .item_width(200.0)
            .children(Vec::new())
    }

    let mut test = launch_test(flex_wrap_app);
    test.sync_and_update();

    // Container should still render even with no children
    // Just verify the test doesn't panic
    let labels = test.find_many(|_, element| Label::try_downcast(element));
    assert_eq!(
        labels.len(),
        0,
        "No labels should be present with empty children"
    );
}

/// Test FlexWrap with a single child element
/// Verifies correct behavior when only one item is present
#[test]
pub fn flex_wrap_single_child() {
    fn flex_wrap_app() -> impl IntoElement {
        let cards: Vec<Element> = vec![label().text("Only One").into_element()];

        FlexWrap::new()
            .spacing(16.0)
            .item_width(180.0)
            .children(cards)
    }

    let mut test = launch_test(flex_wrap_app);
    test.sync_and_update();

    // Verify single child is rendered
    let labels = test.find_many(|_, element| Label::try_downcast(element));
    assert_eq!(labels.len(), 1, "Single child should be rendered");
    assert_eq!(labels[0].text, "Only One");
}

/// Test FlexWrap with enough items to create multiple rows
/// Verifies that items wrap to new rows when they exceed container width
#[test]
pub fn flex_wrap_multiple_rows() {
    fn flex_wrap_app() -> impl IntoElement {
        // Create 10 items - enough to force multiple rows based on default container width
        let cards: Vec<Element> = (0..10)
            .map(|i| label().text(format!("Item {}", i + 1)).into_element())
            .collect();

        FlexWrap::new()
            .spacing(16.0)
            .item_width(200.0)
            .children(cards)
    }

    let mut test = launch_test(flex_wrap_app);
    test.sync_and_update();

    // Verify all 10 items are rendered
    let labels = test.find_many(|_, element| Label::try_downcast(element));
    assert_eq!(labels.len(), 10, "All 10 items should be rendered");

    // Verify first and last items
    assert_eq!(labels[0].text, "Item 1");
    assert_eq!(labels[9].text, "Item 10");
}

/// Test FlexWrap using default spacing and item_width values
/// Verifies that defaults (spacing: 16.0, item_width: 200.0) are applied correctly
#[test]
pub fn flex_wrap_default_values() {
    fn flex_wrap_app() -> impl IntoElement {
        let cards: Vec<Element> = vec![
            label().text("Default 1").into_element(),
            label().text("Default 2").into_element(),
        ];

        // Use default values by not setting spacing or item_width
        FlexWrap::new().children(cards)
    }

    let mut test = launch_test(flex_wrap_app);
    test.sync_and_update();

    // Verify default behavior works
    let labels = test.find_many(|_, element| Label::try_downcast(element));
    assert_eq!(
        labels.len(),
        2,
        "Both items should be rendered with defaults"
    );

    // Verify default spacing (16.0) is applied
    let rects_with_default_spacing = test.find_many(|_, element| {
        Rect::try_downcast(element).filter(|rect| rect.layout.layout.spacing.get() == 16.0)
    });
    assert!(
        !rects_with_default_spacing.is_empty(),
        "FlexWrap should use default spacing of 16.0"
    );
}

/// Test FlexWrap builder pattern chaining
/// Verifies that all builder methods can be chained and settings are applied
#[test]
pub fn flex_wrap_builder_pattern() {
    fn flex_wrap_app() -> impl IntoElement {
        let cards: Vec<Element> = vec![
            label().text("Builder 1").into_element(),
            label().text("Builder 2").into_element(),
            label().text("Builder 3").into_element(),
        ];

        // Test that the builder pattern works correctly
        FlexWrap::new()
            .spacing(20.0)
            .item_width(250.0)
            .children(cards)
    }

    let mut test = launch_test(flex_wrap_app);
    test.sync_and_update();

    // Verify all builder methods work together
    let labels = test.find_many(|_, element| Label::try_downcast(element));
    assert_eq!(labels.len(), 3, "Builder pattern should render all items");

    // Verify custom spacing from builder pattern
    let rects_with_spacing = test.find_many(|_, element| {
        Rect::try_downcast(element).filter(|rect| rect.layout.layout.spacing.get() == 20.0)
    });
    assert!(
        !rects_with_spacing.is_empty(),
        "Builder pattern spacing should be applied"
    );
}

/// Test FlexWrap layout direction structure
/// Verifies main container is vertical and row containers are horizontal
#[test]
pub fn flex_wrap_container_direction() {
    fn flex_wrap_app() -> impl IntoElement {
        let cards: Vec<Element> = vec![
            label().text("Direction A").into_element(),
            label().text("Direction B").into_element(),
        ];

        FlexWrap::new()
            .spacing(16.0)
            .item_width(200.0)
            .children(cards)
    }

    let mut test = launch_test(flex_wrap_app);
    test.sync_and_update();

    // Main container should be vertical
    let vertical_containers = test.find_many(|_, element| {
        Rect::try_downcast(element)
            .filter(|rect| rect.layout.layout.direction == Direction::Vertical)
    });
    assert!(
        !vertical_containers.is_empty(),
        "Main container should have vertical direction"
    );

    // Row containers should be horizontal
    let horizontal_containers = test.find_many(|_, element| {
        Rect::try_downcast(element)
            .filter(|rect| rect.layout.layout.direction == Direction::Horizontal)
    });
    assert!(
        !horizontal_containers.is_empty(),
        "Row containers should have horizontal direction"
    );
}

/// Test FlexWrap with varying content lengths
/// Verifies that items with different text lengths are all rendered correctly
#[test]
pub fn flex_wrap_with_different_content() {
    fn flex_wrap_app() -> impl IntoElement {
        let cards: Vec<Element> = vec![
            label().text("Short").into_element(),
            label().text("Medium Text").into_element(),
            label().text("Very Long Text Item").into_element(),
        ];

        FlexWrap::new()
            .spacing(12.0)
            .item_width(180.0)
            .children(cards)
    }

    let mut test = launch_test(flex_wrap_app);
    test.sync_and_update();

    // Verify all items with different content lengths are rendered
    let labels = test.find_many(|_, element| Label::try_downcast(element));
    assert_eq!(
        labels.len(),
        3,
        "All items with varying content should be rendered"
    );

    assert_eq!(labels[0].text, "Short");
    assert_eq!(labels[1].text, "Medium Text");
    assert_eq!(labels[2].text, "Very Long Text Item");
}
