use std::borrow::Cow;

use freya_engine::prelude::{
    FontCollection,
    FontMgr,
    SkData,
    TypefaceFontProvider,
};

/// Fonts available to the app, embedded at startup or loaded at runtime.
pub struct Fonts {
    provider: TypefaceFontProvider,
    pub manager: FontMgr,
    pub collection: FontCollection,
    pub default_families: Vec<Cow<'static, str>>,
}

impl Fonts {
    pub fn new(embedded: &[(String, Vec<u8>)], default_families: Vec<Cow<'static, str>>) -> Self {
        let provider = TypefaceFontProvider::new();
        let manager: FontMgr = provider.clone().into();

        let mut collection = FontCollection::new();
        collection.set_default_font_manager(manager.clone(), None);
        collection.set_dynamic_font_manager(manager.clone());
        collection.paragraph_cache_mut().turn_on(false);

        let mut fonts = Self {
            provider,
            manager,
            collection,
            default_families,
        };

        let mut embedded_families = Vec::new();
        for (name, data) in embedded {
            if fonts.register(name, data) {
                embedded_families.push(Cow::Owned(name.clone()));
            }
        }

        if fonts.default_families.is_empty() {
            fonts.default_families = embedded_families;
        }

        fonts
    }

    /// Registers a font loaded at runtime, dropping the caches measured without it.
    pub fn load(&mut self, name: &str, data: &[u8]) {
        self.register(name, data);
        self.collection.clear_caches();
    }

    fn register(&mut self, name: &str, data: &[u8]) -> bool {
        let Some(typeface) = FontMgr::default().new_from_data(SkData::new_copy(data), None) else {
            tracing::error!("Failed to load the font {name}.");
            return false;
        };

        self.provider.register_typeface(typeface, Some(name));
        true
    }
}
