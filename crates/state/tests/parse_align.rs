use freya_engine::prelude::*;
use freya_node_state::Parse;

#[test]
fn parse_center_text_align() {
    let center = TextAlign::parse("center");
    assert_eq!(center, Ok(TextAlign::Center));
}

#[test]
fn parse_end_text_align() {
    let end = TextAlign::parse("end");
    assert_eq!(end, Ok(TextAlign::End));
}

#[test]
fn parse_justify_text_align() {
    let justify = TextAlign::parse("justify");
    assert_eq!(justify, Ok(TextAlign::Justify));
}

#[test]
fn parse_lefttext_align() {
    let left = TextAlign::parse("left");
    assert_eq!(left, Ok(TextAlign::Left));
}

#[test]
fn parse_right_text_align() {
    let right = TextAlign::parse("right");
    assert_eq!(right, Ok(TextAlign::Right));
}

#[test]
fn parse_start_text_align() {
    let start = TextAlign::parse("start");
    assert_eq!(start, Ok(TextAlign::Start));
}
