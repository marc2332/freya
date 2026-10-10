use freya_core::{
    node_id::NodeId,
    render_commands::NodeRecording,
    render_state::RenderState,
};
use torin::prelude::{
    Area,
    Point2D,
    Size2D,
};

#[test]
fn invalidate_area_refreshes_intersections_without_expanding_damage() {
    let mut state = RenderState::default();
    let area = Area::new(Point2D::new(0., 0.), Size2D::new(40., 40.));
    for (node_id, bounds) in [
        (1, Area::new(Point2D::new(0., 0.), Size2D::new(500., 500.))),
        (2, area),
        (3, area.translate((80., 0.).into())),
    ] {
        state.cache.insert(
            NodeId::from(node_id),
            NodeRecording {
                commands: Vec::new(),
                bounds: Some(bounds),
                samples_backdrop: false,
            },
        );
    }

    state.invalidate_area(area);

    assert_eq!(state.area_invalidated.len(), 2);
    assert!(state.area_invalidated.contains(&NodeId::from(1)));
    assert!(state.area_invalidated.contains(&NodeId::from(2)));
    assert_eq!(state.damage.rects(), &[area]);
    assert!(!state.damage.is_full());
}
