use std::hint::black_box;

use rustc_hash::FxHashMap;
use torin::prelude::*;

const ITERATIONS: usize = 3;

#[derive(Default)]
struct TestingTree {
    mapper: FxHashMap<usize, (Option<usize>, Vec<usize>, u16, Node)>,
}

impl TestingTree {
    fn add(&mut self, node_id: usize, parent: Option<usize>, children: Vec<usize>, node: Node) {
        let depth = parent.map(|parent| self.mapper[&parent].2).unwrap_or(0) + 1;
        self.add_with_depth(node_id, parent, children, node, depth);
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
        self.mapper.get(node_id).map(|entry| reader(&entry.3, &entry.1))
    }

    fn root_id(&self) -> usize {
        0
    }
}

#[derive(PartialEq, Eq)]
enum BenchmarkMode {
    NoCache,
    InvalidatedCache,
}

#[hotpath::main(functions_limit = 0)]
fn main() {
    flat_1000();
    flat_10000();
    flat_100000();
    binary_depth_12();
    binary_depth_14();
    binary_depth_17();
    wide_depth_5();
    wide_depth_5_invalidated();
    wide_depth_7();
    wide_depth_7_invalidated();
    aligned_depth_8();
}

#[hotpath::measure]
fn flat_1000() {
    benchmark(2, 1000, BenchmarkMode::NoCache, simple_node);
}

#[hotpath::measure]
fn flat_10000() {
    benchmark(2, 10000, BenchmarkMode::NoCache, simple_node);
}

#[hotpath::measure]
fn flat_100000() {
    benchmark(2, 100000, BenchmarkMode::NoCache, simple_node);
}

#[hotpath::measure]
fn binary_depth_12() {
    benchmark(12, 2, BenchmarkMode::NoCache, simple_node);
}

#[hotpath::measure]
fn binary_depth_14() {
    benchmark(14, 2, BenchmarkMode::NoCache, simple_node);
}

#[hotpath::measure]
fn binary_depth_17() {
    benchmark(17, 2, BenchmarkMode::NoCache, simple_node);
}

#[hotpath::measure]
fn wide_depth_5() {
    benchmark(5, 15, BenchmarkMode::NoCache, simple_node);
}

#[hotpath::measure]
fn wide_depth_5_invalidated() {
    benchmark(5, 15, BenchmarkMode::InvalidatedCache, simple_node);
}

#[hotpath::measure]
fn wide_depth_7() {
    benchmark(7, 5, BenchmarkMode::NoCache, simple_node);
}

#[hotpath::measure]
fn wide_depth_7_invalidated() {
    benchmark(7, 5, BenchmarkMode::InvalidatedCache, simple_node);
}

#[hotpath::measure]
fn aligned_depth_8() {
    benchmark(8, 4, BenchmarkMode::NoCache, aligned_node);
}

fn simple_node(_depth: usize) -> Node {
    Node::from_size_and_direction(
        Size::Pixels(Length::new(100.0)),
        Size::Pixels(Length::new(100.0)),
        Direction::Vertical,
    )
}

fn aligned_node(depth: usize) -> Node {
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
}

fn benchmark(
    depth: usize,
    width: usize,
    mode: BenchmarkMode,
    node_generator: fn(usize) -> Node,
) {
    for _ in 0..ITERATIONS {
        let root_area = Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0));
        let mut measurer = Some(NoopMeasurer);
        let mut tree = TestingTree::default();
        tree.add(
            0,
            None,
            (0..width - 1).map(|index| (index + 1) + 100).collect(),
            Node::from_size_and_direction(
                Size::Percentage(Length::new(100.0)),
                Size::Percentage(Length::new(100.0)),
                Direction::Vertical,
            ),
        );
        let mut invalidate_node = 0;
        build_branch(
            &mut tree,
            node_generator,
            0,
            0,
            depth,
            width,
            &mut invalidate_node,
        );
        let mut layout = Torin::<usize>::new();
        if mode == BenchmarkMode::InvalidatedCache {
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
        measure_layout(&mut layout, root_area, &mut measurer, &tree);
    }
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

#[hotpath::measure]
fn measure_layout(
    layout: &mut Torin<usize>,
    root_area: Area,
    measurer: &mut Option<NoopMeasurer>,
    tree: &TestingTree,
) {
    layout.find_best_root(tree);
    layout.measure(0, root_area, measurer, tree);
    black_box(&*layout);
}
