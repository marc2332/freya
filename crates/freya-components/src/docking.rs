use std::hash::Hash;

use freya_core::prelude::*;
use torin::{
    content::Content,
    direction::Direction,
    position::Position,
    size::Size,
};

use crate::{
    drag_drop::{
        DragZone,
        DropZone,
        use_drag,
    },
    resizable_container::{
        PanelSize,
        ResizableContainer,
        ResizablePanel,
    },
};

/// A tabbed panel containing one or more tabs.
#[derive(Clone, PartialEq, Debug)]
pub struct DockPanel<TabId, PanelId> {
    pub panel_id: PanelId,
    pub tabs: Vec<TabId>,
    pub active_tab_id: Option<TabId>,
}

impl<TabId, PanelId> DockPanel<TabId, PanelId> {
    /// Create a new panel with the given id and tabs.
    pub fn new(panel_id: PanelId, tabs: Vec<TabId>) -> Self
    where
        TabId: Clone,
    {
        let active_tab_id = tabs.first().cloned();
        Self {
            panel_id,
            tabs,
            active_tab_id,
        }
    }

    /// Insert `tab` at `position`.
    pub fn insert_tab(&mut self, tab: TabId, position: usize)
    where
        TabId: Clone + PartialEq,
    {
        let target = match self.tabs.iter().position(|item| *item == tab) {
            Some(existing) => {
                self.tabs.remove(existing);
                if position > existing {
                    position - 1
                } else {
                    position
                }
            }
            None => position,
        };
        self.active_tab_id = Some(tab.clone());
        self.tabs.insert(target.min(self.tabs.len()), tab);
    }

    /// Add `tab` at the end.
    pub fn append_tab(&mut self, tab: TabId)
    where
        TabId: Clone + PartialEq,
    {
        self.tabs.retain(|item| *item != tab);
        self.active_tab_id = Some(tab.clone());
        self.tabs.push(tab);
    }
}

/// A node in the docking tree, either a split or a tabbed panel.
#[derive(Clone, PartialEq, Debug)]
pub enum DockNode<TabId, PanelId> {
    Split {
        direction: Direction,
        children: Vec<DockNode<TabId, PanelId>>,
    },
    Panel(DockPanel<TabId, PanelId>),
}

