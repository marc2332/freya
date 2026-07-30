#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashSet;

use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

/// A node of the example's own data. `Tree` never sees this: it is handed one visible row at a
/// time, which is what lets a real tree fetch a node's children only when it opens.
struct Node {
    name: &'static str,
    children: Vec<Node>,
}

fn node(name: &'static str, children: Vec<Node>) -> Node {
    Node { name, children }
}

fn data() -> Vec<Node> {
    vec![
        node(
            "src",
            vec![
                node(
                    "components",
                    vec![node("tree.rs", vec![]), node("table.rs", vec![])],
                ),
                node("main.rs", vec![]),
            ],
        ),
        node(
            "assets",
            vec![node("icon.png", vec![]), node("fonts", vec![])],
        ),
        node("Cargo.toml", vec![]),
    ]
}

/// One row on screen: what to draw, and the path that identifies it.
///
/// Its disclosure state is carried here rather than read inside the builder, because
/// `VirtualScrollView` memoizes that closure against its data: a flag read from state inside it
/// would go stale exactly as a captured snapshot would.
#[derive(Clone, PartialEq)]
struct Row {
    path: Vec<usize>,
    name: &'static str,
    depth: usize,
    disclosure: Disclosure,
}

/// Flatten the visible rows: a node's children are walked only when its path is expanded, which is
/// the same shape a lazily-loaded tree has.
fn rows(nodes: &[Node], expanded: &HashSet<Vec<usize>>, path: Vec<usize>, out: &mut Vec<Row>) {
    for (i, node) in nodes.iter().enumerate() {
        let mut child_path = path.clone();
        child_path.push(i);
        let open = expanded.contains(&child_path);
        out.push(Row {
            path: child_path.clone(),
            name: node.name,
            depth: path.len(),
            disclosure: if node.children.is_empty() {
                Disclosure::Leaf
            } else {
                Disclosure::from_expanded(open)
            },
        });
        if open {
            rows(node.children.as_slice(), expanded, child_path, out);
        }
    }
}

fn app() -> impl IntoElement {
    let mut expanded = use_state(HashSet::<Vec<usize>>::new);
    let mut selected = use_state(|| None::<Vec<usize>>);

    let tree = data();
    let mut visible = Vec::new();
    rows(&tree, &expanded.read(), Vec::new(), &mut visible);

    rect()
        .width(Size::fill())
        .height(Size::fill())
        .padding(Gaps::new_all(12.))
        .child(
            Tree::new_with_data(
                // Both inputs ride in the builder data, so a selection or an expansion rebuilds the
                // rows rather than leaving the memoized closure showing the old ones.
                (visible.clone(), selected.read().clone()),
                move |item: VirtualItem,
                      (visible, selected_path): &(Vec<Row>, Option<Vec<usize>>)| {
                    let Some(row) = visible.get(item.index).cloned() else {
                        return rect().into();
                    };
                    let toggle_path = row.path.clone();
                    let select_path = row.path.clone();
                    TreeItem::new()
                        .depth(row.depth)
                        .selected(selected_path.as_ref() == Some(&row.path))
                        .disclosure(row.disclosure)
                        .on_toggle(move |_| {
                            let mut set = expanded.write();
                            if !set.remove(&toggle_path) {
                                set.insert(toggle_path.clone());
                            }
                        })
                        .on_press(move |_| selected.set(Some(select_path.clone())))
                        .child(row.name)
                        .into()
                },
            )
            .length(visible.len()),
        )
}
