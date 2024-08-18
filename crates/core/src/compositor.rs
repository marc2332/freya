use std::ops::{
    Deref,
    DerefMut,
};

use freya_common::{
    CompositorDirtyNodes,
    Layers,
};
use freya_native_core::{
    prelude::NodeImmutable,
    NodeId,
};
use itertools::sorted;
use rustc_hash::FxHashMap;
use torin::prelude::{
    Area,
    LayoutNode,
    Torin,
};

use crate::{
    dom::DioxusNode,
    prelude::{
        DioxusDOM,
        ElementUtils,
        ElementUtilsResolver,
        ElementWithUtils,
    },
};

/// Text-like elements with shadows are the only type of elements
/// whose drawing area
///     1. Can affect other nodes
///     2. Are not part of their layout
///
/// Therefore a special cache is needed to be able to mark as dirty the previous area
/// where the shadow of the text was.
#[derive(Clone, Default, Debug)]
pub struct CompositorCache(FxHashMap<NodeId, Area>);

impl Deref for CompositorCache {
    type Target = FxHashMap<NodeId, Area>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CompositorCache {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

    /// Checks if the area (in case of being any) interesects with another area.
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
    #[inline]
    pub fn get_drawing_area(
        node_id: NodeId,
        layout: &Torin<NodeId>,
        rdom: &DioxusDOM,
        scale_factor: f32,
    ) -> Option<Area> {
        let layout_node = layout.get(node_id)?;
        let node = rdom.get(node_id)?;
        let utils = node.node_type().tag()?.utils()?;

        Some(utils.drawing_area(layout_node, &node, scale_factor))
    }

    #[inline]
    pub fn with_utils<T>(
        node_id: NodeId,
        layout: &Torin<NodeId>,
        rdom: &DioxusDOM,
        run: impl FnOnce(DioxusNode, ElementWithUtils, &LayoutNode) -> T,
    ) -> Option<T> {
        let layout_node = layout.get(node_id)?;
        let node = rdom.get(node_id)?;
        let utils = node.node_type().tag()?.utils()?;

        Some(run(node, utils, layout_node))
    }

    /// The compositor runs from the bottom layers to the top and viceversa to check what Nodes might be affected by the
    /// dirty area. How a Node is checked is by calculating its drawing area which consists of its layout area plus any possible
    /// outer effect such as shadows and borders.
    /// Calculating the drawing area might get expensive so we cache them in the `cached_areas` map to make the second layers run faster
    /// (top to bottom).
    /// In addition to that, nodes that have already been united to the dirty area are removed from the `running_layers` to avoid being checked again
    /// at the second layers (top to bottom).
    #[allow(clippy::too_many_arguments)]
    pub fn run<'a>(
        &mut self,
        dirty_nodes: &mut CompositorDirtyNodes,
        dirty_area: &mut CompositorDirtyArea,
        cache: &mut CompositorCache,
        layers: &'a Layers,
        dirty_layers: &'a mut Layers,
        layout: &Torin<NodeId>,
        rdom: &DioxusDOM,
        scale_factor: f32,
    ) -> &'a Layers {
        if self.full_render {
            for nodes in layers.values() {
                for node_id in nodes {
                    Self::with_utils(*node_id, layout, rdom, |node_ref, utils, layout_node| {
                        if utils.needs_cached_area(&node_ref) {
                            let area = utils.drawing_area(layout_node, &node_ref, scale_factor);
                            // Cache the drawing area so it can be invalidated in the next frame
                            cache.insert(*node_id, area);
                        }
                    });
                }
            }
            dirty_nodes.clear();
            dirty_area.take();
            self.full_render = false;
            return layers;
        }

        let mut run_layers = layers.clone();

