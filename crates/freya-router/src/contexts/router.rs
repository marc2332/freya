use std::{
    cell::RefCell,
    error::Error,
    fmt::Display,
    rc::Rc,
};

use freya_core::{
    integration::FxHashSet,
    prelude::*,
};

use crate::{
    components::child_router::consume_child_route_mapping,
    memory::MemoryHistory,
    navigation::NavigationTarget,
    prelude::SiteMapSegment,
    routable::Routable,
    router_cfg::RouterConfig,
};

/// An error returned when a route cannot be parsed.
#[derive(Debug, Clone)]
pub struct ParseRouteError {
    message: String,
}

impl Error for ParseRouteError {}
impl Display for ParseRouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

/// An error that can occur when navigating to an external URL.
#[derive(Debug, Clone)]
pub struct ExternalNavigationFailure(pub String);

impl Error for ExternalNavigationFailure {}
impl Display for ExternalNavigationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "External navigation failed: {}", self.0)
    }
}

struct RouterContextInner {
    subscribers: Rc<RefCell<FxHashSet<ReactiveContext>>>,

    internal_route: fn(&str) -> bool,

    site_map: &'static [SiteMapSegment],

    history: MemoryHistory,
}

impl RouterContextInner {
    fn update_subscribers(&self) {
        for id in self.subscribers.borrow().iter() {
            id.notify();
        }
    }

    fn subscribe_to_current_context(&self) {
        if let Some(mut rc) = ReactiveContext::try_current() {
            rc.subscribe(&self.subscribers);
        }
    }

    fn external(&mut self, external: String) -> Result<(), ExternalNavigationFailure> {
        let failure = ExternalNavigationFailure(external);

        self.update_subscribers();

        Err(failure)
    }
}

/// Navigation state for the nearest [`Router`](crate::components::Router).
///
/// You can retrieve this context with [`Self::get`].
///
/// ```rust,no_run
/// # use freya::{components::Button, prelude::*, router::*};
/// # #[derive(Routable, Clone, PartialEq)]
/// # enum Route { #[route("/")] Home, #[route("/settings")] Settings }
/// # #[derive(PartialEq)]
/// # struct Page;
/// # impl Component for Page {
/// #     fn render(&self) -> impl IntoElement {
/// let router = RouterContext::get();
/// Button::new()
///     .on_press(move |_| {
///         let _ = router.replace(Route::Settings);
///     })
///     .child("Settings")
/// #     }
/// # }
/// ```
#[derive(Clone, Copy)]
pub struct RouterContext {
    inner: State<RouterContextInner>,
}

impl RouterContext {
    pub(crate) fn create<R: Routable + 'static>(cfg: RouterConfig<R>) -> Self {
        let subscribers = Rc::new(RefCell::new(FxHashSet::default()));

        let history = if let Some(initial_path) = cfg.initial_path {
            MemoryHistory::with_initial_path(initial_path)
        } else {
            MemoryHistory::default()
        };

