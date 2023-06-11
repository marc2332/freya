use freya_node_state::{
    parse_border, parse_border_align, BorderAlignment, BorderSettings, BorderStyle,
};
use skia_safe::Color;

#[test]
fn parse_basic_border() {
    let border = parse_border("1 solid red", BorderAlignment::default());

    assert_eq!(
        border,
        Some(BorderSettings {
            width: 1.0,
            color: Color::RED,
            style: BorderStyle::Solid,
            alignment: BorderAlignment::Inner
        })
    );
}

#[test]
fn parse_border_alignments() {
    let inner = parse_border_align("inner");
    let outer = parse_border_align("outer");
    let center = parse_border_align("center");
    let invalid = parse_border_align("invalid");

    assert_eq!(inner, BorderAlignment::Inner);
    assert_eq!(outer, BorderAlignment::Outer);
    assert_eq!(center, BorderAlignment::Center);
    assert_eq!(invalid, BorderAlignment::Inner);
}

#[test]
fn parse_border_style() {
    let solid = parse_border("1 solid red", BorderAlignment::default());
    let none = parse_border("1 rust red", BorderAlignment::default());
    let invalid = parse_border("rust solid red", BorderAlignment::default());

    assert_eq!(
        solid,
        Some(BorderSettings {
            width: 1.0,
            color: Color::RED,
            style: BorderStyle::Solid,
            alignment: BorderAlignment::default()
        })
    );
    assert_eq!(
        none,
        Some(BorderSettings {
            width: 1.0,
            color: Color::RED,
            style: BorderStyle::None,
            alignment: BorderAlignment::default()
        })
    );
    assert!(invalid.is_none());
}