        let mut run_check = |layer: i16, node_id: &NodeId| -> bool {
            Self::with_utils(*node_id, layout, rdom, |node_ref, utils, layout_node| {
                // Use the cached area to invalidate the previous frame area if necessary
                let cached_area = cache.get(node_id);
                let needs_cached_area = utils.needs_cached_area(&node_ref);

                let area = utils.drawing_area(layout_node, &node_ref, scale_factor);

                let is_dirty = dirty_nodes.remove(node_id);
                let cached_area_is_invalidated = cached_area
                    .map(|cached_area| dirty_area.intersects(cached_area))
                    .unwrap_or_default();

                let is_invalidated =
                    is_dirty || cached_area_is_invalidated || dirty_area.intersects(&area);

                if is_invalidated {
                    // Save this node to the layer it corresponds for rendering
                    dirty_layers.insert_node_in_layer(*node_id, layer);

                    // Expand the dirty area with the cached area so it gets cleaned up
                    if cached_area_is_invalidated {
                        dirty_area.unite_or_insert(cached_area.unwrap());
                    }

                    // Cache the drawing area so it can be invalidated in the next frame
                    if needs_cached_area {
                        cache.insert(*node_id, area);
                    }

                    // Expand the dirty area with only nodes who have actually changed
                    if is_dirty {
                        dirty_area.unite_or_insert(&area);
                    }
                }

                !is_invalidated
            })
            .unwrap_or_default()
        };

        for (layer, nodes) in sorted(run_layers.iter_mut()) {
            nodes.retain(|node_id| run_check(*layer, node_id))
        }
        for (layer, nodes) in sorted(run_layers.iter_mut()) {
            nodes.retain(|node_id| run_check(*layer, node_id))
        }

        dirty_nodes.drain();

        dirty_layers
    }

    /// Reset the compositor, thus causing a full render in the next frame.
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
    pub async fn button_drawing() {
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
            let rdom = fdom.rdom();
            let mut compositor_dirty_area = fdom.compositor_dirty_area();
            let mut compositor_dirty_nodes = fdom.compositor_dirty_nodes();
            let mut compositor_cache = fdom.compositor_cache();

            let mut dirty_layers = Layers::default();

            // Process what nodes need to be rendered
            let rendering_layers = compositor.run(
                &mut *compositor_dirty_nodes,
                &mut *compositor_dirty_area,
                &mut compositor_cache,
                &*layers,
                &mut dirty_layers,
                &layout,
                rdom,
                1.0f32,
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

    #[tokio::test]
    pub async fn paragraph_drawing() {
        fn compositor_app() -> Element {
            let mut msg_state = use_signal(|| true);
            let mut shadow_state = use_signal(|| true);

            let msg = if msg_state() { "12" } else { "23" };
            let shadow = if shadow_state() {
                "-40 0 20 black"
            } else {
                "none"
            };

            rsx!(
                rect {
                    height: "200",
                    width: "200",
                    direction: "horizontal",
                    rect {
                        onclick: move |_| msg_state.toggle(),
                        height: "200",
                        width: "200",
                        background: "red"
                    }
                    paragraph {
                        onclick: move |_| shadow_state.toggle(),
                        text {
                            font_size: "75",
                            font_weight: "bold",
                            text_shadow: "{shadow}",
                            "{msg}"
                        }
                    }
                }
            )
        }

        let mut compositor = Compositor::default();
        let mut utils = launch_test(compositor_app);
        let root = utils.root();
        utils.wait_for_update().await;

        assert_eq!(root.get(0).get(1).get(0).get(0).text(), Some("12"));

        fn run_compositor(utils: &TestingHandler, compositor: &mut Compositor) -> (Layers, Layers) {
            let sdom = utils.sdom();
            let fdom = sdom.get();
            let layout = fdom.layout();
            let layers = fdom.layers();
            let rdom = fdom.rdom();
            let mut compositor_dirty_area = fdom.compositor_dirty_area();
            let mut compositor_dirty_nodes = fdom.compositor_dirty_nodes();
            let mut compositor_cache = fdom.compositor_cache();

            let mut dirty_layers = Layers::default();

            // Process what nodes need to be rendered
            let rendering_layers = compositor.run(
                &mut *compositor_dirty_nodes,
                &mut *compositor_dirty_area,
                &mut compositor_cache,
                &*layers,
                &mut dirty_layers,
                &layout,
                rdom,
                1.0f32,
            );

            compositor_dirty_area.take();

            (layers.clone(), rendering_layers.clone())
        }

        let (layers, rendering_layers) = run_compositor(&utils, &mut compositor);
        // First render is always a full render
        assert_eq!(layers, rendering_layers);

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5.0, 5.0).into(),
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

        // Root + First rect + Paragraph + Second rect
        assert_eq!(painted_nodes, 4);

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (205.0, 5.0).into(),
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

        // Root + First rect + Paragraph + Second rect
        assert_eq!(painted_nodes, 4);

        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (5.0, 5.0).into(),
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

        // Root + First rect + Paragraph
        assert_eq!(painted_nodes, 2);
    }
}
