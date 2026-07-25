use std::{
    cell::Cell,
    rc::Rc,
    time::Duration,
};

use freya::prelude::*;
use freya_query::prelude::*;
use freya_testing::prelude::*;

/// Counts how many times the capability actually ran.
type Runs = Captured<Rc<Cell<usize>>>;

#[derive(Clone, PartialEq, Hash, Eq)]
struct SlowFetch(Runs);

impl QueryCapability for SlowFetch {
    type Ok = usize;
    type Err = ();
    type Keys = usize;

    fn run(
        &self,
        keys: &Self::Keys,
    ) -> impl core::future::Future<Output = Result<Self::Ok, Self::Err>> {
        let runs = self.0.clone();
        let keys = *keys;
        async move {
            runs.set(runs.get() + 1);
            async_io::Timer::after(Duration::from_millis(150)).await;
            Ok(keys)
        }
    }
}

#[derive(PartialEq)]
struct Subscriber;

impl Component for Subscriber {
    fn render(&self) -> impl IntoElement {
        let runs = use_consume::<Runs>();
        let query = use_query(Query::new(0usize, SlowFetch(runs)));

        label().text(format!("{:?}", query.read().state()))
    }
}

fn app() -> impl IntoElement {
    let mounted = use_consume::<State<bool>>();
    let mounted = *mounted.read();

    rect().maybe_child(mounted.then_some(Subscriber))
}

#[test]
fn remounting_while_running_does_not_duplicate_the_execution() {
    let (mut test, (runs, mut mounted)) = TestingRunner::new(
        app,
        (200., 200.).into(),
        |runner| {
            (
                runner.provide_root_context(|| Captured(Rc::new(Cell::new(0usize)))),
                runner.provide_root_context(|| State::create(true)),
            )
        },
        1.,
    );

    // Let the subscriber mount and dispatch the query
    test.sync_and_update();
    test.poll_n(Duration::from_millis(10), 2);
    assert_eq!(runs.get(), 1, "the query did not run on mount");

    // Unmount and remount it while that execution is still in flight
    *mounted.write() = false;
    test.poll_n(Duration::from_millis(10), 2);
    *mounted.write() = true;
    test.poll_n(Duration::from_millis(10), 2);

    assert_eq!(
        runs.get(),
        1,
        "remounting while the query was running dispatched a duplicate execution"
    );

    // And it still settles for the remounted subscriber
    test.poll(Duration::from_millis(10), Duration::from_millis(300));
    assert_eq!(runs.get(), 1);

    let label = test
        .find(|node, element| Label::try_downcast(element).map(|_| node))
        .unwrap();
    assert!(
        Label::try_downcast(&*label.element())
            .unwrap()
            .text
            .contains("Settled"),
        "the remounted subscriber never saw the in flight execution settle"
    );
}
