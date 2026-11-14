use std::{
    collections::HashMap,
    fmt::Display,
};

use torin::prelude::*;

#[derive(Default)]
struct TestingTree {
    mapper: HashMap<usize, (Option<usize>, Vec<usize>, u16, Node)>,
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
        if let BenchmarkMode::DistantBranches(branches) = self.mode {
            return self.prepare_distant_branches(branches);
        }

        let root_area = Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0));
        let mut measurer = Some(NoopMeasurer);
        let mut tree = TestingTree::default();
        tree.mapper.insert(
            0,
            (
                None,
                (0..self.wide - 1).map(|index| (index + 1) + 100).collect(),
                1,
                Node::from_size_and_direction(
                    Size::Percentage(Length::new(100.0)),
                    Size::Percentage(Length::new(100.0)),
                    Direction::Vertical,
                ),
            ),
        );

        let mut invalidate_node = 0;
        self.build_branch(&mut tree, 0, 0, &mut invalidate_node);

        let mut layout = Torin::<usize>::new();

        if self.mode == BenchmarkMode::InvalidatedCache {
            layout.find_best_root(&tree);
            layout.measure(0, root_area, &mut measurer, &tree);
            tree.mapper
                .get_mut(&invalidate_node)
                .expect("Node must exist")
                .3 = Node::from_size_and_direction(
                Size::Inner,
                Size::Pixels(Length::new(10.0)),
                Direction::Vertical,
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
    /// Runs root selection and layout without setup or cleanup.
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
    DistantBranches(usize),
}

impl Display for BenchmarkMode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCache => formatter.write_str("not cached"),
            Self::InvalidatedCache => formatter.write_str("cached"),
            Self::DistantBranches(branches) => {
                write!(formatter, "cached distant-branches={branches}")
            }
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

impl BenchmarkConfig {
    pub fn all() -> [Self; 14] {
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
            Self::distant_branches(1),
            Self::distant_branches(2),
            Self::distant_branches(8),
        ]
    }

    fn distant_branches(branches: usize) -> Self {
        Self {
            depth: 6,
            wide: 8,
            mode: BenchmarkMode::DistantBranches(branches),
            sample: 100,
            node_generator: simple_node_generator,
            prefix: String::default(),
        }
    }

    fn prepare_distant_branches(&self, branches: usize) -> BenchmarkInput {
        let mut tree = TestingTree::default();
        let size = self.size();
        let mut level_start = 0;
        let mut level_end = 1;

        // Assign consecutive IDs per level so every branch has unique nodes and accurate heights.
        for level in 0..self.depth {
            for node_id in level_start..level_end {
                let children_start = node_id * self.wide + 1;
                let children_end = (children_start + self.wide).min(size);
                let parent = if node_id == 0 {
                    None
                } else {
                    Some((node_id - 1) / self.wide)
                };
                tree.mapper.insert(
                    node_id,
                    (
                        parent,
                        (children_start..children_end).collect(),
                        level as u16,
                        (self.node_generator)(level),
                    ),
                );
            }
            level_start = level_end;
            level_end = (level_end * self.wide + 1).min(size);
        }

        let mut input = BenchmarkInput {
            tree,
            measurer: Some(NoopMeasurer),
            layout: Torin::new(),
            root_area: Rect::new(Point2D::new(0., 0.), Size2D::new(1000., 1000.)),
        };
        input.run();
        assert_eq!(input.layout.results.len(), size);

        // Spread mutations across root branches, including the first and last when changing several.
        for branch in 0..branches {
            let root_child = if branches == 1 {
                0
            } else {
                branch * (self.wide - 1) / (branches - 1)
            };
            let mut node_id = root_child + 1;
            for _ in 2..self.depth {
                node_id = node_id * self.wide + 1 + self.wide / 2;
            }
            let node = &mut input
                .tree
                .mapper
                .get_mut(&node_id)
                .expect("Leaf must exist")
                .3;
            *node =
                Node::from_size_and_direction(Size::px(40.), Size::px(35.), Direction::Vertical);
            input.layout.invalidate(node_id);
        }

        input
    }

    fn build_branch(
        &self,
        tree: &mut TestingTree,
        root: usize,
        level: usize,
        mid_node: &mut usize,
    ) -> Vec<usize> {
        if level == self.depth - 1 {
            return vec![];
        }

        let nodes = (0..self.wide)
            .map(|index| index + ((level + 1) * 100) + (root * 10))
            .collect::<Vec<usize>>();

        for (index, node_id) in nodes.iter().enumerate() {
            if level == self.depth / 2 && index == nodes.len() / 2 {
                *mid_node = *node_id;
            }

            let children = self.build_branch(tree, *node_id, level + 1, mid_node);
            tree.mapper.insert(
                *node_id,
                (
                    Some(root),
                    children,
                    level as u16,
                    (self.node_generator)(self.depth),
                ),
            );
        }

        nodes
    }
}
