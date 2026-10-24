use std::{
    collections::HashMap,
    fmt::Display,
};

use torin::prelude::*;

#[derive(Default)]
struct TestingTree {
    mapper: HashMap<usize, (Option<usize>, Vec<usize>, u16, Node)>,
}

impl TestingTree {
    fn add(&mut self, node_id: usize, parent: Option<usize>, children: Vec<usize>, node: Node) {
        let depth = parent
            .map(|parent| self.mapper.get(&parent).expect("Parent must exist").2)
            .unwrap_or(0)
            + 1;
        self.mapper.insert(node_id, (parent, children, depth, node));
    }

    fn add_with_depth(
        &mut self,
        node_id: usize,
        parent: Option<usize>,
        children: Vec<usize>,
        node: Node,
        depth: u16,
    ) {
        self.mapper.insert(node_id, (parent, children, depth, node));
    }

    fn set_node(&mut self, node_id: usize, node: Node) {
        self.mapper.get_mut(&node_id).expect("Node must exist").3 = node;
    }
}

impl TreeAdapter<usize> for TestingTree {
    fn parent_of(&self, node_id: &usize) -> Option<usize> {
        self.mapper.get(node_id).and_then(|entry| entry.0)
    }

    fn height(&self, node_id: &usize) -> Option<u16> {
        self.mapper.get(node_id).map(|entry| entry.2)
    }

    fn read_node<R>(
        &self,
        node_id: &usize,
        reader: impl FnOnce(&Node, &[usize]) -> R,
    ) -> Option<R> {
        self.mapper
            .get(node_id)
            .map(|entry| reader(&entry.3, &entry.1))
    }

    fn root_id(&self) -> usize {
        0
    }
}

pub struct BenchmarkConfig {
    depth: usize,
    wide: usize,
    mode: BenchmarkMode,
    pub sample: usize,
    prefix: String,
    node_generator: fn(usize) -> Node,
}

impl BenchmarkConfig {
    pub fn name(&self) -> String {
        format!(
            "{}size={} depth={} wide={} mode={}",
            self.prefix,
            self.size(),
            self.depth,
            self.wide,
            self.mode
        )
    }

    pub fn size(&self) -> usize {
        let mut count = 1;
        let mut previous = 1;
        for _ in 0..self.depth - 1 {
            previous *= self.wide;
            count += previous;
        }
        count
    }

    pub fn prepare(&self) -> BenchmarkInput {
        let root_area = Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0));
        let mut measurer = Some(NoopMeasurer);
        let mut tree = TestingTree::default();
        tree.add(
            0,
            None,
            (0..self.wide - 1).map(|index| (index + 1) + 100).collect(),
            Node::from_size_and_direction(
                Size::Percentage(Length::new(100.0)),
                Size::Percentage(Length::new(100.0)),
                Direction::Vertical,
            ),
        );
        let mut invalidate_node = 0;
        build_branch(
            &mut tree,
            self.node_generator,
            0,
            0,
            self.depth,
            self.wide,
            &mut invalidate_node,
        );
        let mut layout = Torin::<usize>::new();
        if self.mode == BenchmarkMode::InvalidatedCache {
            layout.find_best_root(&tree);
            layout.measure(0, root_area, &mut measurer, &tree);
            tree.set_node(
                invalidate_node,
                Node::from_size_and_direction(
                    Size::Inner,
                    Size::Pixels(Length::new(10.0)),
                    Direction::Vertical,
                ),
            );
            layout.invalidate(invalidate_node);
        }
        BenchmarkInput {
            tree,
            measurer,
            layout,
            root_area,
        }
    }
}

pub struct BenchmarkInput {
    tree: TestingTree,
    measurer: Option<NoopMeasurer>,
    layout: Torin<usize>,
    root_area: Area,
}

impl BenchmarkInput {
    pub fn run(&mut self) {
        self.layout.find_best_root(&self.tree);
        self.layout
            .measure(0, self.root_area, &mut self.measurer, &self.tree);
    }
}

#[derive(PartialEq, Eq)]
enum BenchmarkMode {
    NoCache,
    InvalidatedCache,
}

impl Display for BenchmarkMode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCache => formatter.write_str("not cached"),
            Self::InvalidatedCache => formatter.write_str("cached"),
        }
    }
}

