use freya_node_state::{parse_border, BorderSettings, BorderStyle, BorderAlignment};
use skia_safe::Color;

#[test]
fn parse_basic_border() {
    let border = parse_border("1 solid red");

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
    let inner = parse_border("1 solid red inner");
    let outer = parse_border("1 solid red outer");
    let center = parse_border("1 solid red center");
    let invalid = parse_border("1 solid red invalid");

    assert_eq!(
        inner,
        Some(BorderSettings {
			width: 1.0,
			color: Color::RED,
			style: BorderStyle::Solid,
			alignment: BorderAlignment::Inner
        })
    );
	assert_eq!(
        outer,
        Some(BorderSettings {
			width: 1.0,
			color: Color::RED,
			style: BorderStyle::Solid,
			alignment: BorderAlignment::Outer
        })
    );
	assert_eq!(
        center,
        Some(BorderSettings {
			width: 1.0,
			color: Color::RED,
			style: BorderStyle::Solid,
			alignment: BorderAlignment::Center
        })
    );
	assert_eq!(
        invalid,
        Some(BorderSettings {
			width: 1.0,
			color: Color::RED,
			style: BorderStyle::Solid,
			alignment: BorderAlignment::Inner
        })
    );
}

#[test]
fn parse_border_style() {
    let solid = parse_border("1 solid red inner");
    let none = parse_border("1 solid red outer");
    let invalid = parse_border("1 solid red unknown");

    assert_eq!(
        solid,
        Some(BorderSettings {
			width: 1.0,
			color: Color::RED,
			style: BorderStyle::Solid,
			alignment: BorderAlignment::Inner
        })
    );
	assert_eq!(
        none,
        Some(BorderSettings {
			width: 1.0,
			color: Color::RED,
			style: BorderStyle::None,
			alignment: BorderAlignment::Inner
        })
    );
	assert_eq!(
        invalid,
        Some(BorderSettings {
			width: 1.0,
			color: Color::RED,
			style: BorderStyle::None,
			alignment: BorderAlignment::Inner
        })
    );
}
