use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustc_hash::FxHashSet;
use torin::prelude::*;

struct TestingMeasurer;

impl LayoutMeasurer<usize> for TestingMeasurer {
    fn measure(
        &mut self,
        _node_id: usize,
        _node: &Node,
        _area: &Area,
        _parent_size: &Area,
        _available_parent_area: &Area,
    ) -> Option<Area> {
        None
    }
}

#[derive(Default)]
struct TestingDOM {
    mapper: HashMap<usize, (Option<usize>, Vec<usize>, u16, Node)>,
}

impl TestingDOM {
    fn add(&mut self, node_id: usize, parent: Option<usize>, children: Vec<usize>, node: Node) {
        let depth = parent.map(|p| self.mapper.get(&p).unwrap().2).unwrap_or(0) + 1;
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
        self.mapper.get_mut(&node_id).unwrap().3 = node;
    }
}

impl DOMAdapter<usize> for TestingDOM {
    fn children_of(&mut self, node_id: &usize) -> Vec<usize> {
        self.mapper
            .get(node_id)
            .map(|c| c.1.clone())
            .unwrap_or_default()
    }

    fn parent_of(&self, node_id: &usize) -> Option<usize> {
        self.mapper.get(node_id).map(|c| c.0).flatten()
    }

    fn height(&self, node_id: &usize) -> Option<u16> {
        self.mapper.get(node_id).map(|c| c.2)
    }

    fn get_node(&self, node_id: &usize) -> Option<Node> {
        self.mapper.get(node_id).map(|c| c.3.clone())
    }

    fn is_node_valid(&mut self, _node_id: &usize) -> bool {
        true
    }

