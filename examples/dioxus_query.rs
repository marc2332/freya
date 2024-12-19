#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_query::prelude::*;
use freya::prelude::*;
use serde::Deserialize;

fn main() {
    launch_with_props(app, "dioxus-query", (400.0, 350.0));
}

#[derive(PartialEq, Eq, Clone, Hash)]
enum QueryKey {
    RandomJoke,
}

#[derive(PartialEq)]
enum QueryValue {
    Joke(Joke),
}

#[derive(Deserialize, PartialEq)]
struct Joke {
    setup: String,
    punchline: String,
}

async fn get_random_joke() -> Option<QueryValue> {
    let res = reqwest::get("https://official-joke-api.appspot.com/random_joke")
        .await
        .ok()?;
    let data = res.json::<Joke>().await.ok()?;

    Some(QueryValue::Joke(data))
}

fn app() -> Element {
    use_init_query_client::<QueryValue, (), QueryKey>();
    let client = use_query_client::<QueryValue, (), QueryKey>();
    let joke_query = use_get_query([QueryKey::RandomJoke], |_| async {
        get_random_joke().await.ok_or(()).into()
    });

    let new_joke = move |_| {
        client.invalidate_query(QueryKey::RandomJoke);
    };

    rsx!(
        rect {
            height: "50%",
            width: "100%",
            main_align: "center",
            cross_align: "center",
            match joke_query.result().value() {
                QueryResult::Ok(QueryValue::Joke(joke)) => {
                    rsx!(
                        label {
                            "{joke.setup}"
                        }
                        label {
                            "{joke.punchline}"
                        }
                    )
                }
                QueryResult::Loading(_) => {
                    rsx!(
                        label {
                            "Loading"
                        }
                    )
                }
                QueryResult::Err(_) => {
                    rsx!(
                        label {
                            "An error ocurred."
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
