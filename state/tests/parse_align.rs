use freya_node_state::parse_text_align;
use skia_safe::textlayout::TextAlign;

#[test]
fn parse_center_text_align() {
    let center = parse_text_align("center");
    assert_eq!(center, TextAlign::Center);
}

#[test]
fn parse_end_text_align() {
    let end = parse_text_align("end");
    assert_eq!(end, TextAlign::End);
}

#[test]
fn parse_justify_text_align() {
    let justify = parse_text_align("justify");
    assert_eq!(justify, TextAlign::Justify);
}

#[test]
fn parse_lefttext_align() {
    let left = parse_text_align("left");
    assert_eq!(left, TextAlign::Left);
}

#[test]
fn parse_right_text_align() {
    let right = parse_text_align("right");
    assert_eq!(right, TextAlign::Right);
}

#[test]
fn parse_start_text_align() {
    let start = parse_text_align("start");
    assert_eq!(start, TextAlign::Start);
}
