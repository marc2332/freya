use std::{
    collections::HashMap,
    path::PathBuf,
};

use fluent::{
    FluentArgs,
    FluentBundle,
    FluentResource,
};
use freya_core::prelude::*;
use unic_langid::LanguageIdentifier;

use crate::error::Error;

/// `Locale` is a "place-holder" around what will eventually be a `fluent::FluentBundle`
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Locale {
    id: LanguageIdentifier,
    resource: LocaleResource,
}

impl Locale {
    /// Create a [`Locale`] from a static string containing Fluent (`.ftl`) content.
    ///
    /// Typically used with [`include_str!`] to embed locale files at compile time.
    pub fn new_static(id: LanguageIdentifier, str: &'static str) -> Self {
        Self {
            id,
            resource: LocaleResource::Static(str),
        }
    }

    /// Create a [`Locale`] from a filesystem path to a Fluent (`.ftl`) file.
    ///
    /// The file will be read at runtime when the locale is loaded into a bundle.
    pub fn new_dynamic(id: LanguageIdentifier, path: impl Into<PathBuf>) -> Self {
        Self {
            id,
            resource: LocaleResource::Path(path.into()),
        }
    }
}

impl<T> From<(LanguageIdentifier, T)> for Locale
where
    T: Into<LocaleResource>,
{
    fn from((id, resource): (LanguageIdentifier, T)) -> Self {
        let resource = resource.into();
        Self { id, resource }
    }
}

/// A `LocaleResource` can be static text, or a filesystem file.
#[derive(Debug, PartialEq)]
pub enum LocaleResource {
    Static(&'static str),

    Path(PathBuf),
}

impl LocaleResource {
    pub fn try_to_resource_string(&self) -> Result<String, Error> {
        match self {
            Self::Static(str) => Ok(str.to_string()),

            Self::Path(path) => std::fs::read_to_string(path)
                .map_err(|e| Error::LocaleResourcePathReadFailed(e.to_string())),
        }
    }

    pub fn to_resource_string(&self) -> String {
        let result = self.try_to_resource_string();
        match result {
            Ok(string) => string,
            Err(err) => panic!("failed to create resource string {self:?}: {err}"),
        }
    }
}

impl From<&'static str> for LocaleResource {
    fn from(value: &'static str) -> Self {
        Self::Static(value)
    }
}

impl From<PathBuf> for LocaleResource {
    fn from(value: PathBuf) -> Self {
        Self::Path(value)
    }
}

/// The configuration for `I18n`.
#[derive(Debug, PartialEq)]
pub struct I18nConfig {
    /// The initial language, can be later changed with [`I18n::set_language`]
    pub id: LanguageIdentifier,

    /// The final fallback language if no other locales are found for `id`.
    /// A `Locale` must exist in `locales' if `fallback` is defined.
    pub fallback: Option<LanguageIdentifier>,

    /// The locale_resources added to the configuration.
    pub locale_resources: Vec<LocaleResource>,

    /// The locales added to the configuration.
    pub locales: HashMap<LanguageIdentifier, usize>,
}

impl I18nConfig {
    /// Create an i18n config with the selected [LanguageIdentifier].
    pub fn new(id: LanguageIdentifier) -> Self {
        Self {
            id,
            fallback: None,
            locale_resources: Vec::new(),
            locales: HashMap::new(),
        }
    }

    /// Set a fallback [LanguageIdentifier].
    pub fn with_fallback(mut self, fallback: LanguageIdentifier) -> Self {
        self.fallback = Some(fallback);
        self
    }

    /// Add [Locale].
    /// It is possible to share locales resources. If this locale's resource
    /// matches a previously added one, then this locale will use the existing one.
    /// This is primarily for the static locale_resources to avoid string duplication.
    pub fn with_locale<T>(mut self, locale: T) -> Self
    where
        T: Into<Locale>,
    {
        let locale = locale.into();
        let locale_resources_len = self.locale_resources.len();

        let index = self
            .locale_resources
            .iter()
            .position(|r| *r == locale.resource)
            .unwrap_or(locale_resources_len);

        if index == locale_resources_len {
            self.locale_resources.push(locale.resource)
        };

        self.locales.insert(locale.id, index);
        self
    }

