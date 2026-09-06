use std::{
    cell::Cell,
    rc::Rc,
};

use freya::prelude::Size;
#[cfg(debug_assertions)]
use freya::prelude::timer;
use freya_query::prelude::*;
use freya_testing::prelude::*;

#[derive(Clone, PartialEq, Hash, Eq)]
struct FancyClient;

impl FancyClient {
    pub fn name(&self) -> &'static str {
        "Marc"
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct GetUserName(Captured<FancyClient>);

impl QueryCapability for GetUserName {
    type Ok = String;
    type Err = ();
    type Keys = usize;

    fn run(
        &self,
        user_id: &Self::Keys,
    ) -> impl core::future::Future<Output = Result<Self::Ok, Self::Err>> {
        let client = self.0.clone();
        async move {
            match user_id {
                0 => Ok(client.name().to_string()),
                _ => Err(()),
            }
        }
    }
}

#[test]
fn query_basic() {
    fn app() -> impl IntoElement {
        let user = use_query(Query::new(0, GetUserName(Captured(FancyClient))));
        rect().child(label().text(format!("{:?}", user.read().state())))
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    // Wait for the query to settle
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(200),
    );

    let label = test
        .find(|node, element| Label::try_downcast(element).map(|_| node))
        .unwrap();

    assert!(
        Label::try_downcast(&*label.element())
            .unwrap()
            .text
            .contains("Settled")
    );
}

#[derive(PartialEq)]
struct QuerySubscriber;

impl Component for QuerySubscriber {
    fn render(&self) -> impl IntoElement {
        use_query(
            Query::new(0usize, GetUserName(Captured(FancyClient)))
                .clean_time(std::time::Duration::from_millis(50)),
        );
        rect()
    }
}

#[derive(PartialEq)]
struct SubscriberSlot {
    mounted: bool,
}

impl Component for SubscriberSlot {
    fn render(&self) -> impl IntoElement {
        rect().maybe_child(self.mounted.then(|| QuerySubscriber))
    }
}

#[test]
fn query_survives_subscriber_handover() {
    fn app() -> impl IntoElement {
        let mut stage = use_state(|| 0usize);
        let stage_value = *stage.read();
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .on_press(move |_| {
                *stage.write() += 1;
            })
            .child(SubscriberSlot {
                mounted: stage_value == 1,
            })
            .child(SubscriberSlot {
                mounted: stage_value == 0,
            })
    }

    let mut test = launch_test(app);
    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(100),
    );

    // Mount the second subscriber before the first one unmounts
    test.click_cursor((100.0, 100.0));

    // Wait past the clean time so the wrongly scheduled clean task fires
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(200),
    );

    // Unmounting the surviving subscriber used to panic in update_tasks
    test.click_cursor((100.0, 100.0));
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(200),
    );
}

#[test]
fn query_reactive_subcontext_reruns_on_keys_change() {
    fn app() -> impl IntoElement {
        let mut observed = use_consume::<State<Vec<bool>>>();
        let mut user_id = use_state(|| 0usize);

        let user = use_query(Query::new(
            *user_id.read(),
            GetUserName(Captured(FancyClient)),
        ));

        // Records every settled result this reactive context sees.
        use_side_effect(move || {
            let reader = user.read();
            let state = reader.state();
            if state.is_ok() || state.is_err() {
                observed.write().push(state.is_ok());
            }
        });

        rect()
            .width(Size::fill())
            .height(Size::fill())
            .on_press(move |_| {
                *user_id.write() = 1;
            })
    }

    let (mut test, observed) = TestingRunner::new(
        app,
        (200., 200.).into(),
        |runner| runner.provide_root_context(|| State::create(Vec::<bool>::new())),
        1.,
    );

    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(200),
    );

    assert_eq!(&*observed.peek(), &[true]);

    // Changing the keys re-runs the query with key `1`, which fails.
    test.click_cursor((100.0, 100.0));
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(200),
    );

    assert_eq!(
        &*observed.peek(),
        &[true, false],
        "the reactive context was not notified when the query re-ran"
    );
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct CountCalls(Captured<Rc<Cell<usize>>>);

