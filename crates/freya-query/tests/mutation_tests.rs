use freya_query::prelude::*;
use freya_testing::prelude::*;
use std::{cell::RefCell, rc::Rc};

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

    fn run(
        &self,
        keys: &Self::Keys,
    ) -> impl core::future::Future<Output = Result<Self::Ok, Self::Err>> {
        let client = self.0.clone();
        let keys = keys.clone();
        async move {
            *client.borrow_mut() = keys.1;
            Ok(())
        }
    }

    fn on_settled(
        &self,
        keys: &Self::Keys,
        _result: &Result<Self::Ok, Self::Err>,
    ) -> impl core::future::Future<Output = ()> {
        let user_id = keys.0;
        async move {
            QueriesStorage::<GetUserName>::invalidate_matching(user_id).await;
        }
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
