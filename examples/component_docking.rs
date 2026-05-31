#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;

use freya::{
    prelude::*,
    radio::*,
};

type TabId = usize;
type PanelId = usize;

/// The first (left-most) panel under this node.
fn first_panel(node: &DockNode<TabId, PanelId>) -> Option<&DockPanel<TabId, PanelId>> {
    match node {
        DockNode::Panel(panel) => Some(panel),
        DockNode::Split { children, .. } => children.iter().find_map(first_panel),
    }
}

/// Mutable version of [`first_panel`].
fn first_panel_mut(node: &mut DockNode<TabId, PanelId>) -> Option<&mut DockPanel<TabId, PanelId>> {
    match node {
        DockNode::Panel(panel) => Some(panel),
        DockNode::Split { children, .. } => children.iter_mut().find_map(first_panel_mut),
    }
}

/// Place `tab_id` into `panel_id` at `position`, or append it when `None`, and
/// remove it from any other panel. Returns `false` if the panel doesn't exist.
fn place_tab(
    tree: &mut DockNode<TabId, PanelId>,
    panel_id: PanelId,
    tab_id: TabId,
    position: Option<usize>,
) -> bool {
    let Some(panel) = tree.panel_mut(&panel_id) else {
        return false;
    };
    match position {
        Some(position) => panel.insert_tab(tab_id, position),
        None => panel.append_tab(tab_id),
    }
    tree.remove_tab_except(&tab_id, Some(&panel_id));
    true
}

#[derive(Default, Clone)]
struct Workspace {
    tree: Option<DockNode<TabId, PanelId>>,
    next_panel_id: PanelId,
    next_tab_id: TabId,
    tab_titles: HashMap<TabId, String>,
}

impl Workspace {
    fn new() -> Self {
        let layout = DockNode::Split {
            direction: Direction::Horizontal,
            children: vec![
                DockNode::Panel(DockPanel::new(0, vec![1, 2])),
                DockNode::Panel(DockPanel::new(1, vec![3])),
            ],
        };
        Self {
            next_panel_id: 2,
            tree: Some(layout),
            next_tab_id: 4,
            tab_titles: HashMap::from([
                (1, "Welcome".into()),
                (2, "README.md".into()),
                (3, "src/main.rs".into()),
            ]),
        }
    }

    fn open_new_tab(&mut self) {
        let tab_id = self.next_tab_id;
        self.next_tab_id += 1;
        self.tab_titles.insert(tab_id, format!("Untitled {tab_id}"));

        match self.tree.as_mut().and_then(first_panel_mut) {
            Some(panel) => {
                panel.tabs.push(tab_id);
                panel.active_tab_id = Some(tab_id);
            }
            None => {
                let panel_id = self.next_panel_id;
                self.next_panel_id += 1;
                self.tree = Some(DockNode::Panel(DockPanel::new(panel_id, vec![tab_id])));
            }
        }
    }

    fn close_active(&mut self) {
        let Some(tree) = self.tree.as_mut() else {
            return;
        };
        let Some(tab_id) = first_panel(tree).and_then(|panel| panel.active_tab_id) else {
            return;
        };
        tree.remove_tab_except(&tab_id, None);
        self.tab_titles.remove(&tab_id);
        self.collapse_empty();
    }

    fn title(&self, tab_id: TabId) -> String {
        self.tab_titles
            .get(&tab_id)
            .cloned()
            .unwrap_or_else(|| format!("Tab {tab_id}"))
    }

    fn is_active(&self, tab_id: TabId) -> bool {
        let Some(tree) = self.tree.as_ref() else {
            return false;
        };
        let Some((panel_id, _)) = tree.find_tab(&tab_id) else {
            return false;
        };
        tree.panel(&panel_id).and_then(|panel| panel.active_tab_id) == Some(tab_id)
    }

    fn collapse_empty(&mut self) {
        if let Some(tree) = self.tree.as_mut() {
            tree.close_empty_panels();
            if tree.is_empty() {
                self.tree = None;
            }
        }
    }
}

impl DockingModel for Workspace {
    type TabId = TabId;
    type PanelId = PanelId;
    type DropValue = TabId;