    fn closest_common_parent(
        &self,
        node_id_a: &usize,
        node_id_b: &usize,
        root_track_patch: &mut FxHashSet<usize>,
    ) -> Option<usize> {
        root_track_patch.insert(*node_id_a);
        root_track_patch.insert(*node_id_b);
        self.parent_of(node_id_a)
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("benchmarks");
    g.significance_level(0.1).sample_size(500);

    let params = [
        ("big trees (wide) nodes=1000, depth=1", 1000, 1),
        ("big trees (wide) nodes=10000, depth=1", 10000, 1),
        ("big trees (wide) nodes=100000, depth=1", 100000, 1),
        ("big trees (deep) nodes=4000, depth=12", 4000, 12),
        ("big trees (deep) nodes=10000, depth=14", 10000, 14),
        ("big trees (deep) nodes=100000, depth=17", 100000, 17),
    ];

    for (name, size, depth) in params {
        let size_per_layer = size / depth;

        g.bench_function(name, |b| {
            let mut measurer = Some(TestingMeasurer);
            let mut mocked_dom = TestingDOM::default();

            let children_ids = (1..=size_per_layer).into_iter().collect::<Vec<usize>>();

            let mut root = 0;

            mocked_dom.add(
                0,
                None,
                children_ids.clone(),
                Node::from_size_and_direction(
                    Size::Percentage(Length::new(100.0)),
                    Size::Percentage(Length::new(100.0)),
                    DirectionMode::Vertical,
                ),
            );

            for level in 0..depth {
                for i in &children_ids {
                    let id = (level * size) + *i;
                    let children = if level == depth - 1 {
                        vec![]
                    } else if *i == size_per_layer - 1 {
                        (1..101)
                            .map(move |i| i + ((level + 1) * size))
                            .collect::<Vec<usize>>()
                    } else {
                        vec![]
                    };

                    mocked_dom.add(
                        id,
                        Some(root),
                        children,
                        Node::from_size_and_direction(
                            Size::Pixels(Length::new(100.0)),
                            Size::Pixels(Length::new(100.0)),
                            DirectionMode::Vertical,
                        ),
                    );

                    if *i == size_per_layer - 1 {
                        root = id
                    }
                }
            }

            b.iter(|| {
                black_box({
                    let mut layout = Torin::<usize>::new();
                    layout.find_best_root(&mut mocked_dom);
                    layout.measure(
                        0,
                        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
                        &mut measurer,
                        &mut mocked_dom,
                    )
                });
            })
        });
    }

    g.bench_function(
        "big trees (deep + cached) + invalidated node in the top",
        |b| {
            let mut layout = Torin::<usize>::new();
            let mut measurer = Some(TestingMeasurer);
            let mut mocked_dom = TestingDOM::default();

            let children_ids = (1..=101).into_iter().collect::<Vec<usize>>();

            let mut root = 0;

            mocked_dom.add(
                0,
                None,
                children_ids.clone(),
                Node::from_size_and_direction(
                    Size::Percentage(Length::new(100.0)),
                    Size::Percentage(Length::new(100.0)),
                    DirectionMode::Vertical,
                ),
            );

            let levels = 20;

            for level in 0..levels {
                for i in &children_ids {
                    let id = (level * 1000) + *i;
                    let children = if *i == 101 && level < levels - 1 {
                        (1..101)
                            .map(move |i| i + ((level + 1) * 1000))
                            .collect::<Vec<usize>>()
                    } else {
                        vec![]
                    };

                    mocked_dom.add(
                        id,
                        Some(root),
                        children,
                        Node::from_size_and_direction(
                            Size::Pixels(Length::new(100.0)),
                            Size::Pixels(Length::new(100.0)),
                            DirectionMode::Vertical,
                        ),
                    );

                    if *i == 101 {
                        root = id
                    }
                }
            }

            layout.find_best_root(&mut mocked_dom);
            layout.measure(
                0,
                Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
                &mut measurer,
                &mut mocked_dom,
            );

            b.iter(|| {
                black_box({
                    mocked_dom.set_node(
                        1,
                        Node::from_size_and_direction(
                            Size::Inner,
                            Size::Pixels(Length::new(10.0)),
                            DirectionMode::Vertical,
                        ),
                    );
                    layout.invalidate(1);
                    layout.find_best_root(&mut mocked_dom);
                    layout.measure(
                        0,
                        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
                        &mut measurer,
                        &mut mocked_dom,
                    )
                });
            })
        },
    );

    g.bench_function(
        "big trees (deep + cached) + invalidated node in the bottom",
        |b| {
            let mut layout = Torin::<usize>::new();
            let mut measurer = Some(TestingMeasurer);
            let mut mocked_dom = TestingDOM::default();

            let children_ids = (1..=101).into_iter().collect::<Vec<usize>>();

            let mut root = 0;

            mocked_dom.add(
                0,
                None,
                children_ids.clone(),
                Node::from_size_and_direction(
                    Size::Percentage(Length::new(100.0)),
                    Size::Percentage(Length::new(100.0)),
                    DirectionMode::Vertical,
                ),
            );

            let levels = 20;

            for level in 0..levels {
                for i in &children_ids {
                    let id = (level * 1000) + *i;
                    let children = if *i == 101 && level < levels - 1 {
                        (1..101)
                            .map(move |i| i + ((level + 1) * 1000))
                            .collect::<Vec<usize>>()
                    } else {
                        vec![]
                    };

                    mocked_dom.add(
                        id,
                        Some(root),
                        children,
                        Node::from_size_and_direction(
                            Size::Pixels(Length::new(100.0)),
                            Size::Pixels(Length::new(100.0)),
                            DirectionMode::Vertical,
                        ),
                    );

                    if *i == 101 {
                        root = id
                    }
                }
            }

            layout.find_best_root(&mut mocked_dom);
            layout.measure(
                0,
                Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
                &mut measurer,
                &mut mocked_dom,
            );

            b.iter(|| {
                black_box({
                    mocked_dom.set_node(
                        1,
                        Node::from_size_and_direction(
                            Size::Inner,
                            Size::Pixels(Length::new(10.0)),
                            DirectionMode::Vertical,
                        ),
                    );
                    layout.invalidate(2001);
                    layout.find_best_root(&mut mocked_dom);
                    layout.measure(
                        0,
                        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
                        &mut measurer,
                        &mut mocked_dom,
                    )
                });
            })
        },
    );

    g.bench_function(
        "big trees (deep + cached) + invalidated node in the middle",
        |b| {
            let mut layout = Torin::<usize>::new();
            let mut measurer = Some(TestingMeasurer);
            let mut mocked_dom = TestingDOM::default();

            let children_ids = (1..=101).into_iter().collect::<Vec<usize>>();

            let mut root = 0;

            mocked_dom.add(
                0,
                None,
                children_ids.clone(),
                Node::from_size_and_direction(
                    Size::Percentage(Length::new(100.0)),
                    Size::Percentage(Length::new(100.0)),
                    DirectionMode::Vertical,
                ),
            );

            let levels = 20;

            for level in 0..levels {
                for i in &children_ids {
                    let id = (level * 1000) + *i;
                    let children = if *i == 101 && level < levels - 1 {
                        (1..101)
                            .map(move |i| i + ((level + 1) * 1000))
                            .collect::<Vec<usize>>()
                    } else {
                        vec![]
                    };

                    mocked_dom.add(
                        id,
                        Some(root),
                        children,
                        Node::from_size_and_direction(
                            Size::Pixels(Length::new(100.0)),
                            Size::Pixels(Length::new(100.0)),
                            DirectionMode::Vertical,
                        ),
                    );

                    if *i == 101 {
                        root = id
                    }
                }
            }

            layout.find_best_root(&mut mocked_dom);
            layout.measure(
                0,
                Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
                &mut measurer,
                &mut mocked_dom,
            );

            b.iter(|| {
                black_box({
                    mocked_dom.set_node(
                        1,
                        Node::from_size_and_direction(
                            Size::Inner,
                            Size::Pixels(Length::new(10.0)),
                            DirectionMode::Vertical,
                        ),
                    );
                    layout.invalidate(1001);
                    layout.find_best_root(&mut mocked_dom);
                    layout.measure(
                        0,
                        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
                        &mut measurer,
                        &mut mocked_dom,
                    )
                });
            })
        },
    );

    g.bench_function(
        "big trees (deep + branches + cached) + invalidated node in the middle",
        |b| {
            let mut layout = Torin::<usize>::new();
            let mut measurer = Some(TestingMeasurer);
            let mut mocked_dom = TestingDOM::default();

            mocked_dom.add(
                0,
                None,
                vec![1, 2, 3],
                Node::from_size_and_direction(
                    Size::Percentage(Length::new(100.0)),
                    Size::Percentage(Length::new(100.0)),
                    DirectionMode::Vertical,
                ),
            );

            const LEVELS: usize = 20;

            fn build_branch(mocked_dom: &mut TestingDOM, root: usize, level: usize) -> Vec<usize> {
                if level == LEVELS {
                    return vec![];
                }

                let nodes = (0..=(level + 1 * 3))
                    .map(|i| i + (1000 * (level + 1)))
                    .into_iter()
                    .collect::<Vec<usize>>();
                for id in nodes.iter() {
                    let children = build_branch(mocked_dom, *id, level + 1);
                    mocked_dom.add_with_depth(
                        *id,
                        Some(root),
                        children,
                        Node::from_size_and_direction(
                            Size::Pixels(Length::new(100.0)),
                            Size::Pixels(Length::new(100.0)),
                            DirectionMode::Vertical,
                        ),
                        level as u16,
                    );
                }
                nodes
            }

            build_branch(&mut mocked_dom, 0, 0);

            layout.find_best_root(&mut mocked_dom);
            layout.measure(
                0,
                Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
                &mut measurer,
                &mut mocked_dom,
            );

            b.iter(|| {
                black_box({
                    mocked_dom.set_node(
                        1,
                        Node::from_size_and_direction(
                            Size::Inner,
                            Size::Pixels(Length::new(10.0)),
                            DirectionMode::Vertical,
                        ),
                    );
                    layout.invalidate(8013);
                    layout.find_best_root(&mut mocked_dom);
                    layout.measure(
                        0,
                        Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
                        &mut measurer,
                        &mut mocked_dom,
                    )
                });
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
