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
