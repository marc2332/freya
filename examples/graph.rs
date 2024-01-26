#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app() -> Element {
    let labels = use_hook(|| {
        vec![
            "15/5/23".to_string(),
            "16/5/23".to_string(),
            "17/5/23".to_string(),
            "18/5/23".to_string(),
            "19/5/23".to_string(),
        ]
    });
    let data = use_signal(|| {
        vec![
            GraphLine::new(
                "rgb(255, 184, 76)",
                vec![Some(45), Some(5), Some(182), Some(105), Some(60)],
            ),
            GraphLine::new(
                "rgb(44, 211, 225)",
                vec![Some(80), Some(20), Some(50), Some(90), Some(150)],
            ),
            GraphLine::new(
                "rgb(27, 156, 133)",
                vec![Some(200), Some(150), Some(100), Some(130), Some(40)],
            ),
            GraphLine::new(
                "rgb(210, 83, 128)",
                vec![Some(20), Some(50), Some(80), Some(110), Some(140)],
            ),
            GraphLine::new(
                "rgb(90, 5, 180)",
                vec![None, None, Some(5), Some(60), Some(100)],
            ),
        ]
    });

    rsx!(Graph {
        labels: labels.clone(),
        data: data.read().clone()
    })
}
