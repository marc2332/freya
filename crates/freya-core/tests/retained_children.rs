use std::{
    cell::Cell,
    rc::Rc,
};

use freya_core::{
    element::ElementExt,
    integration::Runner,
    path_element::{
        PathChildren,
        PathElement,
    },
    prelude::*,
    tree::DiffModifies,
};

struct DiffProbe(Rc<Cell<usize>>);

impl ElementExt for DiffProbe {
    fn diff(&self, _previous: &Rc<dyn ElementExt>) -> DiffModifies {
        self.0.set(self.0.get() + 1);
        DiffModifies::empty()
    }
}

#[test]
fn ordinary_children_use_owned_storage() {
    let element = PathElement::from_element(vec![0], rect().into(), None);
    assert!(matches!(
        element,
        PathElement::Element {
            elements: PathChildren::Owned(children),
            ..
        } if children.is_empty()
    ));
}

#[test]
fn retained_children_reuse_requires_same_rc() {
    let diffs = Rc::new(Cell::new(0));
    let children = Rc::new(vec![Element::Element {
        key: DiffKey::None,
        element: Rc::new(DiffProbe(diffs.clone())),
        elements: Vec::new().into(),
    }]);
    let mut runner = Runner::new(|| {
        let children = use_consume::<State<Rc<Vec<Element>>>>();
        rect().retained_children(children.read().clone()).into()
    });
    let mut state = runner.provide_root_context(|| State::create(children.clone()));
    runner.sync_and_update();

    state.set(children.clone());
    runner.sync_and_update();
    assert_eq!(diffs.get(), 0);

    state.set(Rc::new(children.as_ref().clone()));
    runner.sync_and_update();
    assert_eq!(diffs.get(), 1);
}
