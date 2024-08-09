#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::{
    Path,
    PathBuf,
};

use freya::prelude::*;
use home::home_dir;

fn main() {
    launch_with_props(app, "File Explorer", (500.0, 500.0));
}

type TreeFileItem = TreeItem<PathBuf, ()>;
type FlatFileItem = FlatItem<PathBuf>;

pub async fn read_folder_as_items(dir: &Path) -> tokio::io::Result<Vec<TreeFileItem>> {
    let mut paths = tokio::fs::read_dir(dir).await?;
    let mut folder_items = Vec::default();
    let mut files_items = Vec::default();

    while let Ok(Some(entry)) = paths.next_entry().await {
        let file_type = entry.file_type().await?;
        let is_file = file_type.is_file();
        let path = entry.path();

        if is_file {
            files_items.push(TreeItem::Standalone {
                id: path,
                value: (),
            })
        } else {
            folder_items.push(TreeItem::Expandable {
                id: path,
                value: (),
                state: ExpandableItemState::Closed,
            })
        }
    }

    folder_items.extend(files_items);

    Ok(folder_items)
}

type State = (Vec<FlatFileItem>, Signal<Option<Vec<TreeFileItem>>>);

fn app() -> Element {
    let mut tree = use_signal(|| None);

    // Open the HOME dir
    use_effect(move || {
        spawn(async move {
            let home_path = home_dir().expect("Failed to get the Home dir.");
            let items = read_folder_as_items(&home_path).await.unwrap_or_default();
            tree.set(Some(items));
        });
    });

    // Flat the items
    let flat_items = {
        let tree = tree.read();
        let tree = tree.as_ref().unwrap_or_default();
        tree.iter()
            .flat_map(|tree| tree.flat(0, tree.id()))
            .collect::<Vec<FlatFileItem>>()
    };

    // Render the items
    rsx!(VirtualScrollView {
        length: flat_items.len(),
        item_size: 25.,
        builder_args: (flat_items, tree),
        builder: |index: usize, values: &Option<State>| {
            let (flat_items, mut tree) = values.as_ref().unwrap();
            let item = &flat_items[index];
            let margin = item.depth * 10;

            let onclick = {
                to_owned![item];
                move |_| {
                    to_owned![item];
                    spawn(async move {
                        let mut tree = tree.write();
                        let tree = tree.as_mut().unwrap();
                        let expandable_item = tree
                            .iter_mut()
                            .find(|tree_item| tree_item.id() == &item.root_id)
                            .unwrap();
                        if item.is_open {
                            expandable_item.set_state(&item.id, &ExpandableItemState::Closed);
                        } else {
                            let items = read_folder_as_items(&item.id).await.unwrap_or_default();
                            expandable_item.set_state(&item.id, &ExpandableItemState::Open(items));
                        }
                    });
                }
            };

            rsx!(
                label {
                    key: "{item.id:?}",
                    height: "25",
                    margin: "0 0 0 {margin}",
                    onclick,
                    max_lines: "1",
                    text_overflow: "ellipsis",
                    "{item.id:?}"
                }
            )
        }
    })
}