    /// Add multiple locales from given folder, based on their filename.
    ///
    /// If the path represents a folder, then the folder will be deep traversed for
    /// all '*.ftl' files. If the filename represents a [LanguageIdentifier] then it
    ///  will be added to the config.
    ///
    /// If the path represents a file, then the filename must represent a
    /// unic_langid::LanguageIdentifier for it to be added to the config.
    #[cfg(feature = "discovery")]
    pub fn try_with_auto_locales(self, path: PathBuf) -> Result<Self, Error> {
        if path.is_dir() {
            let files = find_ftl_files(&path)?;
            files
                .into_iter()
                .try_fold(self, |acc, file| acc.with_auto_pathbuf(file))
        } else if is_ftl_file(&path) {
            self.with_auto_pathbuf(path)
        } else {
            Err(Error::InvalidPath(path.to_string_lossy().to_string()))
        }
    }

    #[cfg(feature = "discovery")]
    fn with_auto_pathbuf(self, file: PathBuf) -> Result<Self, Error> {
        assert!(is_ftl_file(&file));

        let stem = file.file_stem().ok_or_else(|| {
            Error::InvalidLanguageId(format!("No file stem: '{}'", file.display()))
        })?;

        let id_str = stem.to_str().ok_or_else(|| {
            Error::InvalidLanguageId(format!("Cannot convert: {}", stem.to_string_lossy()))
        })?;

        let id = LanguageIdentifier::from_bytes(id_str.as_bytes())
            .map_err(|e| Error::InvalidLanguageId(e.to_string()))?;

        Ok(self.with_locale((id, file)))
    }

    /// Add multiple locales from given folder, based on their filename.
    ///
    /// Will panic! on error.
    #[cfg(feature = "discovery")]
    pub fn with_auto_locales(self, path: PathBuf) -> Self {
        let path_name = path.display().to_string();
        let result = self.try_with_auto_locales(path);
        match result {
            Ok(result) => result,
            Err(err) => panic!("with_auto_locales must have valid pathbuf {path_name}: {err}",),
        }
    }
}

#[cfg(feature = "discovery")]
fn find_ftl_files(folder: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    let ftl_files: Vec<PathBuf> = walkdir::WalkDir::new(folder)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| is_ftl_file(entry.path()))
        .map(|entry| entry.path().to_path_buf())
        .collect();

    Ok(ftl_files)
}

#[cfg(feature = "discovery")]
fn is_ftl_file(entry: &std::path::Path) -> bool {
    entry.is_file() && entry.extension().map(|ext| ext == "ftl").unwrap_or(false)
}

/// Provide an existing [`I18n`] instance to descendant components.
///
/// This is useful for sharing the same global i18n state across different parts of the
/// component tree or across multiple windows. Typically paired with [`I18n::create_global`].
///
/// ```rust,no_run
/// # use freya::prelude::*;
/// # use freya::i18n::*;
/// struct MyApp {
///     i18n: I18n,
/// }
///
/// impl App for MyApp {
///     fn render(&self) -> impl IntoElement {
///         // Make the I18n instance available to all descendant components
///         use_share_i18n(move || self.i18n);
///
///         rect().child(t!("hello_world"))
///     }
/// }
/// ```
pub fn use_share_i18n(i18n: impl FnOnce() -> I18n) {
    use_provide_context(i18n);
}

/// Initialize an [`I18n`] instance and provide it to descendant components.
///
/// This is the recommended way to set up i18n in your application. Call it once in a root
/// component and then use [`I18n::get`] (or the `t!`, `te!`, `tid!` macros) in any
/// descendant component to access translations.
///
/// See [`I18n::create`] for a manual initialization where you can also handle errors.
///
/// # Panics
///
/// Panics if the [`I18nConfig`] fails to produce a valid Fluent bundle.
///
/// ```rust
/// # use freya::prelude::*;
/// # use freya::i18n::*;
/// fn app() -> impl IntoElement {
///     let mut i18n = use_init_i18n(|| {
///         I18nConfig::new(langid!("en-US"))
///             .with_locale(Locale::new_static(
///                 langid!("en-US"),
///                 include_str!("../../../examples/i18n/en-US.ftl"),
///             ))
///             .with_locale(Locale::new_dynamic(
///                 langid!("es-ES"),
///                 "../../../examples/i18n/es-ES.ftl",
///             ))
///     });
///
///     let change_to_spanish = move |_| i18n.set_language(langid!("es-ES"));
///
///     rect()
///         .child(t!("hello_world"))
///         .child(Button::new().on_press(change_to_spanish).child("Spanish"))
/// }
/// ```
pub fn use_init_i18n(init: impl FnOnce() -> I18nConfig) -> I18n {
    use_provide_context(move || {
        // Coverage false -ve: See https://github.com/xd009642/tarpaulin/issues/1675
        match I18n::create(init()) {
            Ok(i18n) => i18n,
            Err(e) => panic!("Failed to create I18n context: {e}"),
        }
    })
}

