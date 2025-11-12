use std::{
    collections::VecDeque,
    rc::Rc,
};

use rustc_hash::FxHashMap;

use crate::{
    diff_key::DiffKey,
    element::{
        ComponentProps,
        Element,
        ElementExt,
    },
    runner::Diff,
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
                        let mut path = path.clone();
                        path.push(i as u32);
                        PathElement::from_element(path, e)
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
        match previous {
            None => {
                match self {
                    PathElement::Component { path, .. } => {
                        diff.added.push(path.clone());
                    }
                    PathElement::Element { path, elements, .. } => {
                        diff.added.push(path.clone());

                        // For Elements, recurse into children to mark them as added if needed
                        for element in elements {
                            element.diff(None, diff);
                        }
                    }
                }
            }
            Some(previous) => match (self, previous) {
                (
                    PathElement::Component { key: k1, path, .. },
                    PathElement::Component {
                        key: k2,
                        path: path2,
                        ..
                    },
                ) => {
                    if k1 != k2 || diff.removed.iter().any(|p| **p == path2[..path2.len() - 1]) {
                        diff.added.push(path.clone());
                        diff.removed.push(path2.clone());
                    } else if !path.is_empty() && path[path.len() - 1] != path2[path2.len() - 1] {
                        diff.moved
                            .entry(Box::from(path[..path.len() - 1].to_vec()))
                            .or_default()
                            .push((*path2.last().unwrap(), *path.last().unwrap()));
                    }
                }
                (
                    PathElement::Element {
                        elements: e1,
                        element: element1,
                        path,
                        key: k1,
                        ..
                    },
                    PathElement::Element {
                        elements: e2,
                        element: element2,
                        path: path2,
                        key: k2,
                        ..
                    },
                ) => {
                    if k1 != k2 || diff.removed.iter().any(|p| **p == path2[..path2.len() - 1]) {
                        diff.added.push(path.clone());
                        diff.removed.push(path2.clone());
                    } else {
                        let diff_flags = element1.diff(element2);
                        if !diff_flags.is_empty() {
                            diff.modified.push((path.clone(), diff_flags));
                        } else if !path.is_empty() && path[path.len() - 1] != path2[path2.len() - 1]
                        {
                            diff.moved
                                .entry(Box::from(path[..path.len() - 1].to_vec()))
                                .or_default()
                                .push((*path2.last().unwrap(), *path.last().unwrap()));
                        }
                    }

                    let mut previous_keys = FxHashMap::<&DiffKey, VecDeque<usize>>::default();

                    for (i, e) in e2.iter().enumerate() {
                        let (PathElement::Element { key, .. } | PathElement::Component { key, .. }) =
                            e;
                        previous_keys.entry(key).or_default().push_back(i)
                    }

                    for e in e1 {
                        let (PathElement::Element { key, .. } | PathElement::Component { key, .. }) =
                            e;
                        if let Some(old_i) =
                            previous_keys.get_mut(key).and_then(VecDeque::pop_front)
                        {
                            e.diff(Some(&e2[old_i]), diff);
                        } else {
                            e.diff(None, diff);
                        }
                    }

                    for indexes in previous_keys.values() {
                        for i in indexes {
                            let (PathElement::Element { path, .. }
                            | PathElement::Component { path, .. }) = &e2[*i];
                            // The same element might have mistakenly gotten marked as moved in a previous call
                            diff.moved.remove(path);
                            diff.removed.push(path.clone());
                            // No need to remove recursively here because scope and tree diff handling already runs recursively
                        }
                    }
                }
                (s, o) => {
                    // Removed old
                    let (PathElement::Element { path, .. } | PathElement::Component { path, .. }) =
                        o;
                    diff.removed.push(path.clone());

                    // Add new recursively
                    s.diff(None, diff);
                }
            },
        }
    }
}
