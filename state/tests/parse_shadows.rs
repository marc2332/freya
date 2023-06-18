use freya_node_state::{Parse, Shadow, ShadowPosition};
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
            color: Color::RED,
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
            color: Color::RED,
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
            color: Color::RED,
            position: ShadowPosition::Inset
        })
    );
}
