use std::time::Duration;

use freya::{
    prelude::*,
    query::*,
};

#[derive(Clone, PartialEq, Hash, Eq)]
struct FancyClient {
    name: String,
}

impl FancyClient {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct GetUserName(Captured<State<FancyClient>>);

impl QueryCapability for GetUserName {
    type Ok = String;
    type Err = ();
    type Keys = usize;

    async fn run(&self, user_id: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        async_io::Timer::after(Duration::from_secs(2)).await;
        match user_id {
            0 => Ok(self.0.read().name()),
            _ => Err(()),
        }
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct SetUserName(Captured<State<FancyClient>>);

impl MutationCapability for SetUserName {
    type Ok = ();
    type Err = ();
    type Keys = (usize, String);

    async fn run(&self, keys: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        let name = keys.1.clone();
        self.0.write_unchecked().set_name(name.clone());
        Ok(())
    }

    async fn on_settled(&self, keys: &Self::Keys, _result: &Result<Self::Ok, Self::Err>) {
        let user_id = keys.0;
        QueriesStorage::<GetUserName>::invalidate_matching(user_id).await;
    }
}

fn app() -> impl IntoElement {
    let client = use_state(|| FancyClient::new("Marc"));
    let user = use_query(Query::new(0, GetUserName(Captured(client))));
    let mutation = use_mutation(Mutation::new(SetUserName(Captured(client))));

    rect()
        .spacing(6.)
        .child(label().text(format!("User: {:?}", user.read().state())))
        .child(
            Button::new()
                .on_press(move |_| mutation.mutate((0, "John".to_string())))
                .child("Set to John"),
        )
        .child(
            Button::new()
                .on_press(move |_| mutation.mutate((0, "Jane".to_string())))
                .child("Set to Jane"),
        )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
