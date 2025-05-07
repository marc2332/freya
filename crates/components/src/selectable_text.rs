use std::rc::Rc;

use dioxus::prelude::*;
use freya_core::platform::CursorIcon;
use freya_elements::{
    self as dioxus_elements,
    events::KeyboardEvent,
    MouseEvent,
};
use freya_hooks::{
    use_editable,
    use_focus,
    use_platform,
    EditableConfig,
    EditableEvent,
    EditableMode,
    TextEditor,
};

/// Current status of the SelectableText.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SelectableTextStatus {
    /// Default state.
    #[default]
    Idle,
    /// Mouse is hovering the text.
    Hovering,
}

/// Text that can be selected with a mouse or keyboard.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(SelectableText {
///         value: "You can select this looooooooooong text"
///     })
/// }
/// ```
#[component]
pub fn SelectableText(value: ReadOnlySignal<String>) -> Element {
    let platform = use_platform();
    let mut editable = use_editable(
        move || EditableConfig::new(value()).with_allow_changes(false),
        EditableMode::MultipleLinesSingleEditor,
    );
    let mut status = use_signal(SelectableTextStatus::default);
    let mut focus = use_focus();
    let mut drag_origin = use_signal(|| None);

    if &*value.read() != editable.editor().read().rope() {
        editable.editor_mut().write().set(&value.read());
        editable.editor_mut().write().editor_history().clear();
    }

    use_drop(move || {
        if *status.peek() == SelectableTextStatus::Hovering {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let a11y_id = focus.attribute();
    let cursor_reference = editable.cursor_attr();
    let highlights = editable.highlights_attr(0);

    let onmousedown = move |e: MouseEvent| {
        e.stop_propagation();
        drag_origin.set(Some(e.get_screen_coordinates() - e.element_coordinates));
        editable.process_event(&EditableEvent::MouseDown(e.data, 0));
        focus.request_focus();
    };

    let onglobalmousemove = move |mut e: MouseEvent| {
        if focus.is_focused() {
            if let Some(drag_origin) = drag_origin() {
                let data = Rc::get_mut(&mut e.data).unwrap();
                data.element_coordinates.x -= drag_origin.x;
                data.element_coordinates.y -= drag_origin.y;
                editable.process_event(&EditableEvent::MouseMove(e.data, 0));
            }
        }
    };

    let onmouseenter = move |_| {
        platform.set_cursor(CursorIcon::Text);
        *status.write() = SelectableTextStatus::Hovering;
    };

    let onmouseleave = move |_| {
        platform.set_cursor(CursorIcon::default());
        *status.write() = SelectableTextStatus::default();
    };

    let onclick = move |_: MouseEvent| {
        editable.process_event(&EditableEvent::Click);
    };

    let onkeydown = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyDown(e.data));
    };

    let onkeyup = move |e: KeyboardEvent| {
        editable.process_event(&EditableEvent::KeyUp(e.data));
    };

    let onglobalclick = move |_| {
        match *status.read() {
            SelectableTextStatus::Idle if focus.is_focused() => {
                editable.process_event(&EditableEvent::Click);
            }
            SelectableTextStatus::Hovering => {
                editable.process_event(&EditableEvent::Click);
            }
            _ => {}
        };

        // Unfocus text when this:
        // + is focused
        // + it has not just being dragged
        // + a global click happened
        if focus.is_focused() {
            if drag_origin.read().is_some() {
                drag_origin.set(None);
            } else {
                editable.editor_mut().write().clear_selection();
                focus.request_unfocus();
            }
        }
    };

    rsx!(
        paragraph {
            a11y_focusable: "true",
            a11y_id,
            cursor_id: "0",
            cursor_mode: "editable",
            cursor_color: "black",
            highlights,
            cursor_reference,
            onclick,
            onglobalmousemove,
            onmousedown,
            onmouseenter,
            onmouseleave,
            onglobalclick,
            onkeydown,
            onkeyup,
            text {
                "{editable.editor()}"
            }
        }
    )
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn selectable_text() {
        fn selectable_text_app() -> Element {
            rsx!(SelectableText {
                value: "Hello, World!"
            })
        }

        let mut utils = launch_test(selectable_text_app);

        // Initial state
        let root = utils.root().get(0);
        assert_eq!(root.get(0).get(0).text(), Some("Hello, World!"));
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        utils.push_event(TestEvent::Mouse {
            name: EventName::MouseDown,
            cursor: (3.0, 3.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        utils.push_event(TestEvent::Mouse {
            name: EventName::MouseMove,
            cursor: (55.0, 3.0).into(),
            button: Some(MouseButton::Left),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let root = utils.root().get(0);
        let highlights = root.state().cursor.highlights.clone();
        #[cfg(not(target_os = "macos"))]
        assert_eq!(highlights, Some(vec![(0, 8)]));

        #[cfg(target_os = "macos")]
        assert_eq!(highlights, Some(vec![(0, 7)]));
    }
}
