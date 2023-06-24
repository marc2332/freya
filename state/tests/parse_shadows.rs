use freya_node_state::{Parse, Fill, Shadow, ShadowPosition};
use skia_safe::Color;

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
fn test() {
    println!("{:?}", Shadow::parse("0 8 12 2 linear-gradient(-90deg, rgb(207, 119, 243) 0%, rgb(0, 155, 255) 47%, rgb(42, 201, 219) 100%)"));
}