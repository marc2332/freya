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
    let padding = use_state(&cx, || 10);

    use_effect(&cx, padding, |padding| async move {
        sleep(Duration::from_millis(1)).await;
        padding.with_mut(|padding| {
            if *padding < 2000 {
                *padding += 1;
            } else {
                *padding = 0;
            }
        });
    });
    
    render!(
        rect { }
    )
}
