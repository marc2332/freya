use freya_engine::prelude::*;
use freya_node_state::{Fill, GradientStop, LinearGradient, Parse, Shadow, ShadowPosition};

#[test]
fn parse_big_shadow() {
    let shadow = Shadow::parse("1 2 50 25.0 red");
    assert_eq!(
        shadow,
        Ok(Shadow {
            x: 1.0,
            y: 2.0,
            blur: 50.0,
            spread: 25.0,
            fill: Fill::Color(Color::RED),
            position: ShadowPosition::Normal
        })
    );
}

#[test]
fn parse_inset_shadow() {
    let shadow = Shadow::parse("inset 1 2 50 25.0 red");
    assert_eq!(
        shadow,
        Ok(Shadow {
            x: 1.0,
            y: 2.0,
            blur: 50.0,
            spread: 25.0,
            fill: Fill::Color(Color::RED),
            position: ShadowPosition::Inset
        })
    );
}

#[test]
fn parse_shadow_with_assumed_spread() {
    let shadow = Shadow::parse("inset 1 2 50 red");
    assert_eq!(
        shadow,
        Ok(Shadow {
            x: 1.0,
            y: 2.0,
            blur: 50.0,
            spread: 0.0,
            fill: Fill::Color(Color::RED),
            position: ShadowPosition::Inset
        })
    );
}

#[test]
fn parse_gradient_shadow() {
    let shadow = Shadow::parse("inset 1 2 50 linear-gradient(red 0%, blue 100%)");
    assert_eq!(
        shadow,
        Ok(Shadow {
            x: 1.0,
            y: 2.0,
            blur: 50.0,
            spread: 0.0,
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
            position: ShadowPosition::Inset
        })
    );
}
