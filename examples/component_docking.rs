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

        let placed = self
            .tree
            .as_mut()
            .and_then(DockNode::first_panel_mut)
            .map(|panel| {
                panel.tabs.push(tab_id);
                panel.active_tab_id = Some(tab_id);
            })
            .is_some();

        if !placed {
            let panel_id = self.next_panel_id;
            self.next_panel_id += 1;
            self.tree = Some(DockNode::Panel(DockPanel::new(panel_id, vec![tab_id])));
        }
    }

    fn close_active(&mut self) {
        let Some(tab) = self
            .tree
            .as_ref()
            .and_then(DockNode::first_panel)
            .and_then(|panel| panel.active_tab_id)
        else {
            return;
        };

        if let Some(tree) = self.tree.as_mut() {
            tree.remove_tab_except(&tab, None);
        }
        self.collapse_empty();
        self.tab_titles.remove(&tab);
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

    fn root(&self) -> Option<&DockNode<TabId, PanelId>> {
        self.tree.as_ref()
    }

    fn move_tab(&mut self, tab: TabId, target: DropTarget<PanelId>) -> bool {
        let Some(tree) = self.tree.as_mut() else {
            return false;
        };

        let success = match target {
            DropTarget::Tab { panel, position } => {
                let Some(target) = tree.panel_mut(&panel) else {
                    return false;
                };
                target.insert_tab(tab, position);
                tree.remove_tab_except(&tab, Some(&panel));
                true
            }
            DropTarget::Center(panel) => {
                let Some(target) = tree.panel_mut(&panel) else {
                    return false;
                };
                target.append_tab(tab);
                tree.remove_tab_except(&tab, Some(&panel));
                true
            }
            DropTarget::Split { panel, side } => {
                let new_panel_id = self.next_panel_id;
                let new_panel = DockPanel::new(new_panel_id, vec![tab]);
                if !tree.split_panel(&panel, side, &new_panel) {
                    return false;
                }
                tree.remove_tab_except(&tab, Some(&new_panel_id));
                self.next_panel_id += 1;
                true
            }
        };

        self.collapse_empty();
        success
    }

    fn set_active(&mut self, panel: PanelId, tab: TabId) -> bool {
        let Some(target) = self.tree.as_mut().and_then(|tree| tree.panel_mut(&panel)) else {
            return false;
        };
        if !target.tabs.contains(&tab) {
            return false;
        }
        target.active_tab_id = Some(tab);
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