impl<TabId, PanelId> DockNode<TabId, PanelId>
where
    TabId: Clone + PartialEq,
    PanelId: Copy + PartialEq,
{
    /// Whether this node is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            DockNode::Panel(panel) => panel.tabs.is_empty(),
            DockNode::Split { children, .. } => children.is_empty(),
        }
    }

    /// Find the panel with the given id.
    pub fn panel(&self, id: &PanelId) -> Option<&DockPanel<TabId, PanelId>> {
        match self {
            DockNode::Panel(panel) => (&panel.panel_id == id).then_some(panel),
            DockNode::Split { children, .. } => children.iter().find_map(|child| child.panel(id)),
        }
    }

    /// Mutable version of [`DockNode::panel`].
    pub fn panel_mut(&mut self, id: &PanelId) -> Option<&mut DockPanel<TabId, PanelId>> {
        match self {
            DockNode::Panel(panel) => (&panel.panel_id == id).then_some(panel),
            DockNode::Split { children, .. } => {
                children.iter_mut().find_map(|child| child.panel_mut(id))
            }
        }
    }

    /// The first (left-most) panel under this node.
    pub fn first_panel(&self) -> Option<&DockPanel<TabId, PanelId>> {
        match self {
            DockNode::Panel(panel) => Some(panel),
            DockNode::Split { children, .. } => children.iter().find_map(DockNode::first_panel),
        }
    }

    /// Mutable version of [`DockNode::first_panel`].
    pub fn first_panel_mut(&mut self) -> Option<&mut DockPanel<TabId, PanelId>> {
        match self {
            DockNode::Panel(panel) => Some(panel),
            DockNode::Split { children, .. } => {
                children.iter_mut().find_map(DockNode::first_panel_mut)
            }
        }
    }

    /// Find `tab` under this node.
    pub fn find_tab(&self, tab: &TabId) -> Option<(PanelId, usize)> {
        match self {
            DockNode::Panel(panel) => panel
                .tabs
                .iter()
                .position(|item| item == tab)
                .map(|position| (panel.panel_id, position)),
            DockNode::Split { children, .. } => {
                children.iter().find_map(|child| child.find_tab(tab))
            }
        }
    }

    /// Remove `tab` from every panel except `except_panel`.
    pub fn remove_tab_except(&mut self, tab: &TabId, except_panel: Option<&PanelId>) -> bool {
        match self {
            DockNode::Panel(panel) => {
                if except_panel == Some(&panel.panel_id) {
                    return false;
                }
                let Some(position) = panel.tabs.iter().position(|item| item == tab) else {
                    return false;
                };
                panel.tabs.remove(position);
                if panel.active_tab_id.as_ref() == Some(tab) {
                    panel.active_tab_id = panel.tabs.first().cloned();
                }
                true
            }
            DockNode::Split { children, .. } => children
                .iter_mut()
                .any(|child| child.remove_tab_except(tab, except_panel)),
        }
    }

    /// Turn the matching panel into a split that holds the old panel and a new one.
    pub fn split_panel(
        &mut self,
        panel_id: &PanelId,
        side: Side,
        new_panel: &DockPanel<TabId, PanelId>,
    ) -> bool {
        match self {
            DockNode::Panel(panel) if &panel.panel_id == panel_id => {
                let new_dock_node = DockNode::Split {
                    direction: side.direction(),
                    children: Vec::new(),
                };
                let old_dock_node = std::mem::replace(self, new_dock_node);
                if let DockNode::Split { children, .. } = self {
                    let new_node = DockNode::Panel(new_panel.clone());
                    *children = match side {
                        Side::Left | Side::Top => vec![new_node, old_dock_node],
                        Side::Right | Side::Bottom => vec![old_dock_node, new_node],
                    };
                }
                true
            }
            DockNode::Split { children, .. } => children
                .iter_mut()
                .any(|child| child.split_panel(panel_id, side, new_panel)),
            _ => false,
        }
    }

    /// Remove empty panels.
    pub fn close_empty_panels(&mut self) {
        let DockNode::Split { children, .. } = self else {
            return;
        };
        children.iter_mut().for_each(DockNode::close_empty_panels);
        children.retain(|child| !child.is_empty());
        if children.len() == 1 {
            *self = children.remove(0);
        }
    }
}

/// Which side of a panel a drop targets.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    /// The direction a split on this side runs along.
    pub fn direction(self) -> Direction {
        match self {
            Side::Left | Side::Right => Direction::Horizontal,
            Side::Top | Side::Bottom => Direction::Vertical,
        }
    }
}

/// Describes where a dragged tab should be dropped.
#[derive(Clone, PartialEq, Debug)]
pub enum DropTarget<PanelId> {
    Tab { panel_id: PanelId, position: usize },
    Center(PanelId),
    Split { panel_id: PanelId, side: Side },
}

pub trait DockingModel: 'static {
    /// Id for a tab.
    type TabId: Copy + PartialEq + Hash + 'static;
    /// Id for a panel.
    type PanelId: Copy + PartialEq + 'static;
    /// The value carried by a drag-and-drop.
    type DropValue: Clone + PartialEq + 'static + From<Self::TabId>;

    /// The current tree of panels and splits, or `None` when it's empty.
    fn root(&self) -> Option<&DockNode<Self::TabId, Self::PanelId>>;
    /// Apply a dropped [`Self::DropValue`] at `target`. Returns `true` if
    /// something changed.
    fn on_drop(&mut self, value: Self::DropValue, target: DropTarget<Self::PanelId>) -> bool;
    /// Make `tab` the active one in `panel`. Returns `true` if it was found.
    fn set_active(&mut self, panel: Self::PanelId, tab: Self::TabId) -> bool;
}

/// The payload carried by a drag in a docking area.
#[derive(Clone, PartialEq, Debug)]
pub struct DockDrag<Value> {
    value: Value,
}

