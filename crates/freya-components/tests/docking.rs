use freya::prelude::*;

type TabId = usize;
type PanelId = usize;
type Node = DockNode<TabId, PanelId>;
type Panel = DockPanel<TabId, PanelId>;
type Target = DropTarget<PanelId>;

#[derive(Default, Clone, PartialEq, Debug)]
struct DockingState {
    root: Option<Node>,
    next_panel_id: PanelId,
}

impl DockingState {
    fn new() -> Self {
        Self::default()
    }

    fn with_layout(layout: Node) -> Self {
        fn max_panel_id(node: &Node) -> Option<PanelId> {
            match node {
                DockNode::Panel(panel) => Some(panel.panel_id),
                DockNode::Split { children, .. } => children.iter().filter_map(max_panel_id).max(),
            }
        }
        let next_panel_id = max_panel_id(&layout).map_or(0, |max| max + 1);
        Self {
            root: Some(layout),
            next_panel_id,
        }
    }

    fn reserve_panel_id(&mut self) -> PanelId {
        let id = self.next_panel_id;
        self.next_panel_id += 1;
        id
    }

    fn find_tab(&self, tab: TabId) -> Option<(PanelId, usize)> {
        self.root.as_ref().and_then(|root| root.find_tab(&tab))
    }

    fn set_active(&mut self, panel: PanelId, tab: TabId) -> bool {
        let Some(target) = self.root.as_mut().and_then(|root| root.panel_mut(&panel)) else {
            return false;
        };
        if !target.tabs.contains(&tab) {
            return false;
        }
        target.active = Some(tab);
        true
    }

    fn close_tab(&mut self, tab: TabId) -> bool {
        let Some(root) = self.root.as_mut() else {
            return false;
        };
        let removed = root.remove_tab_except(&tab, None);
        self.compact();
        removed
    }

    fn move_tab(&mut self, tab: TabId, target: Target) -> bool {
        let Some(root) = self.root.as_mut() else {
            return false;
        };
        let success = match target {
            DropTarget::Tab {
                panel: target_panel,
                position,
            } => insert_at(root, target_panel, tab, Some(position)),
            DropTarget::Center(target_panel) => insert_at(root, target_panel, tab, None),
            DropTarget::Split {
                panel: target_panel,
                side,
            } => {
                let new_panel_id = self.next_panel_id;
                let new_panel = Panel::new(new_panel_id, vec![tab]);
                if !root.split_panel(&target_panel, side, &new_panel) {
                    return false;
                }
                root.remove_tab_except(&tab, Some(&new_panel_id));
                self.next_panel_id += 1;
                true
            }
        };
        self.compact();
        success
    }

    fn compact(&mut self) {
        let Some(root) = self.root.as_mut() else {
            return;
        };
        root.close_empty_panels();
        if root.is_empty() {
            self.root = None;
        }
    }
}

fn insert_at(root: &mut Node, target_panel: PanelId, tab: TabId, position: Option<usize>) -> bool {
    let Some(panel) = root.panel_mut(&target_panel) else {
        return false;
    };
    match position {
        Some(at) => panel.insert_tab(tab, at),
        None => panel.append_tab(tab),
    }
    root.remove_tab_except(&tab, Some(&target_panel));
    true
}

fn make_panel(state: &mut DockingState, tabs: Vec<TabId>) -> PanelId {
    let panel_id = state.reserve_panel_id();
    let panel = Panel::new(panel_id, tabs);
    state.root = Some(match state.root.take() {
        None => DockNode::Panel(panel),
        Some(existing) => DockNode::Split {
            direction: Direction::Horizontal,
            children: vec![existing, DockNode::Panel(panel)],
        },
    });
    panel_id
}

#[test]
fn find_tab_in_simple_panel() {
    let mut state = DockingState::new();
    let panel = make_panel(&mut state, vec![10, 11, 12]);
    assert_eq!(state.find_tab(11), Some((panel, 1)));
    assert_eq!(state.find_tab(99), None);
}

#[test]
fn move_tab_between_panels() {
    let mut state = DockingState::new();
    let source_panel = make_panel(&mut state, vec![1, 2, 3]);
    let target_panel = make_panel(&mut state, vec![4]);

    assert!(state.move_tab(
        2,
        DropTarget::Tab {
            panel: target_panel,
            position: 0,
        },
    ));

    assert_eq!(state.find_tab(2), Some((target_panel, 0)));
    let (origin, _) = state.find_tab(1).unwrap();
    assert_eq!(origin, source_panel);
    if let Some(DockNode::Split { children, .. }) = &state.root {
        for child in children {
            if let DockNode::Panel(panel) = child
                && panel.panel_id == target_panel
            {
                assert_eq!(panel.tabs, vec![2, 4]);
                assert_eq!(panel.active, Some(2));
            }
        }
    }
}

