use std::{
    any::{
        Any,
        TypeId,
    },
    rc::Rc,
};

use generational_box::{
    AnyStorage,
    GenerationalBox,
    Owner,
    UnsyncStorage,
};
use pathmap::PathMap;
use rustc_hash::FxHashMap;

use crate::{
    diff_key::DiffKey,
    element::{
        ComponentProps,
        Element,
    },
    node_id::NodeId,
    path_element::PathElement,
    reactive_context::ReactiveContext,
    scope_id::ScopeId,
};

pub struct Scope {
    pub id: ScopeId,
    pub parent_id: Option<ScopeId>,
    pub height: usize,

    pub parent_node_id_in_parent: NodeId,
    pub path_in_parent: Box<[u32]>, // TODO: Maybe just store the index rather than the whole path?

    pub nodes: PathMap<PathNode>,

    pub key: DiffKey,

    pub comp: Rc<dyn Fn(Rc<dyn ComponentProps>) -> Element>,

    pub props: Rc<dyn ComponentProps>,

    pub element: Option<PathElement>,
}

impl Scope {
    pub fn with_element<D>(&self, target_path: &[u32], with: D)
    where
        D: FnOnce(&PathElement),
    {
        if let Some(element) = &self.element {
            element.with_element(target_path, with);
        }
    }
}

#[derive(Clone)]
pub struct ScopeStorage {
    pub parent_id: Option<ScopeId>,
    pub current_run: usize,
    pub current_value: usize,
    pub values: Vec<Rc<dyn Any>>,

    pub contexts: FxHashMap<TypeId, Rc<dyn Any>>,

    pub reactive_context: ReactiveContext,

    pub owner: Owner,
}

impl ScopeStorage {
    pub(crate) fn new<T: 'static>(
        parent_id: Option<ScopeId>,
        reactive_context: impl FnOnce(&dyn Fn(T) -> GenerationalBox<T>) -> ReactiveContext,
    ) -> Self {
        let owner = UnsyncStorage::owner();
        let reactive_context = (reactive_context)(&|d: T| owner.insert(d));
        Self {
            parent_id,
            current_run: Default::default(),
            current_value: Default::default(),
            values: Default::default(),
            contexts: Default::default(),
            reactive_context,
            owner,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.current_run = 0;
        self.current_value = 0;
        self.values.clear();
        self.contexts.clear();
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathNode {
    pub node_id: NodeId,
    pub scope_id: Option<ScopeId>,
}
