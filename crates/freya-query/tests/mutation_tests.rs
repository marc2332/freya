use std::{
    cell::RefCell,
    rc::Rc,
};

use freya::prelude::Size;
use freya_query::prelude::*;
use freya_testing::prelude::*;

#[derive(Clone, PartialEq, Hash, Eq)]
struct GetUserName(Captured<Rc<RefCell<String>>>);

impl QueryCapability for GetUserName {
    type Ok = String;
    type Err = ();
    type Keys = usize;

    fn run(
        &self,
        user_id: &Self::Keys,
    ) -> impl core::future::Future<Output = Result<Self::Ok, Self::Err>> {
        let client = self.0.clone();
        let user_id = *user_id;
        async move {
            match user_id {
                0 => Ok(client.borrow().clone()),
                _ => Err(()),
            }
        }
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct SetUserName(Captured<Rc<RefCell<String>>>);

impl MutationCapability for SetUserName {
    type Ok = ();
    type Err = ();
    type Keys = (usize, String);

    async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let client = self.0.clone();
        let keys = keys.clone();
        *client.borrow_mut() = keys.1;
        Ok(())
    }

    async fn on_settled(&self, keys: &Self::Keys, _result: &Result<Self::Ok, Self::Err>) {
        let user_id = keys.0;
        QueriesStorage::<GetUserName>::invalidate_matching(user_id).await;
    }
}

#[test]
fn mutation_basic() {
    fn app() -> impl IntoElement {
        let client = use_hook(|| Captured(Rc::new(RefCell::new(String::from("Marc")))));
        let user = use_query(Query::new(0usize, GetUserName(client.clone())));
        let mutation = use_mutation(Mutation::new(SetUserName(client.clone())));

        use_after_side_effect(move || {
            mutation.mutate((0usize, "John".to_string()));
        });

        rect().child(label().text(format!("{:?}", user.read().state())))
    }

    let mut test = launch_test(app);
    test.sync_and_update();

    let label = test
        .find(|node, element| Label::try_downcast(element).map(|_| node))
        .unwrap();

    let initial_text = &Label::try_downcast(&*label.element()).unwrap().text;
    assert!(
        initial_text.contains("Pending")
            || initial_text.contains("Loading")
            || initial_text.contains("Settled")
    );

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
            .contains("John")
    );
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct CheckUser {
    user_id: usize,
}

impl MutationCapability for CheckUser {
    type Ok = ();
    type Err = ();
    type Keys = ();

    async fn run(&self, _keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        match self.user_id {
            0 => Ok(()),
            _ => Err(()),
        }
    }
}

#[test]
fn mutation_reactive_context_reruns_on_identity_change() {
    fn app() -> impl IntoElement {
        let mut observed = use_consume::<State<Vec<bool>>>();
        let mut user_id = use_state(|| 0usize);

        let mutation = use_mutation(Mutation::new(CheckUser {
            user_id: *user_id.read(),
        }));

        // Records every settled result this reactive context sees.
        use_side_effect(move || {
            let reader = mutation.read();
            let state = reader.state();
            if state.is_ok() || state.is_err() {
                observed.write().push(state.is_ok());
            }
        });

        // Runs the mutation again whenever the identity changes.
        use_after_side_effect(move || {
            let _ = user_id.read();
            mutation.mutate(());
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

    // Changing the identity re-runs the mutation with user `1`, which fails.
    test.click_cursor((100.0, 100.0));
    test.poll(
        std::time::Duration::from_millis(10),
        std::time::Duration::from_millis(200),
    );

    assert_eq!(
        &*observed.peek(),
        &[true, false],
        "the reactive context was not notified when the mutation re-ran"
    );
}