    fn root(&self) -> Option<&DockNode<TabId, PanelId>> {
        self.tree.as_ref()
    }

    fn on_drop(&mut self, tab_id: TabId, target: DropTarget<PanelId>) -> bool {
        let Some(tree) = self.tree.as_mut() else {
            return false;
        };

        let success = match target {
            DropTarget::Tab { panel_id, position } => {
                place_tab(tree, panel_id, tab_id, Some(position))
            }
            DropTarget::Center(panel_id) => place_tab(tree, panel_id, tab_id, None),
            DropTarget::Split { panel_id, side } => {
                let new_panel_id = self.next_panel_id;
                let new_panel = DockPanel::new(new_panel_id, vec![tab_id]);
                if !tree.split_panel(&panel_id, side, &new_panel) {
                    return false;
                }
                tree.remove_tab_except(&tab_id, Some(&new_panel_id));
                self.next_panel_id += 1;
                true
            }
        };

        self.collapse_empty();
        success
    }

    fn set_active(&mut self, panel_id: PanelId, tab_id: TabId) -> bool {
        let Some(target) = self
            .tree
            .as_mut()
            .and_then(|tree| tree.panel_mut(&panel_id))
        else {
            return false;
        };
        if !target.tabs.contains(&tab_id) {
            return false;
        }
        target.active_tab_id = Some(tab_id);
        true
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy, Hash)]
enum AppChannel {
    Workspace,
}

impl RadioChannel<Workspace> for AppChannel {}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(1200., 900.)))
}

fn app() -> impl IntoElement {
    use_init_theme(dark_theme);
    use_init_radio_station::<Workspace, AppChannel>(Workspace::new);

    let mut radio = use_radio::<Workspace, AppChannel>(AppChannel::Workspace);
    let workspace = radio.slice_mut_current(|state| state).into_writable();

    rect()
        .expanded()
        .background((20, 20, 20))
        .color(Color::WHITE)
        .child(
            rect()
                .horizontal()
                .width(Size::fill())
                .padding(8.)
                .spacing(8.)
                .background((30, 30, 30))
                .cross_align(Alignment::Center)
                .child("Docking")
                .child(
                    Button::new()
                        .on_press(move |_| radio.write().open_new_tab())
                        .child("New tab"),
                )
                .child(
                    Button::new()
                        .on_press(move |_| radio.write().close_active())
                        .child("Close active tab"),
                ),
        )
        .child(
            rect().expanded().child(
                DockingArea::new(
                    workspace,
                    move |ctx: ContentContext<TabId, PanelId>| {
                        let Some(tab_id) = ctx.tab_id else {
                            return rect()
                                .expanded()
                                .center()
                                .background((25, 25, 25))
                                .color((150, 150, 150))
                                .child("This panel has no open tabs.")
                                .into_element();
                        };
                        let title = radio.read().title(tab_id);
                        rect()
                            .expanded()
                            .padding(Gaps::new_all(16.))
                            .background((25, 25, 25))
                            .child(label().font_size(20.).text(format!("Editing: {title}")))
                            .child("Drag a tab header onto another panel or onto a panel edge to split it.")
                            .into_element()
                    },
                    move |ctx: TabContext<TabId>| {
                        let workspace = radio.read();
                        let title = workspace.title(ctx.tab_id);
                        let is_active = workspace.is_active(ctx.tab_id);
                        Activable::new(FloatingTab::new().child(title)).active(is_active).into_element()
                    },
                    move |tab_id| {
                        let title = radio.read().title(tab_id);
                        rect()
                            .interactive(false)
                            .child(Activable::new(FloatingTab::new().child(label().text(title).max_lines(1))).active(true))
                            .into_element()

                    },
                    |ctx: TabBarContext<PanelId>| {
                        ScrollView::new()
                            .direction(Direction::Horizontal)
                            .height(Size::px(48.))
                            .show_scrollbar(false)
                            .child(rect().padding(4.).spacing(4.).horizontal().cross_align(Alignment::Center).children(ctx.tab_children))
                            .into_element()
                    },
                )
                .preview_element(
                    rect()
                        .interactive(false)
                        .expanded()
                        .background((255, 255, 255, 0.10)),
                ),
            ),
        )
}