/// The main handle for accessing and managing internationalization state.
///
/// `I18n` holds the selected language, fallback language, locale resources, and the active
/// [Fluent](https://projectfluent.org/) bundle used for translating messages. It is `Clone + Copy`
/// so it can be freely passed around in components.
///
/// There are several ways to obtain an `I18n` instance:
/// - [`use_init_i18n`] to create and provide it to descendant components.
/// - [`I18n::create`] to manually create one from an [`I18nConfig`] (useful when you need to handle errors).
/// - [`I18n::create_global`] to create one with global lifetime, suitable for multi-window apps.
/// - [`I18n::get`] or [`I18n::try_get`] to retrieve an already-provided instance from the component context.
/// - [`use_share_i18n`] to re-provide an existing instance to a different part of the component tree.
#[derive(Clone, Copy)]
pub struct I18n {
    selected_language: State<LanguageIdentifier>,
    fallback_language: State<Option<LanguageIdentifier>>,
    locale_resources: State<Vec<LocaleResource>>,
    locales: State<HashMap<LanguageIdentifier, usize>>,
    active_bundle: State<FluentBundle<FluentResource>>,
}

impl I18n {
    /// Try to retrieve the [`I18n`] instance from the component context.
    ///
    /// Returns `None` if no [`I18n`] has been provided via [`use_init_i18n`] or [`use_share_i18n`]
    /// in an ancestor component.
    pub fn try_get() -> Option<Self> {
        try_consume_context()
    }

    /// Retrieve the [`I18n`] instance from the component context.
    ///
    /// This is the primary way to access the i18n state from within a component that is a
    /// descendant of a component that called [`use_init_i18n`] or [`use_share_i18n`].
    ///
    /// # Panics
    ///
    /// Panics if no [`I18n`] has been provided in an ancestor component.
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// # use freya::i18n::*;
    /// #[derive(PartialEq)]
    /// struct MyComponent;
    ///
    /// impl Component for MyComponent {
    ///     fn render(&self) -> impl IntoElement {
    ///         let mut i18n = I18n::get();
    ///
    ///         let change_to_english = move |_| i18n.set_language(langid!("en-US"));
    ///
    ///         rect()
    ///             .child(t!("hello_world"))
    ///             .child(Button::new().on_press(change_to_english).child("English"))
    ///     }
    /// }
    /// ```
    pub fn get() -> Self {
        consume_context()
    }

    /// Manually create an [`I18n`] instance from an [`I18nConfig`].
    ///
    /// Unlike [`use_init_i18n`], this does not automatically provide the instance to descendant
    /// components. Use [`use_share_i18n`] to share it afterwards, or call this when you need
    /// explicit error handling during initialization.
    ///
    /// The created state is scoped to the current component. For global state that outlives
    /// any single component, see [`I18n::create_global`].
    pub fn create(
        I18nConfig {
            id: selected_language,
            fallback: fallback_language,
            locale_resources,
            locales,
        }: I18nConfig,
    ) -> Result<Self, Error> {
        let bundle = try_create_bundle(
            &selected_language,
            &fallback_language,
            &locale_resources,
            &locales,
        )?;
        Ok(Self {
            selected_language: State::create(selected_language),
            fallback_language: State::create(fallback_language),
            locale_resources: State::create(locale_resources),
            locales: State::create(locales),
            active_bundle: State::create(bundle),
        })
    }

    /// Create an [`I18n`] instance with global lifetime.
    ///
    /// Unlike [`I18n::create`], the state created here is not scoped to any component and will
    /// live for the entire duration of the application. This is useful for multi-window apps
    /// where i18n state needs to be created in `main` and then shared across different windows
    /// via [`use_share_i18n`].
    ///
    /// ```rust,no_run
    /// # use freya::prelude::*;
    /// # use freya::i18n::*;
    /// struct MyApp {
    ///     i18n: I18n,
    /// }
    ///
    /// impl App for MyApp {
    ///     fn render(&self) -> impl IntoElement {
    ///         // Re-provide the global I18n to this window's component tree
    ///         use_share_i18n(move || self.i18n);
    ///
    ///         rect().child(t!("hello_world"))
    ///     }
    /// }
    ///
    /// fn main() {
    ///     // Create I18n with global lifetime in main, before any window is opened
    ///     let i18n = I18n::create_global(I18nConfig::new(langid!("en-US")).with_locale((
    ///         langid!("en-US"),
    ///         include_str!("../../../examples/i18n/en-US.ftl"),
    ///     )))
    ///     .expect("Failed to create I18n");
    ///
    ///     // Pass it to each window's app struct
    ///     launch(LaunchConfig::new().with_window(WindowConfig::new_app(MyApp { i18n })))
    /// }
    /// ```
    pub fn create_global(
        I18nConfig {
            id: selected_language,
            fallback: fallback_language,
            locale_resources,
            locales,
        }: I18nConfig,
    ) -> Result<Self, Error> {
        let bundle = try_create_bundle(
            &selected_language,
            &fallback_language,
            &locale_resources,
            &locales,
        )?;
        Ok(Self {
            selected_language: State::create_global(selected_language),
            fallback_language: State::create_global(fallback_language),
            locale_resources: State::create_global(locale_resources),
            locales: State::create_global(locales),
            active_bundle: State::create_global(bundle),
        })
    }

