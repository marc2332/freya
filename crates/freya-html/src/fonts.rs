use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::Rc,
    sync::Arc,
};

use freya_core::prelude::{
    Bytes,
    GlobalContexts,
};
use freya_engine::prelude::Typeface;
use parley::fontique::{
    Blob,
    Collection,
    CollectionOptions,
    GenericFamily,
    SourceCache,
};

pub(crate) fn font_context(fonts: &[Bytes]) -> blitz_dom::FontContext {
    let mut context = match GlobalContexts::try_get() {
        Some(contexts) => contexts.get_context_or_insert(system_font_context),
        None => system_font_context(),
    };

    if fonts.is_empty() {
        return context;
    }

    let mut family_ids = Vec::new();
    for data in fonts {
        let decoded = blitz_dom::decode_font_bytes(data).into_owned();
        let registered = context
            .collection
            .register_fonts(Blob::new(Arc::new(decoded) as _), None);
        family_ids.extend(registered.into_iter().map(|(family_id, _)| family_id));
    }
    for generic in [
        GenericFamily::SansSerif,
        GenericFamily::Serif,
        GenericFamily::Monospace,
        GenericFamily::SystemUi,
    ] {
        context
            .collection
            .append_generic_families(generic, family_ids.iter().copied());
    }
    context
}

fn system_font_context() -> blitz_dom::FontContext {
    let mut context = blitz_dom::FontContext {
        source_cache: SourceCache::new_shared(),
        collection: Collection::new(CollectionOptions {
            shared: false,
            system_fonts: !cfg!(target_os = "emscripten"),
        }),
    };
    context
        .collection
        .register_fonts(Blob::new(Arc::new(blitz_dom::BULLET_FONT) as _), None);
    context
}

struct CachedTypeface {
    /// Keeps the source blob available across document lifetimes.
    font: peniko::FontData,
    normalized_coords: Vec<i16>,
    typeface: Typeface,
}

#[derive(Clone, Default)]
pub(crate) struct TypefaceCache {
    resources: Rc<RefCell<VecDeque<CachedTypeface>>>,
}

impl TypefaceCache {
    pub(crate) fn global() -> Self {
        match GlobalContexts::try_get() {
            Some(contexts) => contexts.get_context_or_insert(Self::default),
            None => Self::default(),
        }
    }

    pub(crate) fn get(
        &self,
        font: &peniko::FontData,
        normalized_coords: &[i16],
    ) -> Option<Typeface> {
        let mut resources = self.resources.borrow_mut();
        let index = resources.iter().position(|entry| {
            entry.font.index == font.index
                && entry.font.data.id() == font.data.id()
                && entry.normalized_coords == normalized_coords
        })?;
        let entry = resources.remove(index)?;
        let typeface = entry.typeface.clone();
        resources.push_back(entry);
        Some(typeface)
    }

    pub(crate) fn insert(
        &self,
        font: peniko::FontData,
        normalized_coords: &[i16],
        typeface: Typeface,
    ) {
        let mut resources = self.resources.borrow_mut();
        if resources.len() == 32 {
            resources.pop_front();
        }
        resources.push_back(CachedTypeface {
            font,
            normalized_coords: normalized_coords.to_vec(),
            typeface,
        });
    }
}
