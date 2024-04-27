use freya::prelude::*;
use freya_engine::prelude::Color;
use freya_testing::prelude::*;
use std::{path::PathBuf, str::FromStr};

#[tokio::test]
pub async fn filedrop_events() {
    #[derive(PartialEq)]
    enum Status {
        Idle,
        Hovering,
    }

    fn filedrop_events_app() -> Element {
        let mut path = use_signal::<Option<String>>(|| None);
        let mut status = use_signal(|| Status::Idle);

        let msg = path.read().clone().unwrap_or("Default".to_string());

        let background = if *status.read() == Status::Hovering {
            "red"
        } else {
            "blue"
        };

        rsx!(
            rect {
                height: "100%",
                width: "100%",
                background: "{background}",
                onglobalfilehover: move |_| status.set(Status::Hovering),
                onglobalfilehovercancelled: move |_| status.set(Status::Idle),
                onfiledrop: move |e| {
                    status.set(Status::Idle);
                    path.set(Some(e.file_path.as_ref().unwrap().to_string_lossy().to_string()))
                },
                label {
                    "{msg}"
                }
            }
        )
    }

    let mut utils = launch_test(filedrop_events_app);

    let root = utils.root();

    assert_eq!(root.get(0).get(0).get(0).text(), Some("Default"));
    assert_eq!(root.get(0).style().background, Fill::Color(Color::BLUE));

    utils.push_event(PlatformEvent::File {
        name: EventName::GlobalFileHover,
        cursor: (5., 5.).into(),
        file_path: None,
    });

    utils.wait_for_update().await;

    assert_eq!(root.get(0).style().background, Fill::Color(Color::RED));

    utils.push_event(PlatformEvent::File {
        name: EventName::FileDrop,
        cursor: (5., 5.).into(),
        file_path: Some(PathBuf::from_str("/nice/path/right.rs").unwrap()),
    });

    utils.wait_for_update().await;

    assert_eq!(
        root.get(0).get(0).get(0).text(),
        Some("/nice/path/right.rs")
    );
    assert_eq!(root.get(0).style().background, Fill::Color(Color::BLUE));
}
