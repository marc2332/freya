#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_title("Native Drag and Drop")
                .with_size(560., 420.),
        ),
    )
}

fn app() -> impl IntoElement {
    let mut message = use_state(|| "Drop text or files here".to_string());
    let mut status = use_state(|| "Drag the card here or into another application".to_string());
    let mut hovering = use_state(|| false);

    rect()
        .expanded()
        .padding(24.)
        .spacing(24.)
        .child(
            NativeDrag::new(|| TransferData::text("Hello from Freya!"))
                .preview(|| {
                    rect()
                        .padding(16.)
                        .background((30, 120, 210))
                        .color(Color::WHITE)
                        .child("Hello from Freya!")
                        .into_element()
                })
                .preview_size(220., 64.)
                .on_finished(move |result: Result<NativeDragResult, DragError>| {
                    status.set_if_modified(match result {
                        Ok(NativeDragResult::Dropped(_)) => "Drag completed".into(),
                        Ok(NativeDragResult::Cancelled) => "Drag cancelled".into(),
                        Err(DragError::Unsupported) => {
                            "Dragging is not supported on this platform".into()
                        }
                        Err(error) => format!("Could not start the drag: {error}"),
                    });
                })
                .child(
                    rect()
                        .padding(20.)
                        .background((30, 120, 210))
                        .color(Color::WHITE)
                        .child("Drag this text"),
                ),
        )
        .child(
            rect()
                .expanded()
                .center()
                .padding(16.)
                .background(if hovering() {
                    (160, 220, 170)
                } else {
                    (230, 230, 230)
                })
                .on_drag_over(move |event: Event<DragEventData>| {
                    if event.data.has_text() || event.data.has_files() {
                        hovering.set_if_modified(true);
                        event.accept(DropAction::Copy);
                    }
                })
                .on_drag_leave(move |_| hovering.set_if_modified(false))
                .on_drop(move |event: Event<DragEventData>| {
                    hovering.set_if_modified(false);
                    let data = event.data.clone();
                    spawn(async move {
                        message.set_if_modified(match read_drop(&data).await {
                            Ok(content) => content,
                            Err(DragError::Unsupported) => {
                                "Dropping data is not supported on this platform".into()
                            }
                            Err(DragError::Expired) => {
                                "The dropped data is no longer available. Try dropping it again."
                                    .into()
                            }
                            Err(error) => format!("Could not read the dropped data: {error}"),
                        });
                    });
                })
                .child(message.read().clone()),
        )
        .child(status.read().clone())
}

async fn read_drop(data: &DragDataHandle) -> Result<String, DragError> {
    let (files, text) = futures_lite::future::zip(data.read_files(), data.read_text()).await;
    match files {
        Ok(paths) => {
            let count = paths.len();
            let paths = paths
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join("\n");
            Ok(format!("Dropped files ({count}):\n{paths}"))
        }
        Err(file_error) => {
            text.map(|text| format!("Dropped text:\n{text}"))
                .map_err(|text_error| {
                    if data.has_files() {
                        file_error
                    } else {
                        text_error
                    }
                })
        }
    }
}
