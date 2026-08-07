use std::time::Duration;

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
        let user = use_query(Query::new(0usize, GetUserName(Captured(FancyClient))));
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

#[derive(Clone, PartialEq, Hash, Eq)]
struct GetGreeting;

impl QueryCapability for GetGreeting {
    type Ok = String;
    type Err = ();
    type Keys = usize;

    fn run(
        &self,
        id: &Self::Keys,
    ) -> impl core::future::Future<Output = Result<Self::Ok, Self::Err>> {
        async move { Ok(format!("greeting-{id}")) }
    }
}

#[test]
fn query_keeps_old_data_on_keys_change() {
    fn app() -> impl IntoElement {
        let mut id = use_state(|| 0usize);
        let greeting = use_query(Query::new(*id.read(), GetGreeting).keep_old_data(true));

        rect()
            .expanded()
            .on_press(move |_| *id.write() = 1)
            .child(greeting.read().state().ok().cloned().unwrap_or_default())
    }

    fn greeting(test: &mut TestingRunner) -> String {
        let label = test
            .find(|node, element| Label::try_downcast(element).map(|_| node))
            .unwrap();
        Label::try_downcast(&*label.element())
            .unwrap()
            .text
            .to_string()
    }

    let mut test = launch_test(app);
    test.sync_and_update();
    test.poll(Duration::from_millis(10), Duration::from_millis(200));
    assert_eq!(greeting(&mut test), "greeting-0");

    // While the new keys load, the previous data is kept.
    test.click_cursor((100.0, 100.0));
    test.sync_and_update();
    assert_eq!(greeting(&mut test), "greeting-0");

    test.poll(Duration::from_millis(10), Duration::from_millis(200));
    assert_eq!(greeting(&mut test), "greeting-1");
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
