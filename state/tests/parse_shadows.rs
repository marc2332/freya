use freya_node_state::{parse_shadow, ShadowSettings};
use skia_safe::Color;

#[test]
fn parse_big_shadow() {
    let shadow = parse_shadow("1 2 50 25.0 red", 1.0);
    assert_eq!(
        shadow,
        Some(ShadowSettings {
            x: 1.0,
            y: 2.0,
            blur: 50.0,
            spread: 25.0,
            color: Color::RED,
            inset: false
        })
    );
}

#[test]
fn parse_inset_shadow() {
    let shadow = parse_shadow("inset 1 2 50 25.0 red", 1.0);
    assert_eq!(
        shadow,
        Some(ShadowSettings {
            x: 1.0,
            y: 2.0,
            blur: 50.0,
            spread: 25.0,
            color: Color::RED,
            inset: true
        })
    );
}

#[test]
fn parse_shadow_with_assumed_spread() {
    let shadow = parse_shadow("inset 1 2 50 red");
    assert_eq!(
        shadow,
        Some(ShadowSettings {
            x: 1.0,
            y: 2.0,
            blur: 50.0,
            spread: 0.0,
            color: Color::RED,
            inset: true
        })
    );
}