fn simple_node_generator(_depth: usize) -> Node {
    Node::from_size_and_direction(
        Size::Pixels(Length::new(100.0)),
        Size::Pixels(Length::new(100.0)),
        Direction::Vertical,
    )
}

pub fn benchmarks() -> [BenchmarkConfig; 11] {
    [
        BenchmarkConfig {
            depth: 2,
            wide: 1000,
            mode: BenchmarkMode::NoCache,
            sample: 500,
            node_generator: simple_node_generator,
            prefix: String::default(),
        },
        BenchmarkConfig {
            depth: 2,
            wide: 10000,
            mode: BenchmarkMode::NoCache,
            sample: 500,
            node_generator: simple_node_generator,
            prefix: String::default(),
        },
        BenchmarkConfig {
            depth: 2,
            wide: 100000,
            mode: BenchmarkMode::NoCache,
            sample: 500,
            node_generator: simple_node_generator,
            prefix: String::default(),
        },
        BenchmarkConfig {
            depth: 12,
            wide: 2,
            mode: BenchmarkMode::NoCache,
            sample: 500,
            node_generator: simple_node_generator,
            prefix: String::default(),
        },
        BenchmarkConfig {
            depth: 14,
            wide: 2,
            mode: BenchmarkMode::NoCache,
            sample: 100,
            node_generator: simple_node_generator,
            prefix: String::default(),
        },
        BenchmarkConfig {
            depth: 17,
            wide: 2,
            mode: BenchmarkMode::NoCache,
            sample: 100,
            node_generator: simple_node_generator,
            prefix: String::default(),
        },
        BenchmarkConfig {
            depth: 5,
            wide: 15,
            mode: BenchmarkMode::NoCache,
            sample: 500,
            node_generator: simple_node_generator,
            prefix: String::default(),
        },
        BenchmarkConfig {
            depth: 5,
            wide: 15,
            mode: BenchmarkMode::InvalidatedCache,
            sample: 500,
            node_generator: simple_node_generator,
            prefix: String::default(),
        },
        BenchmarkConfig {
            depth: 7,
            wide: 5,
            mode: BenchmarkMode::NoCache,
            sample: 100,
            node_generator: simple_node_generator,
            prefix: String::default(),
        },
        BenchmarkConfig {
            depth: 7,
            wide: 5,
            mode: BenchmarkMode::InvalidatedCache,
            sample: 100,
            node_generator: simple_node_generator,
            prefix: String::default(),
        },
        BenchmarkConfig {
            depth: 8,
            wide: 4,
            mode: BenchmarkMode::NoCache,
            sample: 70,
            node_generator: |depth| {
                if depth.is_multiple_of(2) {
                    Node::from_size_and_alignments_and_direction_and_padding(
                        Size::Pixels(Length::new(100.0)),
                        Size::Inner,
                        Alignment::Start,
                        Alignment::Center,
                        Direction::Vertical,
                        Gaps::default(),
                    )
                } else {
                    Node::from_size_and_alignments_and_direction_and_padding(
                        Size::Pixels(Length::new(100.0)),
                        Size::Pixels(Length::new(100.0)),
                        Alignment::Center,
                        Alignment::End,
                        Direction::Vertical,
                        Gaps::default(),
                    )
                }
            },
            prefix: "alignments=true ".to_string(),
        },
    ]
}

fn build_branch(
    tree: &mut TestingTree,
    node_generator: fn(usize) -> Node,
    root: usize,
    level: usize,
    depth: usize,
    width: usize,
    mid_node: &mut usize,
) -> Vec<usize> {
    if level == depth - 1 {
        return vec![];
    }
    let nodes = (0..width)
        .map(|index| index + ((level + 1) * 100) + (root * 10))
        .collect::<Vec<usize>>();
    for (index, node_id) in nodes.iter().enumerate() {
        if level == depth / 2 && index == nodes.len() / 2 {
            *mid_node = *node_id;
        }
        let children = build_branch(
            tree,
            node_generator,
            *node_id,
            level + 1,
            depth,
            width,
            mid_node,
        );
        tree.add_with_depth(
            *node_id,
            Some(root),
            children,
            node_generator(depth),
            level as u16,
        );
    }
    nodes
}
