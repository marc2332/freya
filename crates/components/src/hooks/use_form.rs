// This hook is in freya-components instead of freya-hooks because it uses some component props.

use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
};

use dioxus::prelude::*;

use crate::{
    ButtonProps,
    InputMode,
    InputProps,
};

type SubmitCallback<Id> = Box<dyn Fn(&HashMap<Id, String>)>;

/// Form controller
///
/// Use [`Self::input()`] to register inputs
/// And [`Self::submit()`] to register a submitter button
#[derive(Clone)]
pub struct UseForm<Id: Hash + Eq + 'static> {
    data: Signal<HashMap<Id, String>>,
    onsubmit: Signal<SubmitCallback<Id>>,
}

impl<Id: Clone + Hash + Eq + Display> UseForm<Id> {
    /// Register an Input component
    pub fn input(&self, id: Id) -> InputProps {
        let value = self.data.read().get(&id).cloned().unwrap_or_default();
        let placeholder = id.to_string();
        let mut data = self.data;
        InputProps {
            onchange: EventHandler::new(move |txt| {
                data.write().insert(id.clone(), txt);
            }),
            theme: None,
            mode: InputMode::default(),
            value,
            placeholder: Some(placeholder),
            auto_focus: false,
            onvalidate: None,
        }
    }

    /// Register a Button component
    pub fn submit(&self) -> ButtonProps {
        let submit = self.onsubmit;
        let data = self.data;
        ButtonProps {
            theme: None,
            onpress: Some(EventHandler::new(move |_| {
                (submit.peek())(&data.read());
            })),
            children: Ok(VNode::placeholder()),
            onclick: None,
        }
    }
}

/// Create a Form controller with a submit callback.
pub fn use_form<Id: Hash + Eq + Clone>(
    onsubmit: impl Fn(&HashMap<Id, String>) + 'static,
) -> UseForm<Id> {
    use_hook(|| UseForm {
        data: Signal::new(HashMap::default()),
        onsubmit: Signal::new(Box::new(onsubmit)),
    })
}