#[test]
fn moving_last_tab_collapses_panel() {
    let mut state = DockingState::new();
    let _source = make_panel(&mut state, vec![1]);
    let target = make_panel(&mut state, vec![2]);

    assert!(state.move_tab(1, DropTarget::Center(target)));

    match state.root.as_ref().unwrap() {
        DockNode::Panel(panel) => {
            assert_eq!(panel.panel_id, target);
            assert_eq!(panel.tabs, vec![2, 1]);
        }
        other => panic!("expected panel, got {other:?}"),
    }
}

#[test]
fn reorder_within_same_panel() {
    let mut state = DockingState::new();
    let panel = make_panel(&mut state, vec![1, 2, 3, 4]);

    assert!(state.move_tab(1, DropTarget::Tab { panel, position: 3 },));

    if let Some(DockNode::Panel(panel)) = &state.root {
        assert_eq!(panel.tabs, vec![2, 3, 1, 4]);
    } else {
        panic!("expected single panel root");
    }
}

#[test]
fn split_creates_new_panel() {
    let mut state = DockingState::new();
    let panel = make_panel(&mut state, vec![1, 2]);

    assert!(state.move_tab(
        1,
        DropTarget::Split {
            panel,
            side: Side::Right,
        },
    ));

    match state.root.as_ref().unwrap() {
        DockNode::Split {
            direction,
            children,
        } => {
            assert_eq!(*direction, Direction::Horizontal);
            assert_eq!(children.len(), 2);
            let DockNode::Panel(left) = &children[0] else {
                panic!("expected panel");
            };
            let DockNode::Panel(right) = &children[1] else {
                panic!("expected panel");
            };
            assert_eq!(left.panel_id, panel);
            assert_eq!(left.tabs, vec![2]);
            assert_eq!(right.tabs, vec![1]);
            assert_eq!(right.active, Some(1));
        }
        other => panic!("expected split, got {other:?}"),
    }
}

#[test]
fn split_left_places_new_panel_first() {
    let mut state = DockingState::new();
    let panel = make_panel(&mut state, vec![1, 2]);

    assert!(state.move_tab(
        2,
        DropTarget::Split {
            panel,
            side: Side::Left,
        },
    ));

    if let Some(DockNode::Split { children, .. }) = &state.root {
        let DockNode::Panel(first) = &children[0] else {
            panic!("expected panel");
        };
        assert_eq!(first.tabs, vec![2]);
    } else {
        panic!("expected split");
    }
}

#[test]
fn split_top_uses_vertical_direction() {
    let mut state = DockingState::new();
    let panel = make_panel(&mut state, vec![1, 2]);

    state.move_tab(
        2,
        DropTarget::Split {
            panel,
            side: Side::Top,
        },
    );

    if let Some(DockNode::Split { direction, .. }) = &state.root {
        assert_eq!(*direction, Direction::Vertical);
    } else {
        panic!("expected split");
    }
}

#[test]
fn close_tab_removes_and_compacts() {
    let mut state = DockingState::new();
    let removed = make_panel(&mut state, vec![1]);
    let _kept = make_panel(&mut state, vec![2, 3]);

    state.close_tab(1);

    match state.root.as_ref().unwrap() {
        DockNode::Panel(panel) => {
            assert_ne!(panel.panel_id, removed);
            assert_eq!(panel.tabs, vec![2, 3]);
        }
        other => panic!("expected single panel, got {other:?}"),
    }
}

#[test]
fn closing_active_tab_promotes_first() {
    let mut state = DockingState::new();
    let panel = make_panel(&mut state, vec![1, 2, 3]);
    state.set_active(panel, 2);
    state.close_tab(2);

    if let Some(DockNode::Panel(panel)) = &state.root {
        assert_eq!(panel.tabs, vec![1, 3]);
        assert_eq!(panel.active, Some(1));
    }
}

#[test]
fn with_layout_advances_id_counter() {
    let mut state = DockingState::with_layout(DockNode::Split {
        direction: Direction::Horizontal,
        children: vec![
            DockNode::Panel(Panel::new(0, vec![10])),
            DockNode::Panel(Panel::new(5, vec![20])),
        ],
    });
    assert_eq!(state.reserve_panel_id(), 6);
}

#[test]
fn split_self_then_drop_keeps_tab_in_new_panel() {
    let mut state = DockingState::new();
    let panel = make_panel(&mut state, vec![1, 2]);

    state.move_tab(
        1,
        DropTarget::Split {
            panel,
            side: Side::Right,
        },
    );

    if let Some(DockNode::Split { children, .. }) = &state.root {
        let DockNode::Panel(left) = &children[0] else {
            panic!()
        };
        let DockNode::Panel(right) = &children[1] else {
            panic!()
        };
        assert_eq!(left.tabs, vec![2]);
        assert_eq!(right.tabs, vec![1]);
    } else {
        panic!("expected split");
    }
}
