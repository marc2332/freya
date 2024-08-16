use std::ops::{
    Deref,
    DerefMut,
};

use freya_native_core::NodeId;
use itertools::sorted;
use rustc_hash::FxHashSet;
use torin::prelude::Area;

use crate::Layers;

#[derive(Clone, Default, Debug)]
pub struct CompositorDirtyNodes(FxHashSet<NodeId>);

impl Deref for CompositorDirtyNodes {
    type Target = FxHashSet<NodeId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CompositorDirtyNodes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CompositorDirtyNodes {
    /// Mark a certain node as invalidated.
    pub fn invalidate(&mut self, node_id: NodeId) {
        self.0.insert(node_id);
    }
}

#[derive(Clone, Default, Debug)]
pub struct CompositorDirtyArea(Option<Area>);

impl CompositorDirtyArea {
    /// Take the area, leaving nothing behind.
    pub fn take(&mut self) -> Option<Area> {
        self.0.take()
    }

    /// Unite the area or insert it if none is yet present.
    pub fn unite_or_insert(&mut self, other: &Area) {
        if let Some(dirty_area) = &mut self.0 {
            *dirty_area = dirty_area.union(other);
        } else {
            self.0 = Some(*other);
        }
    }

    /// Round the dirty area to the out bounds to prevent float pixel issues.
    pub fn round_out(&mut self) {
        if let Some(dirty_area) = &mut self.0 {
            *dirty_area = dirty_area.round_out();
        }
    }

    pub fn intersects(&self, other: &Area) -> bool {
        self.0
            .map(|dirty_area| dirty_area.intersects(other))
            .unwrap_or_default()
    }
}

impl Deref for CompositorDirtyArea {
    type Target = Option<Area>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Compositor {
    full_render: bool,
}

impl Default for Compositor {
    fn default() -> Self {
        Self { full_render: true }
    }
}

impl Compositor {
    /// Run the compositor to obtain the rendering layers
    /// and finish measuring the dirty area given based on the intersected layers.
    pub fn run<'a>(
        &mut self,
        dirty_nodes: &mut CompositorDirtyNodes,
        dirty_area: &mut CompositorDirtyArea,
        layers: &'a Layers,
        dirty_layers: &'a mut Layers,
        get_drawing_area: impl Fn(&NodeId) -> Option<Area>,
    ) -> &'a Layers {
        if self.full_render {
            dirty_nodes.clear();
            dirty_area.take();
            self.full_render = false;
            return layers;
        }

        let mut run_check = |layer: i16, nodes: &[NodeId]| {
            for node_id in nodes {
                let Some(area) = get_drawing_area(node_id) else {
                    continue;
                };
                let is_invalidated = dirty_nodes.contains(node_id);
                let is_area_invalidated = dirty_area.intersects(&area);

                if is_invalidated || is_area_invalidated {
                    // Save this node to the layer it corresponds for rendering later
                    dirty_layers.insert_node_in_layer(*node_id, layer);

                    // Expand the dirty area with only nodes who have actually changed
                    if is_invalidated {
                        dirty_area.unite_or_insert(&area);
                    }
                }
            }
        };

        // From bottom to top
        for (layer, nodes) in sorted(layers.iter()) {
            run_check(*layer, nodes);
        }

        // From top to bottom
        for (layer, nodes) in sorted(layers.iter()).rev() {
            run_check(*layer, nodes);
        }

        dirty_nodes.drain();

        dirty_layers
    }

    pub fn reset(&mut self) {
        self.full_render = true;
    }
}

#[cfg(test)]
mod test {
    use freya::{
        common::*,
        prelude::*,
    };
    use freya_testing::prelude::*;
    use itertools::sorted;

    #[tokio::test]
    pub async fn compositor() {
        fn compositor_app() -> Element {
            let mut count = use_signal(|| 0);

            rsx!(
                rect {
                    height: "50%",
                    width: "100%",
                    main_align: "center",
                    cross_align: "center",
                    background: "rgb(0, 119, 182)",
                    color: "white",
                    shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
                    label {
                        font_size: "75",
                        font_weight: "bold",
                        "{count}"
                    }
                }
                rect {
                    height: "50%",
                    width: "100%",
                    main_align: "center",
                    cross_align: "center",
                    direction: "horizontal",
                    Button {
                        onclick: move |_| count += 1,
                        label { "Increase" }
                    }
                }
            )
        }

        let mut compositor = Compositor::default();
        let mut utils = launch_test(compositor_app);
        let root = utils.root();
        let label = root.get(0).get(0);
        utils.wait_for_update().await;

        assert_eq!(label.get(0).text(), Some("0"));

        fn run_compositor(utils: &TestingHandler, compositor: &mut Compositor) -> (Layers, Layers) {
            let sdom = utils.sdom();
            let fdom = sdom.get();
            let layout = fdom.layout();
            let layers = fdom.layers();
            let mut compositor_dirty_area = fdom.compositor_dirty_area();
            let mut compositor_dirty_nodes = fdom.compositor_dirty_nodes();

            let mut dirty_layers = Layers::default();

            // Process what nodes need to be rendered
            let rendering_layers = compositor.run(
                &mut *compositor_dirty_nodes,
                &mut *compositor_dirty_area,
                &*layers,
                &mut dirty_layers,
                |node_id| {
                    layout
                        .get(*node_id)
                        .map(|layout_node| layout_node.visible_area()) // TODO: actuall consider drawing area
                },
            );

            compositor_dirty_area.take();

            (layers.clone(), rendering_layers.clone())
        }

        let (layers, rendering_layers) = run_compositor(&utils, &mut compositor);
        // First render is always a full render
        assert_eq!(layers, rendering_layers);

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (275.0, 375.0).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        let (_, rendering_layers) = run_compositor(&utils, &mut compositor);
        let mut painted_nodes = 0;
        for (_, nodes) in sorted(rendering_layers.iter()) {
            let sdom = utils.sdom();
            let fdom = sdom.get();
            let layout = fdom.layout();
            for node_id in nodes {
                if layout.get(*node_id).is_some() {
                    painted_nodes += 1;
                }
            }
        }

        // Root + Second rect + Button's internal rect + Button's label
        assert_eq!(painted_nodes, 4);

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (275.0, 375.0).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;

        assert_eq!(label.get(0).text(), Some("1"));
    }
}