    /// Translate a message by its identifier, optionally with Fluent arguments.
    ///
    /// The `msg` can be a simple message id (e.g. `"hello"`) or a dotted attribute
    /// id (e.g. `"my_component.placeholder"`). See [`I18n::decompose_identifier`] for details.
    ///
    /// Returns an error if the message id is not found, the pattern is missing,
    /// or Fluent reports errors during formatting.
    ///
    /// Prefer the `t!`, `te!`, or `tid!` macros for ergonomic translations.
    pub fn try_translate_with_args(
        &self,
        msg: &str,
        args: Option<&FluentArgs>,
    ) -> Result<String, Error> {
        let (message_id, attribute_name) = Self::decompose_identifier(msg)?;

        let bundle = self.active_bundle.read();

        let message = bundle
            .get_message(message_id)
            .ok_or_else(|| Error::MessageIdNotFound(message_id.into()))?;

        let pattern = if let Some(attribute_name) = attribute_name {
            let attribute = message
                .get_attribute(attribute_name)
                .ok_or_else(|| Error::AttributeIdNotFound(msg.to_string()))?;
            attribute.value()
        } else {
            message
                .value()
                .ok_or_else(|| Error::MessagePatternNotFound(message_id.into()))?
        };

        let mut errors = vec![];
        let translation = bundle
            .format_pattern(pattern, args, &mut errors)
            .to_string();

        (errors.is_empty())
            .then_some(translation)
            .ok_or_else(|| Error::FluentErrorsDetected(format!("{errors:#?}")))
    }

    /// Split a message identifier into its message id and optional attribute name.
    ///
    /// - `"hello"` returns `("hello", None)`
    /// - `"my_component.placeholder"` returns `("my_component", Some("placeholder"))`
    ///
    /// Returns an error if the identifier contains more than one dot.
    pub fn decompose_identifier(msg: &str) -> Result<(&str, Option<&str>), Error> {
        let parts: Vec<&str> = msg.split('.').collect();
        match parts.as_slice() {
            [message_id] => Ok((message_id, None)),
            [message_id, attribute_name] => Ok((message_id, Some(attribute_name))),
            _ => Err(Error::InvalidMessageId(msg.to_string())),
        }
    }

    /// Translate a message by its identifier, optionally with Fluent arguments.
    ///
    /// This is the panicking version of [`I18n::try_translate_with_args`].
    ///
    /// # Panics
    ///
    /// Panics if the translation fails for any reason.
    pub fn translate_with_args(&self, msg: &str, args: Option<&FluentArgs>) -> String {
        let result = self.try_translate_with_args(msg, args);
        match result {
            Ok(translation) => translation,
            Err(err) => panic!("Failed to translate {msg}: {err}"),
        }
    }

    /// Translate a message by its identifier, without arguments.
    ///
    /// Shorthand for `self.try_translate_with_args(msg, None)`.
    #[inline]
    pub fn try_translate(&self, msg: &str) -> Result<String, Error> {
        self.try_translate_with_args(msg, None)
    }

    /// Translate a message by its identifier, without arguments.
    ///
    /// This is the panicking version of [`I18n::try_translate`].
    ///
    /// # Panics
    ///
    /// Panics if the translation fails for any reason.
    pub fn translate(&self, msg: &str) -> String {
        let result = self.try_translate(msg);
        match result {
            Ok(translation) => translation,
            Err(err) => panic!("Failed to translate {msg}: {err}"),
        }
    }

    /// Get the selected language.
    #[inline]
    pub fn language(&self) -> LanguageIdentifier {
        self.selected_language.read().clone()
    }

    /// Get the fallback language.
    pub fn fallback_language(&self) -> Option<LanguageIdentifier> {
        self.fallback_language.read().clone()
    }

