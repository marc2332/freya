use freya_core::{parsing::Parse, values::HighlightMode};

#[test]
fn parse_expanded_highlight_mode() {
    let expanded = HighlightMode::parse("expanded");
    assert_eq!(expanded, Ok(HighlightMode::Expanded));
}

#[test]
fn parse_fit_highlight_mode() {
    let fit = HighlightMode::parse("fit");
    assert_eq!(fit, Ok(HighlightMode::Fit));
}

#[test]
fn parse_fallback_highlight_mode() {
    let fallback = HighlightMode::parse("Hello, World!");
    assert_eq!(fallback, Ok(HighlightMode::Fit));
}
