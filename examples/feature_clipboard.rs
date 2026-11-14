#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(
        LaunchConfig::new().with_window(
            WindowConfig::new(app)
                .with_title("Clipboard")
                .with_size(680., 520.),
        ),
    )
}

fn app() -> impl IntoElement {
    let text = use_state(|| "Hello from Freya!".to_string());
    let mut output = use_state(|| "Pasted text or file paths will appear here.".to_string());
    let mut busy = use_state(|| false);

    let mut paste = move |format: DataFormat| {
        if busy() {
            return;
        }

        busy.set(true);
        output.set("Reading the clipboard...".into());
        let read = Clipboard::get(format);

        spawn(async move {
            output.set(match read.await {
                Ok(DataValue::Text(text)) => format!("Pasted text:\n{text}"),
                Ok(DataValue::Files(paths)) => {
                    let count = paths.len();
                    let paths = paths
                        .iter()
                        .map(|path| path.display().to_string())
                        .collect::<Vec<_>>()
                        .join("\n");
                    format!("Pasted files ({count}):\n{paths}")
                }
                Ok(DataValue::Bytes(bytes)) => format!("Pasted {} bytes.", bytes.len()),
                Err(ClipboardError::UnsupportedFormat) => {
                    "The clipboard does not offer this format, or the platform does not support it."
                        .into()
                }
                Err(error) => format!("Could not paste: {error}"),
            });
            busy.set(false);
        });
    };

    rect()
        .expanded()
        .padding(24.)
        .spacing(16.)
        .child(label().text("Clipboard").font_size(24.))
        .child(Input::new(text).expanded().placeholder("Text to copy"))
        .child(
            rect()
                .horizontal()
                .spacing(8.)
                .child(
                    Button::new()
                        .enabled(!busy())
                        .on_press(move |_| {
                            if busy() {
                                return;
                            }

                            busy.set(true);
                            let write = Clipboard::set_text(text.read().clone());

                            spawn(async move {
                                output.set(match write.await {
                                    Ok(()) => {
                                        "Text copied. Paste it here or in another app.".into()
                                    }
                                    Err(error) => format!("Could not copy: {error}"),
                                });
                                busy.set(false);
                            });
                        })
                        .child("Copy text"),
                )
                .child(
                    Button::new()
                        .enabled(!busy())
                        .on_press(move |_| paste(DataFormat::Text))
                        .child("Paste text"),
                )
                .child(
                    Button::new()
                        .enabled(!busy())
                        .on_press(move |_| paste(DataFormat::Files))
                        .child("Paste files"),
                ),
        )
        .child("To paste files, copy them in a desktop file manager first.")
        .child("Copying text replaces the clipboard. File paste reads paths, not file contents.")
        .child(ScrollView::new().child(label().width(Size::fill()).text(output.read().clone())))
}