impl<Value> DockDrag<Value> {
    /// Wrap a value to be dragged onto a docking area.
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

/// Passed to the `render_content` callback.
#[derive(Clone, PartialEq, Debug)]
pub struct ContentContext<TabId, PanelId> {
    /// The id of the panel this content belongs to.
    pub panel_id: PanelId,
    /// The panel's active tab, or `None` when the panel has no tabs.
    pub tab_id: Option<TabId>,
    /// Number of tabs open in the panel.
    pub tab_count: usize,
}

/// Passed to the `render_tab_bar` callback.
pub struct TabBarContext<PanelId> {
    pub panel_id: PanelId,
    /// The tab header elements to lay out in the bar.
    pub tab_children: Vec<Element>,
    /// Number of tabs open in the panel.
    pub tab_count: usize,
}

/// Passed to the `render_tab` callback.
#[derive(Clone, PartialEq, Debug)]
pub struct TabContext<TabId> {
    pub tab_id: TabId,
    /// True when a drag is going on and the cursor is over this tab.
    pub is_drop_target: bool,
}

/// The drop target currently under the cursor during a drag.
#[derive(Clone, Copy, PartialEq)]
enum HoverTarget<TabId> {
    Tab(TabId),
    Edge(Side),
    Center,
}

/// Set `hover` to `target` on enter, or clear it on leave if it still points at `target`.
fn toggle_hover<TabId: Copy + PartialEq + 'static>(
    mut hover: State<Option<HoverTarget<TabId>>>,
    target: HoverTarget<TabId>,
    hovering: bool,
) {
    if hovering {
        hover.set(Some(target));
    } else if hover() == Some(target) {
        hover.set(None);
    }
}

/// Make the center take 50% of the width.
const MIDDLE_FLEX: f32 = 2.0;

/// The render callbacks.
#[derive(Clone, PartialEq)]
struct Renderers<TabId: 'static, PanelId: 'static> {
    content: Callback<ContentContext<TabId, PanelId>, Element>,
    tab: Callback<TabContext<TabId>, Element>,
    drag: Callback<TabId, Element>,
    bar: Callback<TabBarContext<PanelId>, Element>,
}

pub struct DockingArea<M: DockingModel> {
    controller: Writable<M>,
    renderers: Renderers<M::TabId, M::PanelId>,
    preview_element: Option<Element>,
    key: DiffKey,
}

impl<M: DockingModel> Clone for DockingArea<M> {
    fn clone(&self) -> Self {
        Self {
            controller: self.controller.clone(),
            renderers: self.renderers.clone(),
            preview_element: self.preview_element.clone(),
            key: self.key.clone(),
        }
    }
}

impl<M: DockingModel> PartialEq for DockingArea<M> {
    fn eq(&self, other: &Self) -> bool {
        self.controller == other.controller
            && self.renderers == other.renderers
            && self.preview_element == other.preview_element
            && self.key == other.key
    }
}

impl<M: DockingModel> KeyExt for DockingArea<M> {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl<M: DockingModel> DockingArea<M> {
    pub fn new(
        controller: impl Into<Writable<M>>,
        render_content: impl Into<Callback<ContentContext<M::TabId, M::PanelId>, Element>>,
        render_tab: impl Into<Callback<TabContext<M::TabId>, Element>>,
        render_drag: impl Into<Callback<M::TabId, Element>>,
        render_tab_bar: impl Into<Callback<TabBarContext<M::PanelId>, Element>>,
    ) -> Self {
        Self {
            controller: controller.into(),
            renderers: Renderers {
                content: render_content.into(),
                tab: render_tab.into(),
                drag: render_drag.into(),
                bar: render_tab_bar.into(),
            },
            preview_element: None,
            key: DiffKey::default(),
        }
    }

