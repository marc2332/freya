#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::time::Duration;

use freya::prelude::*;
use tokio::time::sleep;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let range = use_state(cx, || 0);

    use_effect(cx, range, |range| async move {
        sleep(Duration::from_millis(500)).await;
        range.set(*range.get());
    });

    use_effect(cx, range, |range| async move {
        sleep(Duration::from_millis(1000)).await;
        range.with_mut(|v| *v+=1 );
    });

    let children = (0..*range.get())
        .map(|i| {
            rsx!(label { color:"black", key: "{i}", "{i}" })
        });

    render!(
        rect {
            children
        }
    )
}