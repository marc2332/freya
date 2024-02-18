use freya_engine::prelude::*;
use freya_node_state::CornerRadius;

#[test]
fn smooth_corner_radius() {
    let corner_radius = CornerRadius {
        top_left: 12.,
        top_right: 10.,
        bottom_left: 5.,
        bottom_right: 3.,
        smoothing: 20.,
    };

    let rounded_rect = RRect::new_rect_radii(
        Rect::new(5., 12., 8., 60.),
        &[
            (15., 15.).into(),
            (30., 30.).into(),
            (5., 5.).into(),
            (20., 20.).into(),
        ],
    );

    let path = corner_radius.smoothed_path(rounded_rect);

    assert!(path.is_valid());
    assert!(path.is_last_contour_closed());
}