    /// Update the selected language, rebuilding the active Fluent bundle.
    ///
    /// Returns an error if the bundle cannot be rebuilt for the new language.
    pub fn try_set_language(&mut self, id: LanguageIdentifier) -> Result<(), Error> {
        *self.selected_language.write() = id;
        self.try_update_active_bundle()
    }

    /// Update the selected language, rebuilding the active Fluent bundle.
    ///
    /// This is the panicking version of [`I18n::try_set_language`].
    ///
    /// # Panics
    ///
    /// Panics if the bundle cannot be rebuilt for the new language.
    pub fn set_language(&mut self, id: LanguageIdentifier) {
        let id_name = id.to_string();
        let result = self.try_set_language(id);
        match result {
            Ok(()) => (),
            Err(err) => panic!("cannot set language {id_name}: {err}"),
        }
    }

    /// Update the fallback language, rebuilding the active Fluent bundle.
    ///
    /// The given language must have a corresponding [`Locale`] registered in the config.
    /// Returns an error if no locale exists for the language or if the bundle cannot be rebuilt.
    pub fn try_set_fallback_language(&mut self, id: LanguageIdentifier) -> Result<(), Error> {
        self.locales
            .read()
            .get(&id)
            .ok_or_else(|| Error::FallbackMustHaveLocale(id.to_string()))?;

        *self.fallback_language.write() = Some(id);
        self.try_update_active_bundle()
    }

    /// Update the fallback language, rebuilding the active Fluent bundle.
    ///
    /// This is the panicking version of [`I18n::try_set_fallback_language`].
    ///
    /// # Panics
    ///
    /// Panics if no locale exists for the language or if the bundle cannot be rebuilt.
    pub fn set_fallback_language(&mut self, id: LanguageIdentifier) {
        let id_name = id.to_string();
        let result = self.try_set_fallback_language(id);
        match result {
            Ok(()) => (),
            Err(err) => panic!("cannot set fallback language {id_name}: {err}"),
        }
    }

    fn try_update_active_bundle(&mut self) -> Result<(), Error> {
        let bundle = try_create_bundle(
            &self.selected_language.peek(),
            &self.fallback_language.peek(),
            &self.locale_resources.peek(),
            &self.locales.peek(),
        )?;

        self.active_bundle.set(bundle);
        Ok(())
    }
}

fn try_create_bundle(
    selected_language: &LanguageIdentifier,
    fallback_language: &Option<LanguageIdentifier>,
    locale_resources: &[LocaleResource],
    locales: &HashMap<LanguageIdentifier, usize>,
) -> Result<FluentBundle<FluentResource>, Error> {
    let add_resource = move |bundle: &mut FluentBundle<FluentResource>,
                             langid: &LanguageIdentifier,
                             locale_resources: &[LocaleResource]| {
        if let Some(&i) = locales.get(langid) {
            let resource = &locale_resources[i];
            let resource =
                FluentResource::try_new(resource.try_to_resource_string()?).map_err(|e| {
                    Error::FluentErrorsDetected(format!("resource langid: {langid}\n{e:#?}"))
                })?;
            bundle.add_resource_overriding(resource);
        };
        Ok(())
    };

    let mut bundle = FluentBundle::new(vec![selected_language.clone()]);
    if let Some(fallback_language) = fallback_language {
        add_resource(&mut bundle, fallback_language, locale_resources)?;
    }

    let (language, script, region, variants) = selected_language.clone().into_parts();
    let variants_lang = LanguageIdentifier::from_parts(language, script, region, &variants);
    let region_lang = LanguageIdentifier::from_parts(language, script, region, &[]);
    let script_lang = LanguageIdentifier::from_parts(language, script, None, &[]);
    let language_lang = LanguageIdentifier::from_parts(language, None, None, &[]);

    add_resource(&mut bundle, &language_lang, locale_resources)?;
    add_resource(&mut bundle, &script_lang, locale_resources)?;
    add_resource(&mut bundle, &region_lang, locale_resources)?;
    add_resource(&mut bundle, &variants_lang, locale_resources)?;

    /* Add this code when the fluent crate includes FluentBundle::add_builtins.
     * This will allow the use of built-in functions like `NUMBER` and `DATETIME`.
     * See [Fluent issue](https://github.com/projectfluent/fluent-rs/issues/181) for more information.
    bundle
        .add_builtins()
        .map_err(|e| Error::FluentErrorsDetected(e.to_string()))?;
    */

    Ok(bundle)
}
