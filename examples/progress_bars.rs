use std::time::Duration;

use dioxus::prelude::*;
use tokio::time::sleep;
use trev::launch;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let progress_1 = use_state(&cx, || 5);
    let progress_2 = use_state(&cx, || 10);
    let progress_3 = use_state(&cx, || 15);

    use_effect(&cx, progress_1, |progress| async move {
        sleep(Duration::from_millis(15)).await;
        progress.with_mut(|padding| {
            if *padding < 100 {
                *padding += 2;
            } else {
                *padding = 0;
            }
        });
    });

    use_effect(&cx, progress_2, |progress| async move {
        sleep(Duration::from_millis(5)).await;
        progress.with_mut(|padding| {
            if *padding < 100 {
                *padding += 1;
            } else {
                *padding = 0;
            }
        });
    });

    use_effect(&cx, progress_3, |progress| async move {
        sleep(Duration::from_millis(30)).await;
        progress.with_mut(|padding| {
            if *padding < 100 {
                *padding += 1;
            } else {
                *padding = 0;
            }
        });
    });

    cx.render(rsx! {
        LoadingBar {
            progress: **progress_1
        }
        LoadingBar {
            progress: **progress_2
        }
        LoadingBar {
            progress: **progress_3
        }
    })
}

#[derive(PartialEq, Props)]
struct LoadingBarProps {
    progress: i32,
}

#[allow(non_snake_case)]
fn LoadingBar<'a>(cx: Scope<'a, LoadingBarProps>) -> Element {
    cx.render(rsx!(
        div {
            width: "auto",
            height: "30",
            padding: "15",
            background: "white",
            div {
                width: "{&cx.props.progress}%",
                height: "stretch",
                background: "blue",
            }
        }
    ))
}
