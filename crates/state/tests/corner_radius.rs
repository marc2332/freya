use freya_engine::prelude::*;
use freya_node_state::CornerRadius;

#[test]
fn smooth_corner_radius() {
    let corner_radius = CornerRadius {
        top_left: 12.,
        top_right: 10.,
        bottom_left: 5.0,
        bottom_right: 3.0,
        smoothing: 20.,
    };

    let rect = RRect::default();

    let path = corner_radius.smoothed_path(rect);

    assert!(path.is_valid());
    assert!(path.is_last_contour_closed());
}