    /// Preview shown over the drop target while dragging.
    pub fn preview_element(mut self, element: impl IntoElement) -> Self {
        self.preview_element = Some(element.into_element());
        self
    }
}

impl<M: DockingModel> Component for DockingArea<M> {
    fn render(&self) -> impl IntoElement {
        let controller = self.controller.clone();
        let renderers = self.renderers.clone();
        let preview_element = self.preview_element.clone();

        let node = controller.read().root().cloned();

        rect().expanded().map(node, move |element, root| {
            element.child(render_node(root, controller, renderers, preview_element))
        })
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Draw one node of the tree (a split or a panel).
fn render_node<M: DockingModel>(
    node: DockNode<M::TabId, M::PanelId>,
    controller: Writable<M>,
    renderers: Renderers<M::TabId, M::PanelId>,
    preview_element: Option<Element>,
) -> Element {
    match node {
        DockNode::Split {
            direction,
            children,
        } => {
            let share = 100. / children.len().max(1) as f32;
            ResizableContainer::new()
                .direction(direction)
                .panels_iter(children.into_iter().map(|child| {
                    ResizablePanel::new(PanelSize::percent(share))
                        .min_size(5.)
                        .child(render_node(
                            child,
                            controller.clone(),
                            renderers.clone(),
                            preview_element.clone(),
                        ))
                }))
                .into_element()
        }
        DockNode::Panel(panel) => DockPanelView {
            panel,
            controller,
            renderers,
            preview_element,
            key: DiffKey::default(),
        }
        .into_element(),
    }
}

struct DockPanelView<M: DockingModel> {
    panel: DockPanel<M::TabId, M::PanelId>,
    controller: Writable<M>,
    renderers: Renderers<M::TabId, M::PanelId>,
    preview_element: Option<Element>,
    key: DiffKey,
}

impl<M: DockingModel> Clone for DockPanelView<M> {
    fn clone(&self) -> Self {
        Self {
            panel: self.panel.clone(),
            controller: self.controller.clone(),
            renderers: self.renderers.clone(),
            preview_element: self.preview_element.clone(),
            key: self.key.clone(),
        }
    }
}

impl<M: DockingModel> PartialEq for DockPanelView<M> {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl<M: DockingModel> KeyExt for DockPanelView<M> {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl<M: DockingModel> ComponentOwned for DockPanelView<M> {
    fn render(self) -> impl IntoElement {
        let DockPanelView {
            panel:
                DockPanel {
                    panel_id,
                    tabs,
                    active_tab_id,
                },
            controller,
            renderers,
            preview_element,
            ..
        } = self;

        let drag = use_drag::<DockDrag<M::DropValue>>();
        let is_dragging = drag.read().is_some();
        let hover = use_state(|| None::<HoverTarget<M::TabId>>);

        let hovered = is_dragging.then(&*hover).flatten();
        let tab_count = tabs.len();
        let mut tab_children: Vec<Element> = tabs
            .iter()
            .enumerate()
            .map(|(index, &tab_id)| {
                let handle = renderers.tab.call(TabContext {
                    tab_id,
                    is_drop_target: hovered == Some(HoverTarget::Tab(tab_id)),
                });
                let dragger = DragZone::<DockDrag<M::DropValue>>::new(
                    DockDrag::new(M::DropValue::from(tab_id)),
                    handle,
                )
                .drag_element(renderers.drag.call(tab_id))
                .into_element();

                let activatable = rect()
                    .on_press({
                        let mut controller = controller.clone();
                        move |_| {
                            controller.write().set_active(panel_id, tab_id);
                        }
                    })
                    .child(dragger)
                    .into_element();

                DropZone::<DockDrag<M::DropValue>>::new(activatable, {
                    let mut controller = controller.clone();
                    move |payload: DockDrag<M::DropValue>| {
                        controller.write().on_drop(
                            payload.value,
                            DropTarget::Tab {
                                panel_id,
                                position: index,
                            },
                        );
                    }
                })
                .on_drag_over(move |hovering| {
                    toggle_hover(hover, HoverTarget::Tab(tab_id), hovering)
                })
                .key(tab_id)
                .into_element()
            })
            .collect();

        tab_children.push(
            DropZone::<DockDrag<M::DropValue>>::new(rect().expanded().into_element(), {
                let mut controller = controller.clone();
                move |payload: DockDrag<M::DropValue>| {
                    controller.write().on_drop(
                        payload.value,
                        DropTarget::Tab {
                            panel_id,
                            position: tab_count,
                        },
                    );
                }
            })
            .into_element(),
        );
        let tab_bar = renderers.bar.call(TabBarContext {
            panel_id,
            tab_children,
            tab_count,
        });

        let content = renderers.content.call(ContentContext {
            panel_id,
            tab_id: active_tab_id,
            tab_count,
        });

        let overlay = is_dragging.then(move || {
            let ghost = match (hover(), preview_element) {
                (Some(HoverTarget::Edge(side)), Some(preview)) => {
                    let (width, height) = match side {
                        Side::Top | Side::Bottom => (Size::percent(100.), Size::percent(50.)),
                        Side::Left | Side::Right => (Size::percent(50.), Size::percent(100.)),
                    };
                    let position = match side {
                        Side::Top | Side::Left => Position::new_absolute(),
                        Side::Bottom => Position::new_absolute().bottom(0.),
                        Side::Right => Position::new_absolute().right(0.),
                    };
                    Some(drag_preview(position, width, height, preview))
                }
                (Some(HoverTarget::Center), Some(preview)) => Some(drag_preview(
                    Position::new_absolute(),
                    Size::percent(100.),
                    Size::percent(100.),
                    preview,
                )),
                _ => None,
            };

            let edge = |side: Side, width: Size, height: Size| -> Element {
                rect()
                    .width(width)
                    .height(height)
                    .child(drop_zone_for_side::<M>(
                        panel_id,
                        side,
                        controller.clone(),
                        hover,
                    ))
                    .into_element()
            };

            let center_drop =
                DropZone::<DockDrag<M::DropValue>>::new(rect().expanded().into_element(), {
                    let mut controller = controller.clone();
                    move |payload: DockDrag<M::DropValue>| {
                        controller
                            .write()
                            .on_drop(payload.value, DropTarget::Center(panel_id));
                    }
                })
                .on_drag_over(move |hovering| toggle_hover(hover, HoverTarget::Center, hovering))
                .into_element();

            let middle_row = rect()
                .width(Size::percent(100.))
                .height(Size::flex(MIDDLE_FLEX))
                .horizontal()
                .content(Content::flex())
                .child(edge(Side::Left, Size::flex(1.), Size::percent(100.)))
                .child(
                    rect()
                        .width(Size::flex(MIDDLE_FLEX))
                        .height(Size::percent(100.))
                        .child(center_drop),
                )
                .child(edge(Side::Right, Size::flex(1.), Size::percent(100.)));

            rect()
                .position(Position::new_absolute())
                .layer(Layer::Overlay)
                .width(Size::percent(100.))
                .height(Size::percent(100.))
                .vertical()
                .content(Content::flex())
                .maybe_child(ghost)
                .child(edge(Side::Top, Size::percent(100.), Size::flex(1.)))
                .child(middle_row)
                .child(edge(Side::Bottom, Size::percent(100.), Size::flex(1.)))
                .into_element()
        });

        rect()
            .a11y_role(AccessibilityRole::Pane)
            .expanded()
            .child(tab_bar)
            .child(
                rect()
                    .expanded()
                    .overflow(Overflow::Clip)
                    .child(content)
                    .maybe_child(overlay),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

/// Drop zone for one edge.
fn drop_zone_for_side<M: DockingModel>(
    panel_id: M::PanelId,
    side: Side,
    mut controller: Writable<M>,
    hover: State<Option<HoverTarget<M::TabId>>>,
) -> Element {
    DropZone::<DockDrag<M::DropValue>>::new(
        rect().expanded().into_element(),
        move |payload: DockDrag<M::DropValue>| {
            controller
                .write()
                .on_drop(payload.value, DropTarget::Split { panel_id, side });
        },
    )
    .on_drag_over(move |hovering| toggle_hover(hover, HoverTarget::Edge(side), hovering))
    .into_element()
}

fn drag_preview(position: Position, width: Size, height: Size, preview: Element) -> Element {
    rect()
        .position(position)
        .interactive(false)
        .width(width)
        .height(height)
        .child(preview)
        .into_element()
}
