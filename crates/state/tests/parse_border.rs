use freya_engine::prelude::*;
use freya_node_state::{
    Border,
    BorderAlignment,
    BorderStyle,
    Fill,
    GradientStop,
    LinearGradient,
    Parse,
};

#[test]
fn parse_basic_border() {
    let border = Border::parse_value("1 solid red");

    assert_eq!(
        border,
        Ok(Border {
            width: 1.0,
            fill: Fill::Color(Color::RED),
            style: BorderStyle::Solid,
            alignment: BorderAlignment::Inner
        })
    );
}

#[test]
fn parse_gradient_border() {
    let shadow = Border::parse_value("1 solid linear-gradient(red 0%, blue 100%)");
    assert_eq!(
        shadow,
        Ok(Border {
            width: 1.0,
            fill: Fill::LinearGradient(LinearGradient {
                angle: 0.0,
                stops: vec![
                    GradientStop {
                        color: Color::RED,
                        offset: 0.0,
                    },
                    GradientStop {
                        color: Color::BLUE,
                        offset: 1.0,
                    }
                ]
            }),
            style: BorderStyle::Solid,
            alignment: BorderAlignment::Inner
        })
    );
}

#[test]
fn parse_border_alignments() {
    let inner = BorderAlignment::parse_value("inner");
    let outer = BorderAlignment::parse_value("outer");
    let center = BorderAlignment::parse_value("center");
    let invalid = BorderAlignment::parse_value("invalid");

    assert_eq!(inner, Ok(BorderAlignment::Inner));
    assert_eq!(outer, Ok(BorderAlignment::Outer));
    assert_eq!(center, Ok(BorderAlignment::Center));
    assert!(invalid.is_err());
}

#[test]
fn parse_border_style() {
    let solid = Border::parse_value("1 solid red");
    let invalid_style = Border::parse_value("1 rust red");
    let invalid_width = Border::parse_value("rust solid red");

    assert_eq!(
        solid,
        Ok(Border {
            width: 1.0,
            fill: Fill::Color(Color::RED),
            style: BorderStyle::Solid,
            alignment: BorderAlignment::default()
        })
    );

    assert!(invalid_style.is_err());
    assert!(invalid_width.is_err());
}
