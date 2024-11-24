use freya_node_state::{
    Focusable,
    Parse,
};

#[test]
fn parse_focusable_enabled() {
    let enabled = Focusable::parse("true");
    assert_eq!(enabled, Ok(Focusable::Enabled));
}

#[test]
fn parse_focusable_disabled() {
    let disabled = Focusable::parse("false");
    assert_eq!(disabled, Ok(Focusable::Disabled));
}

#[test]
fn parse_focusable_fallback() {
    let fallback = Focusable::parse("hello!!");
    assert_eq!(fallback, Ok(Focusable::Unknown));
}
