use freya_node_state::{parse_shadow, ShadowSettings};
use skia_safe::Color;

#[test]
fn parse_big_shadow() {
    let shadow = parse_shadow("1 2 50 25.0 red");
    assert_eq!(shadow, Some(ShadowSettings {
        x: 1.0,
        y: 2.0,
        intensity: 50,
        size: 25.0,
        color: Color::RED
    }));
}
