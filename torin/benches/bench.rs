use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
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
}

impl DOMAdapter<usize> for TestingDOM {
    fn children_of(&self, node_id: &usize) -> Vec<usize> {
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

    fn is_node_valid(&self, _node_id: &usize) -> bool {
        true
    }

    fn closest_common_parent(&self, node_id_a: &usize, _node_id_b: &usize) -> Option<usize> {
        Some(self.parent_of(node_id_a)?)
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("benchmarks");
    g.sample_size(100);

    g.bench_function("1 root 1000 direct children", |b| {
        let mut layout = Torin::<usize>::new();
        let mut measurer = Some(TestingMeasurer);
        let mut mocked_dom = TestingDOM::default();

        let children_ids = (1..=1001).into_iter().collect::<Vec<usize>>();

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

        for i in children_ids {
            mocked_dom.add(
                i,
                Some(0),
                vec![],
                Node::from_size_and_direction(
                    Size::Pixels(Length::new(100.0)),
                    Size::Pixels(Length::new(100.0)),
                    DirectionMode::Vertical,
                ),
            );
        }

        b.iter(|| {
            black_box(|| {
                layout.find_best_root(&mocked_dom);
                layout.measure(
                    0,
                    Rect::new(Point2D::new(0.0, 0.0), Size2D::new(1000.0, 1000.0)),
                    &mut measurer,
                    &mocked_dom,
                )
            })();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
