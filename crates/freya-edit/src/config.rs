use std::sync::LazyLock;

use keyboard_types::{
    Key,
    Modifiers,
    NamedKey,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct EditableConfig {
    pub(crate) indentation: u8,
    pub(crate) allow_tabs: bool,
    pub(crate) allow_changes: bool,
    pub(crate) allow_read_clipboard: bool,
    pub(crate) allow_write_clipboard: bool,
    pub(crate) select_all_on_double_click: bool,
    pub(crate) select_all_on_init: bool,
}

impl Default for EditableConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl EditableConfig {
    /// Create a [`EditableConfig`].
    pub fn new() -> Self {
        Self {
            indentation: 4,
            allow_tabs: false,
            allow_changes: true,
            allow_read_clipboard: true,
            allow_write_clipboard: true,
            select_all_on_double_click: false,
            select_all_on_init: false,
        }
    }

    /// Specify a custom indentation
    pub fn with_indentation(mut self, indentation: u8) -> Self {
        self.indentation = indentation;
        self
    }

    /// Specify whether you want to allow tabs to be inserted
    pub fn with_allow_tabs(mut self, allow_tabs: bool) -> Self {
        self.allow_tabs = allow_tabs;
        self
    }

    /// Allow changes through keyboard events or not
    pub fn with_allow_changes(mut self, allow_changes: bool) -> Self {
        self.allow_changes = allow_changes;
        self
    }

    /// Allow reading from the clipboard (paste).
    pub fn with_allow_read_clipboard(mut self, allow_read_clipboard: bool) -> Self {
        self.allow_read_clipboard = allow_read_clipboard;
        self
    }

    /// Allow writing to the clipboard (copy and cut).
    pub fn with_allow_write_clipboard(mut self, allow_write_clipboard: bool) -> Self {
        self.allow_write_clipboard = allow_write_clipboard;
        self
    }

    /// Make a double click select the whole text instead of a single word,
    /// behaving like a triple click. Useful for masked inputs.
    pub fn with_select_all_on_double_click(mut self, select_all_on_double_click: bool) -> Self {
        self.select_all_on_double_click = select_all_on_double_click;
        self
    }

    /// Start with the initial content selected instead of the cursor at its beginning.
    ///
    /// For an editor that opens over text the user is about to replace — a rename affordance
    /// seeded with the current name — this is what makes the first keystroke replace it rather
    /// than prepend to it. Without it the cursor sits at position 0, so typing lands in front of
    /// the very text being renamed.
    pub fn with_select_all_on_init(mut self, select_all_on_init: bool) -> Self {
        self.select_all_on_init = select_all_on_init;
        self
    }
}

/// The key of an [`EditChord`]: a character (matched case-insensitively, so a chord
/// holding `z` also matches the shifted `Z`) or a named key.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChordKey {
    Character(char),
    Named(NamedKey),
}

/// One keyboard chord for a text-editing action. `primary` matches Meta or Control,
/// so a chord behaves the same across platforms.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EditChord {
    pub primary: bool,
    pub shift: bool,
    pub alt: bool,
    pub key: ChordKey,
}

impl EditChord {
    /// Primary (Meta/Control) + `key`.
    pub const fn primary(key: char) -> Self {
        Self {
            primary: true,
            shift: false,
            alt: false,
            key: ChordKey::Character(key),
        }
    }

    /// Primary (Meta/Control) + Shift + `key`.
    pub const fn primary_shift(key: char) -> Self {
        Self {
            primary: true,
            shift: true,
            alt: false,
            key: ChordKey::Character(key),
        }
    }

    /// Whether the pressed `key` + `modifiers` hit this chord. Modifiers are matched
    /// exactly, so a chord without Alt does not fire while Alt is held.
    pub fn matches(&self, key: &Key, modifiers: &Modifiers) -> bool {
        let key_matches = match (self.key, key) {
            (ChordKey::Character(expected), Key::Character(actual)) => {
                let mut chars = actual.chars();
                match (chars.next(), chars.next()) {
                    (Some(ch), None) => ch.to_lowercase().eq(expected.to_lowercase()),
                    _ => false,
                }
            }
            (ChordKey::Named(expected), Key::Named(actual)) => expected == *actual,
            _ => false,
        };
        key_matches
            && self.primary == modifiers.intersects(Modifiers::META | Modifiers::CONTROL)
            && self.shift == modifiers.contains(Modifiers::SHIFT)
            && self.alt == modifiers.contains(Modifiers::ALT)
    }
}

/// A text-editing action that [`TextEditor::process_key`](crate::TextEditor::process_key)
/// triggers through a rebindable chord (see [`EditBindings`]).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EditAction {
    SelectAll,
    Copy,
    Cut,
    Paste,
    Undo,
    Redo,
}

