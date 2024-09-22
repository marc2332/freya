use freya_engine::prelude::*;
use freya_node_state::{
    ConicGradient,
    GradientStop,
    LinearGradient,
    Parse,
    RadialGradient,
};

#[test]
fn parse_basic_linear_gradient() {
    assert_eq!(
        LinearGradient::parse("linear-gradient(red 0%, blue 100%)"),
        Ok(LinearGradient {
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
        })
    );
}

#[test]
fn parse_basic_radial_gradient() {
    assert_eq!(
        RadialGradient::parse("radial-gradient(red 0%, blue 100%)"),
        Ok(RadialGradient {
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
        })
    );
}

#[test]
fn parse_basic_conic_gradient() {
    assert_eq!(
        ConicGradient::parse("conic-gradient(red 0%, blue 100%)"),
        Ok(ConicGradient {
            angle: None,
            angles: None,
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
        })
    );
}

#[test]
fn parse_conic_gradient_variants() {
    assert_eq!(
        ConicGradient::parse("conic-gradient(45deg, red 0%, blue 100%)"),
        Ok(ConicGradient {
            angle: Some(45.0),
            angles: None,
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
        })
    );

    assert_eq!(
        ConicGradient::parse("conic-gradient(45deg, from 40deg, red 0%, blue 100%)"),
        Ok(ConicGradient {
            angle: Some(45.0),
            angles: Some((40.0, 360.0)),
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
        })
    );

    assert_eq!(
        ConicGradient::parse("conic-gradient(45deg, from 40deg to 120deg, red 0%, blue 100%)"),
        Ok(ConicGradient {
            angle: Some(45.0),
            angles: Some((40.0, 120.0)),
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
        })
    );
}

#[test]
fn parse_rgb_hsl_gradient() {
    assert_eq!(
        LinearGradient::parse("linear-gradient(0deg, rgb(255, 0, 0) 0%, blue 100%)"),
        Ok(LinearGradient {
            angle: 0.0,
            stops: vec![
                GradientStop {
                    color: Color::from_rgb(255, 0, 0),
                    offset: 0.0,
                },
                GradientStop {
                    color: Color::from_rgb(0, 0, 255),
                    offset: 1.0,
                }
            ]
        })
    );
}

#[test]
fn parse_gradient_angle() {
    assert_eq!(
        LinearGradient::parse("linear-gradient(45deg, red 0%, blue 100%)"),
        Ok(LinearGradient {
            angle: 45.0,
            stops: vec![
                GradientStop {
                    color: Color::from_rgb(255, 0, 0),
                    offset: 0.0,
                },
                GradientStop {
                    color: Color::from_rgb(0, 0, 255),
                    offset: 1.0,
                }
            ]
        })
    );
}

#[test]
fn invalid_linear_gradients() {
    let incorrect_name = LinearGradient::parse("lkdsjfalkasdasdjaslkfjsdklfs(red 0%, blue 100%)");
    let extra_lparen = LinearGradient::parse("linear-gradient((red 0%, blue 100%)");
    let extra_rparen = LinearGradient::parse("linear-gradient(red 0%, blue 100%))");
    let missing_rparen = LinearGradient::parse("linear-gradient(red 0%, blue 100%");
    let missing_commas = LinearGradient::parse("linear-gradient(red 0% blue 100%)");
    let extra_commas = LinearGradient::parse("linear-gradient(red 0%, blue 100%,)");
    let extra_stop_component =
        LinearGradient::parse("linear-gradient(red 0% something, blue 100%)");
    let bad_angle_unit = LinearGradient::parse("linear-gradient(45ft, red 0%, blue 100%)");
    let bad_offset_unit = LinearGradient::parse("linear-gradient(45deg, red 0atm, blue 100kpa)");
    let missing_color = LinearGradient::parse("linear-gradient(45deg, 0%, blue 100%)");
    let missing_offset = LinearGradient::parse("linear-gradient(45deg, red, blue 100%)");

    assert!(incorrect_name.is_err());
    assert!(extra_lparen.is_err());
    assert!(extra_rparen.is_err());
    assert!(missing_rparen.is_err());
    assert!(missing_commas.is_err());
    assert!(extra_commas.is_err());
    assert!(extra_stop_component.is_err());
    assert!(bad_angle_unit.is_err());
    assert!(bad_offset_unit.is_err());
    assert!(missing_color.is_err());
    assert!(missing_offset.is_err());
}

