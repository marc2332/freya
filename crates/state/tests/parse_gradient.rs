use freya_engine::prelude::*;
use freya_node_state::{
    GradientStop,
    LinearGradient,
    Parse,
};

#[test]
fn parse_basic_gradient() {
    assert_eq!(
        LinearGradient::parse_value("linear-gradient(red 0%, blue 100%)"),
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
fn parse_rgb_hsl_gradient() {
    assert_eq!(
        LinearGradient::parse_value("linear-gradient(0deg, rgb(255, 0, 0) 0%, blue 100%)"),
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
        LinearGradient::parse_value("linear-gradient(45deg, red 0%, blue 100%)"),
        Ok(LinearGradient {
            angle: f32::to_radians(45.0),
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
fn invalid_gradients() {
    let incorrect_name =
        LinearGradient::parse_value("lkdsjfalkasdasdjaslkfjsdklfs(red 0%, blue 100%)");
    let extra_lparen = LinearGradient::parse_value("linear-gradient((red 0%, blue 100%)");
    let extra_rparen = LinearGradient::parse_value("linear-gradient(red 0%, blue 100%))");
    let missing_rparen = LinearGradient::parse_value("linear-gradient(red 0%, blue 100%");
    let missing_commas = LinearGradient::parse_value("linear-gradient(red 0% blue 100%)");
    let extra_commas = LinearGradient::parse_value("linear-gradient(red 0%, blue 100%,)");
    let extra_stop_component =
        LinearGradient::parse_value("linear-gradient(red 0% something, blue 100%)");
    let bad_angle_unit = LinearGradient::parse_value("linear-gradient(45ft, red 0%, blue 100%,)");
    let bad_offset_unit =
        LinearGradient::parse_value("linear-gradient(45deg, red 0atm, blue 100kpa)");
    let missing_color = LinearGradient::parse_value("linear-gradient(45deg, 0%, blue 100%)");
    let missing_offset = LinearGradient::parse_value("linear-gradient(45deg, red, blue 100%)");

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
