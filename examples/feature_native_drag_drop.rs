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
            NativeDrag::new(|| DragData::text("Hello from Freya!"))
                .actions([DropAction::Copy])
                .enabled(true)
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
                        let result = if data.has_files() {
                            data.read_files().await.map(|paths| {
                                let count = paths.len();
                                let paths = paths
                                    .iter()
                                    .map(|path| path.display().to_string())
                                    .collect::<Vec<_>>()
                                    .join("\n");
                                format!("Dropped files ({count}):\n{paths}")
                            })
                        } else {
                            data.read_text()
                                .await
                                .map(|text| format!("Dropped text:\n{text}"))
                        };
                        message.set_if_modified(match result {
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
