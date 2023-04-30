#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let progress_1 = use_state(cx, || 5);
    let progress_2 = use_state(cx, || 10);
    let progress_3 = use_state(cx, || 15);

    use_effect(cx, progress_1, |mut progress| async move {
        sleep(Duration::from_millis(15)).await;
        if *progress < 100 {
            progress += 2;
        } else {
            progress.set(0)
        }
    });

    use_effect(cx, progress_2, |mut progress| async move {
        sleep(Duration::from_millis(5)).await;
        if *progress < 100 {
            progress += 1;
        } else {
            progress.set(0)
        }
    });

    use_effect(cx, progress_3, |mut progress| async move {
        sleep(Duration::from_millis(30)).await;
        if *progress < 100 {
            progress += 1;
        } else {
            progress.set(0)
        }
    });

    render!(
        LoadingBar {
            progress: *progress_1.get()
        }
        LoadingBar {
            progress: *progress_2.get()
        }
        LoadingBar {
            progress: *progress_3.get()
        }
    )
}

#[derive(PartialEq, Props)]
struct LoadingBarProps {
    progress: i32,
}

#[allow(non_snake_case)]
fn LoadingBar(cx: Scope<LoadingBarProps>) -> Element {
    render!(
        rect {
            width: "auto",
            height: "30",
            padding: "7",
            background: "white",
            rect {
                width: "{&cx.props.progress}%",
                height: "100%",
                background: "blue",
            }
        }
    )
}
