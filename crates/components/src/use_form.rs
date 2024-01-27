use crate::{ButtonProps, InputMode, InputProps};
use dioxus::prelude::*;
use std::{collections::HashMap, hash::Hash};

#[derive(Clone)]
pub struct UseForm<Id: Hash + Eq + 'static> {
    data: Signal<HashMap<Id, String>>,
    onsubmit: Signal<Box<dyn Fn(&HashMap<Id, String>)>>,
}

impl<Id: Clone + Hash + Eq> UseForm<Id> {
    pub fn register(&self, id: Id) -> InputProps {
        let value = self.data.read().get(&id).cloned().unwrap_or_default();
        let data = self.data;
        InputProps {
            onchange: EventHandler::new(move |txt| {
                data.write().insert(id.clone(), txt);
            }),
            theme: None,
            mode: InputMode::default(),
            value,
        }
    }

    pub fn submit(&self, children: Element) -> ButtonProps {
        let submit = self.onsubmit;
        let data = self.data;
        ButtonProps {
            theme: None,
            onclick: Some(EventHandler::new(move |_| {
                (submit.peek())(&data.read());
            })),
            children,
        }
    }
}

pub fn use_form<Id: Hash + Eq + Clone>(
    onsubmit: impl Fn(&HashMap<Id, String>) + 'static,
) -> UseForm<Id> {
    let form = use_hook(|| UseForm {
        data: Signal::new(HashMap::default()),
        onsubmit: Signal::new(Box::new(onsubmit)),
    });

    form
}
