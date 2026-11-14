//! Touch interaction with editable text, as mobile platforms do it:
//!
//! - A tap places the cursor once the finger lifts, unless the finger moved to scroll.
//! - A double tap or a long press selects a word. Dragging after a long press extends the
//!   selection word by word.
//! - Handles under the cursor and at both ends of a selection move them.
//! - A toolbar offers cut, copy, paste and select all.
//!
//! Dragging a finger over the text never selects it on its own, it scrolls.

use std::{
    ops::Mul,
    time::{
        Duration,
        Instant,
    },
};

use freya_core::{
    elements::paragraph::{
        ParagraphCursorExt,
        ParagraphHolderInner,
    },
    prelude::*,
};
use torin::prelude::CursorPoint;

use crate::{
    EditorLine,
    TextEditor,
    TextSelection,
};

/// How long a finger stays down without moving before it selects a word.
pub const LONG_PRESS_TIMEOUT: Duration = Duration::from_millis(500);

/// How far a finger moves, in logical pixels, before its press becomes a scroll.
pub const TOUCH_SLOP: f64 = 8.0;

/// Maximum time between the two taps of a double tap.
const DOUBLE_TAP_TIMEOUT: Duration = Duration::from_millis(300);

/// Maximum distance between the two taps of a double tap.
const DOUBLE_TAP_SLOP: f64 = 100.0;

/// A handle that moves the cursor or an end of the selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionHandle {
    /// Under the cursor, when nothing is selected.
    Cursor,
    /// At the start of the selection.
    Start,
    /// At the end of the selection.
    End,
}

/// What a finger on the text is doing.
#[derive(Debug, Default, Clone, PartialEq)]
enum Gesture {
    #[default]
    Idle,
    /// Down and still, it may become a tap or a long press.
    Pressed { origin: CursorPoint },
    /// Moved away to scroll the content.
    Scrolling,
    /// Selected a word with a long press and extends the selection by words.
    SelectingWords { anchor: (usize, usize) },
    /// Drags a handle.
    DraggingHandle(SelectionHandle),
    /// Presses the toolbar.
    OnToolbar,
}

/// Touch state of an editable text: the ongoing gesture, and whether the selection handles and
/// the toolbar are shown.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TouchSelection {
    gesture: Gesture,
    last_tap: Option<(Instant, CursorPoint)>,
    handles: bool,
    toolbar: bool,
}

impl TouchSelection {
    /// Whether the selection handles are shown. They are once the user touched the text, until
    /// they type or use a mouse.
    pub fn handles_visible(&self) -> bool {
        self.handles
    }

    /// Whether the cut, copy and paste toolbar is shown.
    pub fn toolbar_visible(&self) -> bool {
        self.toolbar && !matches!(self.gesture, Gesture::Scrolling)
    }

    /// Whether a finger is selecting text, during which the content should not scroll.
    pub fn is_selecting(&self) -> bool {
        matches!(
            self.gesture,
            Gesture::SelectingWords { .. } | Gesture::DraggingHandle(_)
        )
    }

    /// The handle a finger is dragging.
    pub fn dragged_handle(&self) -> Option<SelectionHandle> {
        match self.gesture {
            Gesture::DraggingHandle(handle) => Some(handle),
            _ => None,
        }
    }

    /// Whether a finger that went down on the text, a handle or the toolbar is still down.
    pub fn is_active(&self) -> bool {
        !matches!(self.gesture, Gesture::Idle)
    }

    /// Whether the press of a finger is still waiting to become a tap or a long press.
    pub fn is_pressed(&self) -> bool {
        matches!(self.gesture, Gesture::Pressed { .. })
    }

    /// Hide the handles and the toolbar, e.g. when the user types.
    pub fn hide(&mut self) {
        self.handles = false;
        self.toolbar = false;
    }

    /// Show the handles and the toolbar, e.g. after selecting everything from the toolbar.
    pub fn show(&mut self) {
        self.handles = true;
        self.toolbar = true;
    }

    /// Show or hide the toolbar.
    pub fn set_toolbar_visible(&mut self, visible: bool) {
        self.toolbar = visible;
    }
}

/// A touch on an editable text, see [`crate::UseEditable::process_touch`].
#[derive(Debug)]
pub enum TouchEditEvent<'a> {
    /// A finger touched the text.
    Down { location: CursorPoint },
    /// A finger on the text, or on a handle, moved.
    Move {
        location: CursorPoint,
        editor_line: EditorLine,
        holder: &'a ParagraphHolder,
    },
    /// A finger lifted from the text or from a handle.
    Up {
        location: CursorPoint,
        editor_line: EditorLine,
        holder: &'a ParagraphHolder,
    },
    /// A finger stayed down for [`LONG_PRESS_TIMEOUT`] without moving.
    LongPress {
        location: CursorPoint,
        editor_line: EditorLine,
        holder: &'a ParagraphHolder,
    },
    /// A finger grabbed a handle. Its moves are then reported as [`TouchEditEvent::Move`], with
    /// the location of the text the handle points at.
    HandleDown(SelectionHandle),
    /// A finger touched the toolbar.
    ToolbarDown,
}