impl QueryCapability for CountCalls {
    type Ok = usize;
    type Err = ();
    type Keys = usize;

    fn run(
        &self,
        _keys: &Self::Keys,
    ) -> impl core::future::Future<Output = Result<Self::Ok, Self::Err>> {
        let calls = self.0.clone();
        async move {
            calls.set(calls.get() + 1);
            Ok(calls.get())
        }
    }
}

#[test]
fn invalidate_all_from_the_test_runtime() {
    fn app() -> impl IntoElement {
        let calls = use_consume::<Captured<Rc<Cell<usize>>>>();
        use_query(Query::new(0, CountCalls(calls)));
        rect()
    }

    let (mut test, calls) = TestingRunner::new(
        app,
        (200., 200.).into(),
        |runner| runner.provide_root_context(|| Captured(Rc::new(Cell::new(0usize)))),
        1.,
    );

    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(200),
    );

    assert_eq!(calls.get(), 1);

    test.run_in(|| {
        spawn_forever(async move {
            QueriesStorage::<CountCalls>::invalidate_all().await;
        });
    });
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(200),
    );

    assert_eq!(calls.get(), 2);
}

#[test]
fn peek_query_state_from_the_test() {
    fn app() -> impl IntoElement {
        use_query(Query::new(0, GetUserName(Captured(FancyClient))));
        rect()
    }

    let mut test = launch_test(app);
    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(200),
    );

    let name = test.run_in(|| {
        QueriesStorage::<GetUserName>::peek_matching(0)
            .first()
            .and_then(|query| query.state().ok().cloned())
    });

    assert_eq!(name.as_deref(), Some("Marc"));
}

#[test]
#[cfg(debug_assertions)]
fn mocked_query() {
    fn app() -> impl IntoElement {
        let user = use_query(Query::new(0, GetUserName(Captured(FancyClient))));
        rect().child(label().text(format!("{:?}", user.read().state())))
    }

    let (mut test, _) = TestingRunner::new(
        app,
        (200., 200.).into(),
        |runner| {
            runner.run_in(|| {
                GlobalContexts::get().insert_context(QueriesStorage::<GetUserName>::mocked(
                    |_keys| Ok("Mocked".to_string()),
                ))
            })
        },
        1.,
    );

    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(200),
    );

    let label = test
        .find(|node, element| Label::try_downcast(element).map(|_| node))
        .unwrap();
    let element = label.element();
    let text = &Label::try_downcast(&*element).unwrap().text;

    assert!(
        text.contains("Mocked"),
        "the mock did not replace the query, got {text}"
    );
}

#[test]
#[cfg(debug_assertions)]
fn mocked_async_query() {
    fn app() -> impl IntoElement {
        use_query(Query::new(0, GetUserName(Captured(FancyClient))));
        rect()
    }

    let (mut test, _) = TestingRunner::new(
        app,
        (200., 200.).into(),
        |runner| {
            runner.run_in(|| {
                GlobalContexts::get().insert_context(QueriesStorage::<GetUserName>::mocked_async(
                    |keys| async move {
                        timer(std::time::Duration::from_millis(20)).await;
                        Ok(format!("Mocked {keys}"))
                    },
                ))
            })
        },
        1.,
    );

    test.sync_and_update();

    assert!(test.run_in(|| {
        QueriesStorage::<GetUserName>::peek_matching(0)[0]
            .state()
            .is_loading()
    }));

    test.poll(
        std::time::Duration::from_millis(5),
        std::time::Duration::from_millis(100),
    );

    let name = test.run_in(|| {
        QueriesStorage::<GetUserName>::peek_matching(0)[0]
            .state()
            .ok()
            .cloned()
    });

    assert_eq!(name.as_deref(), Some("Mocked 0"));
}
