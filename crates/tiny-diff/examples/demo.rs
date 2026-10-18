use tiny_diff::{
    Diff,
    DiffNode,
    diff,
};

struct Node {
    key: &'static str,
    text: &'static str,
    path: Vec<u32>,
    children: Vec<Node>,
}

impl DiffNode for Node {
    type Key = &'static str;
    type Modifications = &'static str;

    fn key(&self) -> &Self::Key {
        &self.key
    }

    fn path(&self) -> &[u32] {
        &self.path
    }

    fn children(&self) -> &[Self] {
        &self.children
    }

    fn compatible(&self, _: &Self) -> bool {
        true
    }

    fn modifications(&self, previous: &Self) -> Option<Self::Modifications> {
        (self.text != previous.text).then_some(self.text)
    }
}

fn list(items: &[(&'static str, &'static str)]) -> Node {
    Node {
        key: "list",
        text: "",
        path: Vec::new(),
        children: items
            .iter()
            .enumerate()
            .map(|(index, &(key, text))| Node {
                key,
                text,
                path: vec![index as u32],
                children: Vec::new(),
            })
            .collect(),
    }
}

fn main() {
    let previous = list(&[
        ("apple", "Apple"),
        ("banana", "Banana"),
        ("cherry", "Cherry"),
    ]);
    let current = list(&[
        ("banana", "Ripe banana"),
        ("orange", "Orange"),
        ("apple", "Apple"),
    ]);

    let mut changes = Diff::default();
    diff(&current, Some(&previous), &mut changes);

    println!("Added at new paths: {:?}", changes.added);
    println!("Removed at old paths: {:?}", changes.removed);
    println!("New text at new paths: {:?}", changes.modified);
    println!(
        "Old/new child indices by new parent path: {:?}",
        changes.moved
    );
}
