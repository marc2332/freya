use freya_node_state::{
    HighlightMode,
    Parse,
};

#[test]
fn parse_expanded_highlight_mode() {
    let expanded = HighlightMode::parse_value("expanded");
    assert_eq!(expanded, Ok(HighlightMode::Expanded));
}

#[test]
fn parse_fit_highlight_mode() {
    let fit = HighlightMode::parse_value("fit");
    assert_eq!(fit, Ok(HighlightMode::Fit));
}

#[test]
fn parse_fallback_highlight_mode() {
    let fallback = HighlightMode::parse_value("Hello, World!");

    assert!(fallback.is_err());
}
