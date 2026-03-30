use std::{
    any::Any,
    hash::{Hash, Hasher},
    rc::Rc,
};

use rustc_hash::FxHasher;

use crate::{diff_key::DiffKey, element::Element};

#[cfg(feature = "test")]
pub fn from_fn_captured<T: Fn() -> Element + 'static>(comp: T) -> Element {
    use std::rc::Rc;

    use crate::diff_key::DiffKey;

    Element::Component {
        key: DiffKey::None,
        #[cfg(feature = "hotreload")]
        comp: Rc::new(move |_| crate::hotreload::subsecond::HotFn::current(&comp).call(())),
        #[cfg(not(feature = "hotreload"))]
        comp: Rc::new(move |_| comp()),
        props: Rc::new(()),
    }
}

#[cfg(feature = "test")]
pub fn from_fn_standalone(comp: fn() -> Element) -> Element {
    Element::Component {
        key: comp.into(),
        #[cfg(feature = "hotreload")]
        comp: Rc::new(move |_| crate::hotreload::subsecond::HotFn::current(comp).call(())),
        #[cfg(not(feature = "hotreload"))]
        comp: Rc::new(move |_| comp()),
        props: Rc::new(()),
    }
}

#[cfg(feature = "test")]
pub fn from_fn_standalone_borrowed<P: 'static + PartialEq>(
    props: P,
    comp: fn(&P) -> Element,
) -> Element {
    Element::Component {
        key: comp.into(),
        #[cfg(feature = "hotreload")]
        comp: Rc::new(move |props| {
            let props = (&*props as &dyn Any).downcast_ref::<P>().unwrap();
            crate::hotreload::subsecond::HotFn::current(comp).call((props,))
        }),
        #[cfg(not(feature = "hotreload"))]
        comp: Rc::new(move |props| {
            let props = (&*props as &dyn Any).downcast_ref::<P>().unwrap();
            comp(props)
        }),
        props: Rc::new(props),
    }
}

/// Create a component instance from a given `Key`, `Props` and `Render` function.
pub fn from_fn<P: PartialEq + 'static>(
    key: impl Hash,
    props: P,
    comp: impl Fn(&P) -> Element + Clone + 'static,
) -> Element {
    let mut hasher = FxHasher::default();
    key.hash(&mut hasher);
    Element::Component {
        key: DiffKey::U64(hasher.finish()),
        #[cfg(feature = "hotreload")]
        comp: Rc::new(move |props| {
            let props = (&*props as &dyn Any).downcast_ref::<P>().unwrap();
            crate::hotreload::subsecond::HotFn::current(comp.clone()).call((props,))
        }),
        #[cfg(not(feature = "hotreload"))]
        comp: Rc::new(move |props| {
            let props = (&*props as &dyn Any).downcast_ref::<P>().unwrap();
            comp(props)
        }),
        props: Rc::new(props),
    }
}

/// Create a component instance from a given `Key`, `Props` and `Render` function. Similar to [from_fn] but instead gives owned props.
pub fn from_fn_owned<P: PartialEq + Clone + 'static>(
    key: impl Hash,
    props: P,
    comp: impl Fn(P) -> Element + Clone + 'static,
) -> Element {
    let mut hasher = FxHasher::default();
    key.hash(&mut hasher);
    Element::Component {
        key: DiffKey::U64(hasher.finish()),
        #[cfg(feature = "hotreload")]
        comp: Rc::new(move |props| {
            let props = (&*props as &dyn Any).downcast_ref::<P>().cloned().unwrap();
            crate::hotreload::subsecond::HotFn::current(comp.clone()).call((props,))
        }),
        #[cfg(not(feature = "hotreload"))]
        comp: Rc::new(move |props| {
            let props = (&*props as &dyn Any).downcast_ref::<P>().cloned().unwrap();
            comp(props)
        }),
        props: Rc::new(props),
    }
}

pub fn from_fn_standalone_borrowed_keyed<K: Hash, P: 'static + PartialEq>(
    key: K,
    props: P,
    comp: fn(&P) -> Element,
) -> Element {
    let mut hasher = FxHasher::default();
    key.hash(&mut hasher);
    Element::Component {
        key: DiffKey::U64(hasher.finish()),
        #[cfg(feature = "hotreload")]
        comp: Rc::new(move |props| {
            let props = (&*props as &dyn Any).downcast_ref::<P>().unwrap();
            crate::hotreload::subsecond::HotFn::current(comp).call((props,))
        }),
        #[cfg(not(feature = "hotreload"))]
        comp: Rc::new(move |props| {
            let props = (&*props as &dyn Any).downcast_ref::<P>().unwrap();
            comp(props)
        }),
        props: Rc::new(props),
    }
}