        Self {
            inner: State::create(RouterContextInner {
                subscribers,

                internal_route: |route| R::from_str(route).is_ok(),

                site_map: R::SITE_MAP,

                history,
            }),
        }
    }

    /// Creates a router context that lives for the application lifetime.
    ///
    /// This is useful for sharing router state across multiple windows. Provide it
    /// to each window with [`use_share_router`](crate::components::use_share_router).
    ///
    /// This is **not** a hook, do not use it inside components like you would [`use_route`](crate::hooks::use_route).
    /// You would usually want to call this in your `main` function, not anywhere else.
    ///
    /// # Example
    ///
    /// ```rust, ignore
    /// # use freya::prelude::*;
    /// # use freya::router::*;
    ///
    /// fn main() {
    ///     let router = RouterContext::create_global::<Route>(RouterConfig::default());
    ///
    ///     launch(
    ///         LaunchConfig::new()
    ///             .with_window(WindowConfig::new_app(MyApp { router })),
    ///     );
    /// }
    /// ```
    pub fn create_global<R: Routable + 'static>(cfg: RouterConfig<R>) -> Self {
        let subscribers = Rc::new(RefCell::new(FxHashSet::default()));

        let history = if let Some(initial_path) = cfg.initial_path {
            MemoryHistory::with_initial_path(initial_path)
        } else {
            MemoryHistory::default()
        };

        Self {
            inner: State::create_global(RouterContextInner {
                subscribers,

                internal_route: |route| R::from_str(route).is_ok(),

                site_map: R::SITE_MAP,

                history,
            }),
        }
    }

    /// Returns the router context from the current component subtree, if present.
    ///
    /// Unlike [`Self::get`], this does not panic outside a router.
    pub fn try_get() -> Option<Self> {
        try_consume_context()
    }

    /// Returns the router context from the current component subtree.
    ///
    /// Call this while rendering a descendant of [`Router`](crate::components::Router),
    /// then capture the returned context in event handlers. This panics outside a router.
    #[track_caller]
    pub fn get() -> Self {
        consume_context()
    }

    /// Check whether there is a previous page to navigate back to.
    #[must_use]
    pub fn can_go_back(&self) -> bool {
        self.inner.peek().history.can_go_back()
    }

    /// Check whether there is a future page to navigate forward to.
    #[must_use]
    pub fn can_go_forward(&self) -> bool {
        self.inner.peek().history.can_go_forward()
    }

    /// Goes back to the previous location.
    ///
    /// Does nothing when there is no previous location.
    pub fn go_back(&self) {
        self.inner.peek().history.go_back();
        self.change_route();
    }

    /// Goes forward to the next location.
    ///
    /// Does nothing when there is no next location.
    pub fn go_forward(&self) {
        self.inner.peek().history.go_forward();
        self.change_route();
    }

    /// Pushes a location onto the navigation history.
    ///
    /// Pass a route variant for an internal destination.
    /// External URLs return [`ExternalNavigationFailure`].
    pub fn push(
        &self,
        target: impl Into<NavigationTarget>,
    ) -> Result<(), ExternalNavigationFailure> {
        let target = target.into();
        {
            let mut write = self.inner.write_unchecked();
            match target {
                NavigationTarget::Internal(p) => write.history.push(p),
                NavigationTarget::External(e) => return write.external(e),
            }
        }

        self.change_route();
        Ok(())
    }

    /// Replaces the current location without adding a history entry.
    ///
    /// External URLs return [`ExternalNavigationFailure`].
    pub fn replace(
        &self,
        target: impl Into<NavigationTarget>,
    ) -> Result<(), ExternalNavigationFailure> {
        let target = target.into();
        {
            let mut write = self.inner.write_unchecked();
            match target {
                NavigationTarget::Internal(p) => write.history.replace(p),
                NavigationTarget::External(e) => return write.external(e),
            }
        }

        self.change_route();
        Ok(())
    }

    /// Returns the active route as `R`.
    ///
    /// This subscribes the current component to route changes. Prefer
    /// [`use_route`](crate::hooks::use_route) in components.
    pub fn current<R: Routable>(&self) -> R {
        let absolute_route = self.full_route_string();
        // If this is a child route, map the absolute route to the child route before parsing
        let mapping = consume_child_route_mapping::<R>();
        let route = match mapping.as_ref() {
            Some(mapping) => mapping
                .parse_route_from_root_route(&absolute_route)
                .ok_or_else(|| "Failed to parse route".to_string()),
            None => {
                R::from_str(&absolute_route).map_err(|err| format!("Failed to parse route {err}"))
            }
        };

        match route {
            Ok(route) => route,
            Err(_err) => "/".parse().unwrap_or_else(|err| panic!("{err}")),
        }
    }

    /// Returns the complete active route as a path string.
    ///
    /// From a child router, this returns the root router's path.
    pub fn full_route_string(&self) -> String {
        let inner = self.inner.read();
        inner.subscribe_to_current_context();

        self.inner.peek().history.current_route()
    }

    /// Returns the route type's generated site map.
    pub fn site_map(&self) -> &'static [SiteMapSegment] {
        self.inner.read().site_map
    }

    fn change_route(&self) {
        self.inner.read().update_subscribers();
    }

    pub(crate) fn internal_route(&self, route: &str) -> bool {
        (self.inner.read().internal_route)(route)
    }
}

/// A typed wrapper around [`RouterContext`].
///
/// This exposes the same navigation operations while accepting the route type
/// directly. It is intended for APIs that need a typed router context.
pub struct GenericRouterContext<R> {
    inner: RouterContext,
    _marker: std::marker::PhantomData<R>,
}

impl<R> GenericRouterContext<R>
where
    R: Routable,
{
    /// Check whether there is a previous page to navigate back to.
    #[must_use]
    pub fn can_go_back(&self) -> bool {
        self.inner.can_go_back()
    }

    /// Check whether there is a future page to navigate forward to.
    #[must_use]
    pub fn can_go_forward(&self) -> bool {
        self.inner.can_go_forward()
    }

    /// Goes back to the previous location.
    ///
    /// Does nothing when there is no previous location.
    pub fn go_back(&self) {
        self.inner.go_back();
    }

    /// Goes forward to the next location.
    ///
    /// Does nothing when there is no next location.
    pub fn go_forward(&self) {
        self.inner.go_forward();
    }

    /// Pushes a location onto the navigation history.
    ///
    /// The previous location becomes available through [`Self::go_back`].
    pub fn push(
        &self,
        target: impl Into<NavigationTarget<R>>,
    ) -> Result<(), ExternalNavigationFailure> {
        self.inner.push(target.into())
    }

    /// Replaces the current location without adding a history entry.
    pub fn replace(
        &self,
        target: impl Into<NavigationTarget<R>>,
    ) -> Result<(), ExternalNavigationFailure> {
        self.inner.replace(target.into())
    }

    /// Returns the active route.
    pub fn current(&self) -> R
    where
        R: Clone,
    {
        self.inner.current()
    }
}
