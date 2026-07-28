use std::{
    cell::Cell,
    rc::Rc,
};

#[cfg(debug_assertions)]
use async_io::Timer;
use freya::prelude::Size;
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

    // A capability no component uses has nothing cached
    assert!(
        test.run_in(|| QueriesStorage::<CountCalls>::peek_matching(0))
            .is_empty()
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
            runner.provide_root_context(|| {
                QueriesStorage::<GetUserName>::mocked(|_keys| Ok("Mocked".to_string()))
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
fn mocked_async_query_goes_through_loading() {
    fn app() -> impl IntoElement {
        let user = use_query(Query::new(0, GetUserName(Captured(FancyClient))));
        rect().child(label().text(format!("{:?}", user.read().state())))
    }

    let (mut test, _) = TestingRunner::new(
        app,
        (200., 200.).into(),
        |runner| {
            runner.provide_root_context(|| {
                QueriesStorage::<GetUserName>::mocked_async(|keys| async move {
                    Timer::after(std::time::Duration::from_millis(150)).await;
                    match keys {
                        0 => Ok("Mocked".to_string()),
                        _ => Err(()),
                    }
                })
            })
        },
        1.,
    );

    let state_is = |test: &mut TestingRunner, expected: &str| {
        let label = test
            .find(|node, element| Label::try_downcast(element).map(|_| node))
            .unwrap();
        let element = label.element();
        Label::try_downcast(&*element)
            .unwrap()
            .text
            .contains(expected)
    };

    test.sync_and_update();
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(50),
    );

    assert!(
        state_is(&mut test, "Loading"),
        "the mock resolved instantly, the loading state was not observable"
    );

    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(300),
    );

    assert!(state_is(&mut test, "Settled { Ok(\"Mocked\") }"));
}
