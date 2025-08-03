#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_query::prelude::*;
use freya::prelude::*;
use serde::Deserialize;

fn main() {
    launch_with_params(app, "dioxus-query", (400.0, 350.0));
}

#[derive(Deserialize, PartialEq)]
struct Joke {
    setup: String,
    punchline: String,
}

#[derive(Clone)]
struct JokeClient;

impl JokeClient {
    async fn get_random_joke(&self) -> Option<Joke> {
        let res = reqwest::get("https://official-joke-api.appspot.com/random_joke")
            .await
            .ok()?;
        let joke_data = res.json::<Joke>().await.ok()?;

        Some(joke_data)
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct GetRandomJoke(Captured<JokeClient>);

impl QueryCapability for GetRandomJoke {
    type Ok = Joke;
    type Err = ();
    type Keys = ();

    async fn run(&self, _: &Self::Keys) -> Result<Self::Ok, Self::Err> {
        match self.0.get_random_joke().await {
            Some(joke) => Ok(joke),
            None => Err(()),
        }
    }
}

fn app() -> Element {
    let joke_query = use_query(Query::new((), GetRandomJoke(Captured(JokeClient))));

    let new_joke = move |_| async move {
        QueriesStorage::<GetRandomJoke>::invalidate_matching(()).await;
    };

    rsx!(
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            match &*joke_query.read().state() {
                QueryStateData::Settled { res, .. } => {
                    match res {
                        Ok(joke) => {
                            rsx!(
                                label {
                                    "{joke.setup}"
                                }
                                label {
                                    "{joke.punchline}"
                                }
                            )
                        }
                        Err(_) => {
                            rsx!(
                                label {
                                    "An error ocurred."
                                }
                            )
                        }
                    }
                }
                QueryStateData::Loading { .. } => {
                    rsx!(
                        label {
                            "Loading"
                        }
                    )
                }
                QueryStateData::Pending { .. } => {
                    rsx!(
                        label {
                            "Pending..."
                        }
                    )
                }
            }
        }
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            direction: "horizontal",
            Button {
                onpress: new_joke,
                label { "New joke" }
            }
        }
    )
}
