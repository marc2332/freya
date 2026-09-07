use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn moving_an_a11y_id_between_siblings() {
    let (mut test, mut holder) = TestingRunner::new(
        || {
            let holder = use_consume::<State<usize>>();
            let a11y_id = use_a11y();

            rect().children(
                (0..3)
                    .map(|index| {
                        rect()
                            .width(Size::px(100.))
                            .height(Size::px(100.))
                            .maybe(index == *holder.read(), |el| el.a11y_id(a11y_id))
                            .into_element()
                    })
                    .collect::<Vec<_>>(),
            )
        },
        (500., 500.).into(),
        |runner| runner.provide_root_context(|| State::create(0usize)),
        1.,
    );
    test.sync_and_update();

    for index in 1..3 {
        holder.set(index);
        let update = test.sync_and_update();

        let mut children = update
            .nodes
            .iter()
            .flat_map(|(_, node)| node.children())
            .collect::<Vec<_>>();
        let total = children.len();
        children.sort_unstable();
        children.dedup();
        assert_eq!(children.len(), total, "duplicated accessibility children");
    }
}
