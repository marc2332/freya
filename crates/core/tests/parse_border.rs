use freya_core::{parsing::Parse, values::{
    Border,
    BorderAlignment,
    BorderWidth,
    Fill,
    GradientStop,
    LinearGradient,
}};
use freya_engine::prelude::*;

#[test]
fn parse_basic_border() {
    let border = Border::parse("1 inner red");

    assert_eq!(
        border,
        Ok(Border {
            width: BorderWidth {
                top: 1.0,
                right: 1.0,
                bottom: 1.0,
                left: 1.0,
            },
            fill: Fill::Color(Color::RED),
            alignment: BorderAlignment::Inner
        })
    );
}

#[test]
fn parse_border_widths() {
    let one_width = Border::parse("2 inner red");
    let two_widths = Border::parse("1 2 inner red");
    let three_widths = Border::parse("1 2 3 inner red");
    let four_widths = Border::parse("1 2 3 4 inner red");

    assert_eq!(
        one_width,
        Ok(Border {
            width: BorderWidth {
                top: 2.0,
                right: 2.0,
                bottom: 2.0,
                left: 2.0,
            },
            fill: Fill::Color(Color::RED),
            alignment: BorderAlignment::Inner
        })
    );

    assert_eq!(
        two_widths,
        Ok(Border {
            width: BorderWidth {
                top: 1.0,
                right: 2.0,
                bottom: 1.0,
                left: 2.0,
            },
            fill: Fill::Color(Color::RED),
            alignment: BorderAlignment::Inner
        })
    );

    assert_eq!(
        three_widths,
        Ok(Border {
            width: BorderWidth {
                top: 1.0,
                right: 2.0,
                bottom: 3.0,
                left: 2.0,
            },
            fill: Fill::Color(Color::RED),
            alignment: BorderAlignment::Inner
        })
    );

    assert_eq!(
        four_widths,
        Ok(Border {
            width: BorderWidth {
                top: 1.0,
                right: 2.0,
                bottom: 3.0,
                left: 4.0,
            },
            fill: Fill::Color(Color::RED),
            alignment: BorderAlignment::Inner
        })
    );
}

#[test]
fn parse_gradient_border() {
    let shadow = Border::parse("1 inner linear-gradient(red 0%, blue 100%)");
    assert_eq!(
        shadow,
        Ok(Border {
            width: BorderWidth {
                top: 1.0,
                right: 1.0,
                bottom: 1.0,
                left: 1.0,
            },
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
            alignment: BorderAlignment::Inner
        })
    );
}

#[test]
fn parse_border_alignments() {
    let inner = BorderAlignment::parse("inner");
    let outer = BorderAlignment::parse("outer");
    let center = BorderAlignment::parse("center");
    let invalid = BorderAlignment::parse("invalid");

    assert_eq!(inner, Ok(BorderAlignment::Inner));
    assert_eq!(outer, Ok(BorderAlignment::Outer));
    assert_eq!(center, Ok(BorderAlignment::Center));
    assert_eq!(invalid, Ok(BorderAlignment::Inner));
}
