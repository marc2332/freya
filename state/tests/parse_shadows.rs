use freya_node_state::{parse_shadow, ShadowPosition, ShadowSettings};
use skia_safe::Color;

const SCALE_FACTOR: f32 = 1.0;

#[test]
fn parse_big_shadow() {
    let shadow = parse_shadow("1 2 50 25.0 red", SCALE_FACTOR);
    assert_eq!(
        shadow,
        Some(ShadowSettings {
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
    let shadow = parse_shadow("inset 1 2 50 25.0 red", SCALE_FACTOR);
    assert_eq!(
        shadow,
        Some(ShadowSettings {
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
    let shadow = parse_shadow("inset 1 2 50 red", SCALE_FACTOR);
    assert_eq!(
        shadow,
        Some(ShadowSettings {
            x: 1.0,
            y: 2.0,
            blur: 50.0,
            spread: 0.0,
            color: Color::RED,
            position: ShadowPosition::Inset
        })
    );
}
