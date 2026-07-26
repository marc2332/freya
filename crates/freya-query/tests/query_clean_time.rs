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

/// Kept much larger than the 10ms poll steps so scheduler jitter on a loaded runner
/// cannot fire the clean task inside a window a test means to keep it out of.
const CLEAN_TIME: Duration = Duration::from_millis(250);

#[derive(Clone, PartialEq, Hash, Eq)]
struct InstantFetch(Runs);

impl QueryCapability for InstantFetch {
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
            Ok(keys)
        }
    }
}

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
            async_io::Timer::after(Duration::from_millis(800)).await;
            Ok(keys)
        }
    }
}

/// A settled entry is served from cache while it lives, and a fresh execution only
/// happens once the entry has actually been cleaned.
#[test]
fn an_idle_entry_clears_after_clean_time_without_subscribers() {
    #[derive(PartialEq)]
    struct Subscriber;

    impl Component for Subscriber {
        fn render(&self) -> impl IntoElement {
            let runs = use_consume::<Runs>();
            let query = use_query(
                Query::new(0usize, InstantFetch(runs))
                    .stale_time(Duration::MAX)
                    .clean_time(CLEAN_TIME),
            );

            label().text(format!("{:?}", query.read().state()))
        }
    }

    fn app() -> impl IntoElement {
        let mounted = use_consume::<State<bool>>();
        let mounted = *mounted.read();

        rect().maybe_child(mounted.then_some(Subscriber))
    }

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

    // Mount and settle
    test.sync_and_update();
    test.poll(Duration::from_millis(10), Duration::from_millis(100));
    assert_eq!(runs.get(), 1, "the query did not run on mount");

    // Unmount and remount within the clean time: the cached value is reused
    *mounted.write() = false;
    test.poll_n(Duration::from_millis(10), 2);
    *mounted.write() = true;
    test.poll(Duration::from_millis(10), Duration::from_millis(100));
    assert_eq!(
        runs.get(),
        1,
        "remounting within the clean time re-ran the query instead of reusing the cache"
    );

    // Unmount past the clean time: the entry is cleared and a remount runs it again
    *mounted.write() = false;
    test.poll(Duration::from_millis(10), Duration::from_millis(600));
    *mounted.write() = true;
    test.poll(Duration::from_millis(10), Duration::from_millis(100));
    assert_eq!(
        runs.get(),
        2,
        "the entry was never cleaned after the clean time passed with no subscribers"
    );
}

/// The clean task must not remove an entry whose execution is still in flight: the
/// settlement would be orphaned and a remounting subscriber would dispatch a duplicate.
#[test]
fn an_entry_with_an_execution_in_flight_survives_clean_time() {
    #[derive(PartialEq)]
    struct Subscriber;

    impl Component for Subscriber {
        fn render(&self) -> impl IntoElement {
            let runs = use_consume::<Runs>();
            let query = use_query(
                Query::new(0usize, SlowFetch(runs))
                    .stale_time(Duration::MAX)
                    .clean_time(CLEAN_TIME),
            );

            label().text(format!("{:?}", query.read().state()))
        }
    }

    fn app() -> impl IntoElement {
        let mounted = use_consume::<State<bool>>();
        let mounted = *mounted.read();

        rect().maybe_child(mounted.then_some(Subscriber))
    }

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

    // Mount and dispatch
    test.sync_and_update();
    test.poll_n(Duration::from_millis(10), 2);
    assert_eq!(runs.get(), 1, "the query did not run on mount");

    // Unmount past the clean time while the execution is still in flight
    *mounted.write() = false;
    test.poll(Duration::from_millis(10), Duration::from_millis(400));

    // Remount: the entry must still exist, holding the very same execution
    *mounted.write() = true;
    test.poll(Duration::from_millis(10), Duration::from_millis(800));
    assert_eq!(
        runs.get(),
        1,
        "the in flight entry was cleaned and the remount dispatched a duplicate execution"
    );

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

/// One subscriber unmounting must not schedule cleanup while another remains mounted.
#[test]
fn an_entry_keeps_its_cache_while_another_subscriber_remains() {
    #[derive(PartialEq)]
    struct Subscriber;

    impl Component for Subscriber {
        fn render(&self) -> impl IntoElement {
            let runs = use_consume::<Runs>();
            let query = use_query(
                Query::new(0usize, InstantFetch(runs))
                    .stale_time(Duration::MAX)
                    .clean_time(CLEAN_TIME),
            );

            label().text(format!("{:?}", query.read().state()))
        }
    }

    fn app() -> impl IntoElement {
        let mounted = use_consume::<State<[bool; 2]>>();
        let [first, second] = *mounted.read();

        rect()
            .maybe_child(first.then_some(Subscriber))
            .maybe_child(second.then_some(Subscriber))
    }

    let (mut test, (runs, mut mounted)) = TestingRunner::new(
        app,
        (200., 200.).into(),
        |runner| {
            (
                runner.provide_root_context(|| Captured(Rc::new(Cell::new(0usize)))),
                runner.provide_root_context(|| State::create([true, true])),
            )
        },
        1.,
    );

    // Mount both and settle: one shared entry, one execution
    test.sync_and_update();
    test.poll(Duration::from_millis(10), Duration::from_millis(100));
    assert_eq!(
        runs.get(),
        1,
        "the two subscribers did not share one execution"
    );

    // Drop one subscriber and wait well past the clean time
    *mounted.write() = [true, false];
    test.poll(Duration::from_millis(10), Duration::from_millis(600));

    // Remount it: the entry survived with the remaining subscriber, so no re-run
    *mounted.write() = [true, true];
    test.poll(Duration::from_millis(10), Duration::from_millis(100));
    assert_eq!(
        runs.get(),
        1,
        "the entry was cleaned even though a subscriber was still mounted"
    );
}
