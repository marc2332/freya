use std::rc::Rc;

#[cfg(debug_assertions)]
use rustc_hash::FxHashMap;
use tiny_diff::DiffNode;

use crate::{
    diff_key::DiffKey,
    element::{
        ComponentProps,
        Element,
        ElementExt,
    },
    runner::Diff,
    tree::DiffModifies,
};

pub enum PathElement {
    Component {
        key: DiffKey,

        comp: Rc<dyn Fn(Rc<dyn ComponentProps>) -> Element>,

        props: Rc<dyn ComponentProps>,

        path: Box<[u32]>,
    },
    Element {
        key: DiffKey,

        element: Rc<dyn ElementExt>,
        elements: Box<[PathElement]>,
        path: Box<[u32]>,
    },
}

impl std::fmt::Debug for PathElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathElement::Component { key, path, .. } => f
                .debug_struct("Component")
                .field("key", key)
                .field("path", path)
                .field("comp", &"<fn>")
                .field("props", &"<props>")
                .finish(),
            PathElement::Element { elements, path, .. } => f
                .debug_struct("Element")
                .field("path", path)
                .field("elements", elements)
                .field("element", &"<element>")
                .finish(),
        }
    }
}

impl PathElement {
    #[inline(always)]
    pub fn with_element<D>(&self, target_path: &[u32], with: D)
    where
        D: FnOnce(&PathElement),
    {
        match self {
            Self::Component { path, .. } | Self::Element { path, .. }
                if path.as_ref() == target_path =>
            {
                with(self);
            }
            Self::Element { elements, path, .. } if target_path.starts_with(path) => {
                let next_step = target_path[path.len()];
                elements[next_step as usize].with_element(target_path, with);
            }
            _ => {}
        }
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn from_element(path: Vec<u32>, element: Element) -> Self {
        match element {
            Element::Component { key, comp, props } => PathElement::Component {
                key,
                comp,
                props,
                path: path.into_boxed_slice(),
            },
            Element::Element {
                elements,
                element,
                key,
            } => PathElement::Element {
                elements: elements
                    .into_iter()
                    .enumerate()
                    .map(|(i, e)| {
                        let mut child_path = Vec::with_capacity(path.len() + 1);
                        child_path.extend_from_slice(&path);
                        child_path.push(i as u32);
                        PathElement::from_element(child_path, e)
                    })
                    .collect::<Box<[PathElement]>>(),
                path: path.into_boxed_slice(),
                element,
                key,
            },
        }
    }

    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn diff(&self, previous: Option<&Self>, diff: &mut Diff) {
        tiny_diff::diff(self, previous, diff);
    }
}

impl DiffNode for PathElement {
    type Key = DiffKey;
    type Modifications = DiffModifies;

    fn key(&self) -> &DiffKey {
        let (Self::Element { key, .. } | Self::Component { key, .. }) = self;
        key
    }

    fn path(&self) -> &[u32] {
        let (Self::Element { path, .. } | Self::Component { path, .. }) = self;
        path
    }

    fn children(&self) -> &[Self] {
        match self {
            Self::Element { elements, .. } => elements,
            Self::Component { .. } => &[],
        }
    }

    fn compatible(&self, previous: &Self) -> bool {
        matches!(
            (self, previous),
            (Self::Element { .. }, Self::Element { .. })
                | (Self::Component { .. }, Self::Component { .. })
        )
    }

    fn modifications(&self, previous: &Self) -> Option<DiffModifies> {
        match (self, previous) {
            (
                Self::Element { element, .. },
                Self::Element {
                    element: previous, ..
                },
            ) => {
                let flags = element.diff(previous);
                (!flags.is_empty()).then_some(flags)
            }
            _ => None,
        }
    }

    #[cfg(debug_assertions)]
    fn validate_children(&self) {
        let mut seen = FxHashMap::<&DiffKey, &[u32]>::default();
        for child in self.children() {
            let key = child.key();
            if matches!(key, DiffKey::None | DiffKey::DefaultU64(_)) {
                continue;
            }
            if let Some(previous) = seen.insert(key, child.path()) {
                panic!(
                    "freya diff: duplicate sibling key {:?} under parent {:?} (paths {:?} and {:?})",
                    key,
                    self.path(),
                    previous,
                    child.path(),
                );
            }
        }
    }
}