#[test]
fn invalid_radial_gradients() {
    let incorrect_name = RadialGradient::parse("lkdsjfalkasdasdjaslkfjsdklfs(red 0%, blue 100%)");
    let extra_lparen = RadialGradient::parse("radial-gradient((red 0%, blue 100%)");
    let extra_rparen = RadialGradient::parse("radial-gradient(red 0%, blue 100%))");
    let missing_rparen = RadialGradient::parse("radial-gradient(red 0%, blue 100%");
    let missing_commas = RadialGradient::parse("radial-gradient(red 0% blue 100%)");
    let extra_commas = RadialGradient::parse("radial-gradient(red 0%, blue 100%,)");
    let extra_stop_component =
        RadialGradient::parse("radial-gradient(red 0% something, blue 100%)");
    let bad_offset_unit = RadialGradient::parse("radial-gradient(red 0atm, blue 100kpa)");
    let missing_color = RadialGradient::parse("radial-gradient(0%, blue 100%)");
    let missing_offset = RadialGradient::parse("radial-gradient(red, blue 100%)");

    assert!(incorrect_name.is_err());
    assert!(extra_lparen.is_err());
    assert!(extra_rparen.is_err());
    assert!(missing_rparen.is_err());
    assert!(missing_commas.is_err());
    assert!(extra_commas.is_err());
    assert!(extra_stop_component.is_err());
    assert!(bad_offset_unit.is_err());
    assert!(missing_color.is_err());
    assert!(missing_offset.is_err());
}

#[test]
fn invalid_conic_gradients() {
    let incorrect_name = ConicGradient::parse("lkdsjfalkasdasdjaslkfjsdklfs(red 0%, blue 100%)");
    let extra_lparen = ConicGradient::parse("conic-gradient((red 0%, blue 100%)");
    let extra_rparen = ConicGradient::parse("conic-gradient(red 0%, blue 100%))");
    let missing_rparen = ConicGradient::parse("conic-gradient(red 0%, blue 100%");
    let missing_commas = ConicGradient::parse("conic-gradient(red 0% blue 100%)");
    let extra_commas = ConicGradient::parse("conic-gradient(red 0%, blue 100%,)");
    let extra_stop_component = ConicGradient::parse("conic-gradient(red 0% something, blue 100%)");
    let bad_angle_unit = ConicGradient::parse("conic-gradient(45ft, red 0%, blue 100%)");
    let bad_offset_unit = ConicGradient::parse("conic-gradient(red 0atm, blue 100kpa)");
    let bad_angles =
        ConicGradient::parse("conic-gradient(45deg, from 60rft to 90fft, red 0%, blue 100%)");
    let missing_color = ConicGradient::parse("conic-gradient(0%, blue 100%)");
    let missing_offset = ConicGradient::parse("conic-gradient(red, blue 100%)");

    assert!(incorrect_name.is_err());
    assert!(extra_lparen.is_err());
    assert!(extra_rparen.is_err());
    assert!(missing_rparen.is_err());
    assert!(missing_commas.is_err());
    assert!(extra_commas.is_err());
    assert!(extra_stop_component.is_err());
    assert!(bad_angle_unit.is_err());
    assert!(bad_offset_unit.is_err());
    assert!(bad_angles.is_err());
    assert!(missing_color.is_err());
    assert!(missing_offset.is_err());
}