/// The chords a text editor's [`EditAction`]s respond to in
/// [`TextEditor::process_key`](crate::TextEditor::process_key). Editors expose them via
/// [`TextEditor::edit_bindings`](crate::TextEditor::edit_bindings); implementors like
/// [`RopeEditor`](crate::RopeEditor) let callers replace them at runtime, e.g. to drive
/// the chords from user configurable shortcuts. An empty list disables that action's
/// shortcut.
#[derive(Clone, PartialEq, Debug)]
pub struct EditBindings {
    pub select_all: Vec<EditChord>,
    pub copy: Vec<EditChord>,
    pub cut: Vec<EditChord>,
    pub paste: Vec<EditChord>,
    pub undo: Vec<EditChord>,
    pub redo: Vec<EditChord>,
}

impl Default for EditBindings {
    /// The platform conventions: primary+A/C/X/V select/copy/cut/paste; primary+Z
    /// undoes; primary+Shift+Z and primary+Y redo.
    fn default() -> Self {
        Self {
            select_all: vec![EditChord::primary('a')],
            copy: vec![EditChord::primary('c')],
            cut: vec![EditChord::primary('x')],
            paste: vec![EditChord::primary('v')],
            undo: vec![EditChord::primary('z')],
            redo: vec![EditChord::primary_shift('z'), EditChord::primary('y')],
        }
    }
}

impl EditBindings {
    /// The default bindings as a static, backing [`TextEditor::edit_bindings`]'s
    /// default body.
    pub fn default_ref() -> &'static Self {
        static DEFAULT: LazyLock<EditBindings> = LazyLock::new(EditBindings::default);
        &DEFAULT
    }

    /// The action whose chords the pressed `key` + `modifiers` hit, if any. Actions are
    /// checked in a fixed order (select all, copy, cut, paste, undo, redo) so duplicate
    /// chords resolve deterministically.
    pub fn resolve(&self, key: &Key, modifiers: &Modifiers) -> Option<EditAction> {
        let actions = [
            (EditAction::SelectAll, &self.select_all),
            (EditAction::Copy, &self.copy),
            (EditAction::Cut, &self.cut),
            (EditAction::Paste, &self.paste),
            (EditAction::Undo, &self.undo),
            (EditAction::Redo, &self.redo),
        ];
        actions
            .into_iter()
            .find(|(_, chords)| chords.iter().any(|chord| chord.matches(key, modifiers)))
            .map(|(action, _)| action)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chord_matching() {
        let undo = EditChord::primary('z');
        assert!(undo.matches(&Key::Character("z".into()), &Modifiers::META));
        assert!(undo.matches(&Key::Character("z".into()), &Modifiers::CONTROL));
        // No primary held, or extra modifiers held: no match.
        assert!(!undo.matches(&Key::Character("z".into()), &Modifiers::empty()));
        assert!(!undo.matches(
            &Key::Character("z".into()),
            &(Modifiers::META | Modifiers::ALT)
        ));
        assert!(!undo.matches(
            &Key::Character("Z".into()),
            &(Modifiers::META | Modifiers::SHIFT)
        ));

        // Shifted characters arrive uppercased; the chord still matches.
        let redo = EditChord::primary_shift('z');
        assert!(redo.matches(
            &Key::Character("Z".into()),
            &(Modifiers::META | Modifiers::SHIFT)
        ));
        assert!(!redo.matches(&Key::Character("z".into()), &Modifiers::META));

        let named = EditChord {
            primary: true,
            shift: false,
            alt: false,
            key: ChordKey::Named(NamedKey::Enter),
        };
        assert!(named.matches(&Key::Named(NamedKey::Enter), &Modifiers::META));
        assert!(!named.matches(&Key::Named(NamedKey::Enter), &Modifiers::empty()));
    }

    #[test]
    fn bindings_resolve_actions() {
        let bindings = EditBindings::default();
        assert_eq!(
            bindings.resolve(&Key::Character("a".into()), &Modifiers::META),
            Some(EditAction::SelectAll)
        );
        assert_eq!(
            bindings.resolve(&Key::Character("v".into()), &Modifiers::CONTROL),
            Some(EditAction::Paste)
        );
        assert_eq!(
            bindings.resolve(&Key::Character("z".into()), &Modifiers::META),
            Some(EditAction::Undo)
        );
        assert_eq!(
            bindings.resolve(
                &Key::Character("Z".into()),
                &(Modifiers::META | Modifiers::SHIFT)
            ),
            Some(EditAction::Redo)
        );
        assert_eq!(
            bindings.resolve(&Key::Character("y".into()), &Modifiers::META),
            Some(EditAction::Redo)
        );
        // Plain typing and unbound chords resolve to nothing.
        assert_eq!(
            bindings.resolve(&Key::Character("a".into()), &Modifiers::empty()),
            None
        );
        assert_eq!(
            bindings.resolve(&Key::Character("t".into()), &Modifiers::META),
            None
        );

        // Rebinding is honored, and an emptied list disables the action.
        let rebound = EditBindings {
            undo: vec![EditChord::primary('u')],
            redo: vec![],
            ..EditBindings::default()
        };
        assert_eq!(
            rebound.resolve(&Key::Character("u".into()), &Modifiers::META),
            Some(EditAction::Undo)
        );
        assert_eq!(
            rebound.resolve(&Key::Character("z".into()), &Modifiers::META),
            None
        );
        assert_eq!(
            rebound.resolve(&Key::Character("y".into()), &Modifiers::META),
            None
        );
    }
}
