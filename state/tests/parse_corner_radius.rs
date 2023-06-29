use freya_node_state::{CornerRadius, Parse};

#[test]
fn parse_basic_corner_radius() {
    assert_eq!(
        CornerRadius::parse("3"),
        Ok(CornerRadius {
            top_left: 3.0,
            top_right: 3.0,
            bottom_left: 3.0,
            bottom_right: 3.0,
            smoothing: 0.0
        })
    );
}
#[test]
fn parse_two_value_radius() {
    assert_eq!(
        CornerRadius::parse("2 4"),
        Ok(CornerRadius {
            top_left: 2.0,
            top_right: 2.0,
            bottom_left: 4.0,
            bottom_right: 4.0,
            smoothing: 0.0,
        })
    );
}

#[test]
fn parse_four_value_radius() {
    assert_eq!(
        CornerRadius::parse("2 4 3 1"),
        Ok(CornerRadius {
            top_left: 2.0,
            top_right: 4.0,
            bottom_left: 3.0,
            bottom_right: 1.0,
            smoothing: 0.0,
        })
    );
}

#[test]
fn invalid_radius() {
    let extra_value = CornerRadius::parse("1 2 4 3 1");
    let bad_value_count = CornerRadius::parse("4 3 1");
    let bad_unit = CornerRadius::parse("4deg 3");
    let incorrect_separator = CornerRadius::parse("4, 3, 2, 1");

    assert_eq!(extra_value.is_err(), true);
    assert_eq!(bad_value_count.is_err(), true);
    assert_eq!(bad_unit.is_err(), true);
    assert_eq!(incorrect_separator.is_err(), true);
}
