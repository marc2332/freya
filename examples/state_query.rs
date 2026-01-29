use std::time::Duration;

use freya::{
    prelude::*,
    query::*,
};

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

    async fn run(&self, user_id: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        async_io::Timer::after(Duration::from_secs(2)).await;
        match user_id {
            0 => Ok(self.0.name().to_string()),
            _ => Err(()),
        }
    }
}

fn app() -> impl IntoElement {
    let user = use_query(Query::new(0, GetUserName(Captured(FancyClient))));

    rect().child(format!("{:?}", user.read().state())).child(
        Button::new()
            .on_press(move |_| user.invalidate())
            .child("Refresh"),
    )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}
