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

        utils.drawing_area_with_viewports(layout_node, &node, layout, scale_factor)
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
                            let area =
                                utils.drawing_area(layout_node, &node_ref, layout, scale_factor);
                            // Cache the drawing area so it can be invalidated in the next frame
                            cache.insert(*node_id, area);
                        }
                    });
                }
            }
            self.full_render = false;
            return layers;
        }
        let mut running_layers = layers.clone();

        loop {
            let mut any_marked = false;

            for (layer_n, layer) in running_layers.iter_mut() {
                layer.retain(|node_id| {
                    Self::with_utils(*node_id, layout, rdom, |node_ref, utils, layout_node| {
                        let Some(area) = utils.drawing_area_with_viewports(
                            layout_node,
                            &node_ref,
                            layout,
                            scale_factor,
                        ) else {
                            return false;
                        };

                        let is_dirty = dirty_nodes.remove(node_id);

                        // Use the cached area to invalidate the previous frame area if necessary
                        let mut invalidated_cache_area =
                            cache.get(node_id).and_then(|cached_area| {
                                if is_dirty || dirty_area.intersects(cached_area) {
                                    Some(*cached_area)
                                } else {
                                    None
                                }
                            });

                        let is_invalidated = is_dirty
                            || invalidated_cache_area.is_some()
                            || dirty_area.intersects(&area);

                        if is_invalidated {
                            // Save this node to the layer it corresponds for rendering
                            dirty_layers.insert_node_in_layer(*node_id, *layer_n);

                            // Expand the dirty area with the cached area so it gets cleaned up
                            if let Some(invalidated_cache_area) = invalidated_cache_area.take() {
                                if is_dirty {
                                    dirty_area.unite_or_insert(&invalidated_cache_area);
                                    any_marked = true;
                                }
                            }

                            // Cache the drawing area so it can be invalidated in the next frame
                            if utils.needs_cached_area(&node_ref) {
                                cache.insert(*node_id, area);
                            }

                            // Expand the dirty area with only nodes who have actually changed
                            if is_dirty {
                                dirty_area.unite_or_insert(&area);
                                any_marked = true;
                            }
                        }

                        !is_invalidated
                    })
                    .unwrap_or_default()
                })
            }

            if !any_marked {
                break;
            }
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

    fn run_compositor(
        utils: &TestingHandler<()>,
        compositor: &mut Compositor,
    ) -> (Layers, Layers, usize) {
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
            &mut compositor_dirty_nodes,
            &mut compositor_dirty_area,
            &mut compositor_cache,
            &layers,
            &mut dirty_layers,
            &layout,
            rdom,
            1.0f32,
        );

        compositor_dirty_area.take();
        compositor_dirty_nodes.clear();

        let mut painted_nodes = 0;
        for (_, nodes) in sorted(rendering_layers.iter()) {
            for node_id in nodes {
                if layout.get(*node_id).is_some() {
                    painted_nodes += 1;
                }
            }
        }

        (layers.clone(), rendering_layers.clone(), painted_nodes)
    }

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

        let (layers, rendering_layers, _) = run_compositor(&utils, &mut compositor);
        // First render is always a full render
        assert_eq!(layers, rendering_layers);

        utils.move_cursor((275., 375.)).await;

        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);

        // Root + Second rect + Button's internal rect + Button's label
        assert_eq!(painted_nodes, 4);

        utils.click_cursor((275., 375.)).await;

        assert_eq!(label.get(0).text(), Some("1"));
    }

    #[tokio::test]
    pub async fn after_shadow_drawing() {
        fn compositor_app() -> Element {
            let mut height = use_signal(|| 200);
            let mut shadow = use_signal(|| 20);

            rsx!(
                rect {
                    height: "100",
                    width: "200",
                    background: "red",
                    margin: "0 0 2 0",
                    onclick: move |_| height += 10,
                }
                rect {
                    height: "{height}",
                    width: "200",
                    background: "green",
                    shadow: "0 {shadow} 1 0 rgb(0, 0, 0, 0.5)",
                    margin: "0 0 2 0",
                    onclick: move |_| height -= 10,
                }
                rect {
                    height: "100",
                    width: "200",
                    background: "blue",
                    onclick: move |_| shadow.set(-20),
                }
            )
        }

        let mut compositor = Compositor::default();
        let mut utils = launch_test(compositor_app);
        utils.wait_for_update().await;

        let (layers, rendering_layers, _) = run_compositor(&utils, &mut compositor);
        // First render is always a full render
        assert_eq!(layers, rendering_layers);

        utils.click_cursor((5., 5.)).await;

        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);

        // Root + Second rect + Third rect
        assert_eq!(painted_nodes, 3);

        utils.click_cursor((5., 150.)).await;

        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);

        // Root + Second rect + Third rect
        assert_eq!(painted_nodes, 3);

        utils.click_cursor((5., 350.)).await;

        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);

        // Root + First rect + Second rect + Third Rect
        assert_eq!(painted_nodes, 4);

        utils.click_cursor((5., 150.)).await;

        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);

        // Root + First rect + Second rect + Third Rect
        assert_eq!(painted_nodes, 4);
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
                    spacing: "2",
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

        let (layers, rendering_layers, _) = run_compositor(&utils, &mut compositor);
        // First render is always a full render
        assert_eq!(layers, rendering_layers);

        utils.click_cursor((5., 5.)).await;

        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);

        // Root + First rect + Paragraph + Second rect
        assert_eq!(painted_nodes, 4);

        utils.click_cursor((205., 5.)).await;

        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);

        // Root + First rect + Paragraph + Second rect
        assert_eq!(painted_nodes, 4);

        utils.click_cursor((5., 5.)).await;

        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);

        // Root + First rect + Paragraph
        assert_eq!(painted_nodes, 2);
    }

    #[tokio::test]
    pub async fn rotated_drawing() {
        fn compositor_app() -> Element {
            let mut rotate = use_signal(|| 0);

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
                        rotate: "{rotate}deg",
                        "Hello"
                    }
                    label {
                        "World"
                    }
                }
                rect {
                    height: "50%",
                    width: "100%",
                    main_align: "center",
                    cross_align: "center",
                    direction: "horizontal",
                    Button {
                        onclick: move |_| rotate += 1,
                        label { "Rotate" }
                    }
                }
            )
        }

        let mut compositor = Compositor::default();
        let mut utils = launch_test(compositor_app);
        utils.wait_for_update().await;

        let (layers, rendering_layers, _) = run_compositor(&utils, &mut compositor);
        // First render is always a full render
        assert_eq!(layers, rendering_layers);

        utils.click_cursor((275., 375.)).await;

        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);

        // Root + First rect + First Label + Second Label
        assert_eq!(painted_nodes, 4);
    }

    #[tokio::test]
    pub async fn rotated_shadow_drawing() {
        fn compositor_app() -> Element {
            let mut rotate = use_signal(|| 0);

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
                        rotate: "{rotate}deg",
                        text_shadow: "0 180 12 rgb(0, 0, 0, 240)",
                        "Hello"
                    }
                    label {
                        "World"
                    }
                }
                rect {
                    height: "50%",
                    width: "100%",
                    main_align: "center",
                    cross_align: "center",
                    direction: "horizontal",
                    Button {
                        onclick: move |_| rotate += 1,
                        label { "Rotate" }
                    }
                }
            )
        }

        let mut compositor = Compositor::default();
        let mut utils = launch_test(compositor_app);
        utils.wait_for_update().await;

        let (layers, rendering_layers, _) = run_compositor(&utils, &mut compositor);
        // First render is always a full render
        assert_eq!(layers, rendering_layers);

        utils.click_cursor((275., 375.)).await;

        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);

        // Everything
        assert_eq!(painted_nodes, 7);
    }

    #[tokio::test]
    pub async fn scale_drawing() {
        fn compositor_app() -> Element {
            let mut scale = use_signal(|| 1.);

            rsx!(
                rect {
                    scale: "{scale()} {scale()}",
                    height: "50%",
                    width: "100%",
                    main_align: "center",
                    cross_align: "center",
                    background: "rgb(0, 119, 182)",
                    color: "white",
                    shadow: "0 4 20 5 rgb(0, 0, 0, 80)",
                    label {
                        text_shadow: "0 180 12 rgb(0, 0, 0, 240)",
                        "Hello"
                    }
                    label {
                        "World"
                    }
                }
                rect {
                    height: "50%",
                    width: "100%",
                    main_align: "center",
                    cross_align: "center",
                    direction: "horizontal",
                    Button {
                        onclick: move |_| scale += 0.1,
                        label { "More" }
                    }
                    Button {
                        onclick: move |_| scale -= 0.1,
                        label { "Less" }
                    }
                }
            )
        }

        let mut compositor = Compositor::default();
        let mut utils = launch_test_with_config(
            compositor_app,
            TestingConfig::<()> {
                size: (400.0, 400.0).into(),
                ..TestingConfig::default()
            },
        );
        utils.wait_for_update().await;

        let (layers, rendering_layers, _) = run_compositor(&utils, &mut compositor);
        // First render is always a full render
        assert_eq!(layers, rendering_layers);

        utils.click_cursor((180., 310.)).await;
        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);
        assert_eq!(painted_nodes, 9);

        utils.click_cursor((250., 310.)).await;
        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);
        assert_eq!(painted_nodes, 9);

        utils.click_cursor((250., 310.)).await;
        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);
        assert_eq!(painted_nodes, 7);

        utils.click_cursor((250., 310.)).await;
        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);
        assert_eq!(painted_nodes, 7);

        utils.click_cursor((250., 310.)).await;
        let (_, _, painted_nodes) = run_compositor(&utils, &mut compositor);
        assert_eq!(painted_nodes, 5);
    }
}