impl TouchEditEvent<'_> {
    pub fn process<T: TextEditor>(
        self,
        mut editor: Writable<T>,
        mut touch: Writable<TouchSelection>,
    ) {
        match self {
            TouchEditEvent::Down { location } => {
                touch.write().gesture = Gesture::Pressed { origin: location };
            }
            TouchEditEvent::Move {
                location,
                editor_line,
                holder,
            } => {
                let gesture = touch.peek().gesture.clone();
                match gesture {
                    Gesture::Pressed { origin } if origin.distance_to(location) > TOUCH_SLOP => {
                        touch.write().gesture = Gesture::Scrolling;
                    }
                    Gesture::SelectingWords { anchor } => {
                        let position = position_at(&*editor.peek(), location, editor_line, holder);
                        let (start, end) = editor.peek().find_word_boundaries(position);
                        let selection = if position < anchor.0 {
                            TextSelection::new_range((anchor.1, start))
                        } else {
                            TextSelection::new_range((anchor.0, end.max(anchor.1)))
                        };
                        set_selection(&mut editor, selection);
                    }
                    Gesture::DraggingHandle(handle) => {
                        let position = position_at(&*editor.peek(), location, editor_line, holder);
                        let selection = editor.peek().selection().clone();
                        // A selection made backwards starts at its `to` end.
                        let first = selection.start().min(selection.end());
                        let last = selection.start().max(selection.end());
                        let selection = match handle {
                            SelectionHandle::Cursor => TextSelection::new_cursor(position),
                            // An end never crosses the other one, the selection keeps at least
                            // one character.
                            SelectionHandle::Start => TextSelection::new_range((
                                last,
                                position.min(last.saturating_sub(1)),
                            )),
                            SelectionHandle::End => {
                                TextSelection::new_range((first, position.max(first + 1)))
                            }
                        };
                        set_selection(&mut editor, selection);
                    }
                    _ => {}
                }
            }
            TouchEditEvent::Up {
                location,
                editor_line,
                holder,
            } => {
                let gesture = std::mem::take(&mut touch.write().gesture);
                match gesture {
                    Gesture::Pressed { .. } => {
                        let position = position_at(&*editor.peek(), location, editor_line, holder);
                        let mut touch = touch.write();
                        let double_tap = touch.last_tap.is_some_and(|(at, last_location)| {
                            at.elapsed() <= DOUBLE_TAP_TIMEOUT
                                && last_location.distance_to(location) <= DOUBLE_TAP_SLOP
                        });
                        if double_tap {
                            touch.last_tap = None;
                            select_word(&mut editor, &mut touch, position);
                        } else {
                            touch.last_tap = Some((Instant::now(), location));
                            // Tapping the cursor again toggles the toolbar to paste.
                            let on_cursor = touch.handles
                                && editor.peek().get_selection().is_none()
                                && editor.peek().cursor_pos() == position;
                            touch.toolbar = on_cursor && !touch.toolbar;
                            touch.handles = true;
                            set_selection(&mut editor, TextSelection::new_cursor(position));
                        }
                    }
                    Gesture::SelectingWords { .. } | Gesture::DraggingHandle(_) => {
                        touch.write().toolbar = true;
                    }
                    Gesture::Idle | Gesture::Scrolling | Gesture::OnToolbar => {}
                }
            }
            TouchEditEvent::LongPress {
                location,
                editor_line,
                holder,
            } => {
                if !touch.peek().is_pressed() {
                    return;
                }
                let position = position_at(&*editor.peek(), location, editor_line, holder);
                let mut touch = touch.write();
                touch.last_tap = None;
                select_word(&mut editor, &mut touch, position);
                if let Some(anchor) = editor.peek().get_selection() {
                    // The toolbar shows once the finger lifts.
                    touch.toolbar = false;
                    touch.gesture = Gesture::SelectingWords { anchor };
                }
            }
            TouchEditEvent::ToolbarDown => {
                touch.write().gesture = Gesture::OnToolbar;
            }
            TouchEditEvent::HandleDown(handle) => {
                let mut touch = touch.write();
                touch.gesture = Gesture::DraggingHandle(handle);
                touch.toolbar = false;
            }
        }
    }
}

/// The position in the text at `location`, relative to the paragraph.
fn position_at<T: TextEditor>(
    editor: &T,
    location: CursorPoint,
    editor_line: EditorLine,
    holder: &ParagraphHolder,
) -> usize {
    let location = holder.visible_location(location);
    let holder = holder.0.borrow();
    let Some(ParagraphHolderInner {
        paragraph,
        scale_factor,
        ..
    }) = holder.as_ref()
    else {
        return editor.cursor_pos();
    };
    let index = paragraph.cursor_index_at_point(location.mul(*scale_factor).to_tuple());
    editor.measure_selection(index, editor_line).pos()
}

/// Select the word at `position`, or just move the cursor there when it is not on a word.
fn select_word<T: TextEditor>(
    editor: &mut Writable<T>,
    touch: &mut TouchSelection,
    position: usize,
) {
    let (start, end) = editor.peek().find_word_boundaries(position);
    let text = editor.peek().text().into_owned();
    let start_char = editor.peek().utf16_cu_to_char(start);
    let end_char = editor.peek().utf16_cu_to_char(end);
    let is_word = text
        .chars()
        .skip(start_char)
        .take(end_char - start_char)
        .any(|ch| !ch.is_whitespace());
    let selection = if is_word {
        TextSelection::new_range((start, end))
    } else {
        TextSelection::new_cursor(position)
    };
    touch.handles = true;
    touch.toolbar = true;
    set_selection(editor, selection);
}

fn set_selection<T: TextEditor>(editor: &mut Writable<T>, selection: TextSelection) {
    if *editor.peek().selection() != selection {
        *editor.write().selection_mut() = selection;
    }
}
