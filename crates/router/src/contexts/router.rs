use std::{
    collections::HashSet,
    error::Error,
    fmt::Display,
    sync::{
        Arc,
        Mutex,
    },
};

use dioxus_lib::prelude::*;
use tracing::error;

use crate::{
    components::child_router::consume_child_route_mapping,
    memory::MemoryHistory,
    navigation::NavigationTarget,
    prelude::SiteMapSegment,
    routable::Routable,
    router_cfg::RouterConfig,
};

/// An error that is thrown when the router fails to parse a route
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

/// An error that can occur when navigating.
#[derive(Debug, Clone)]
pub struct ExternalNavigationFailure(pub String);

struct RouterContextInner {
    subscribers: Arc<Mutex<HashSet<ReactiveContext>>>,

    internal_route: fn(&str) -> bool,

    site_map: &'static [SiteMapSegment],

    history: MemoryHistory,
}

impl RouterContextInner {
    fn update_subscribers(&self) {
        for &id in self.subscribers.lock().unwrap().iter() {
            id.mark_dirty();
        }
    }

    fn subscribe_to_current_context(&self) {
        if let Some(rc) = ReactiveContext::current() {
            rc.subscribe(self.subscribers.clone());
        }
    }

    fn external(&mut self, external: String) -> Option<ExternalNavigationFailure> {
        let failure = ExternalNavigationFailure(external);

        self.update_subscribers();

        Some(failure)
    }
}

/// A collection of router data that manages all routing functionality.
#[derive(Clone, Copy)]
pub struct RouterContext {
    inner: CopyValue<RouterContextInner>,
}

impl RouterContext {
    pub(crate) fn new<R: Routable + 'static>(cfg: RouterConfig<R>) -> Self {
        let subscribers = Arc::new(Mutex::new(HashSet::new()));

        let history = if let Some(initial_path) = cfg.initial_path {
            MemoryHistory::with_initial_path(initial_path)
        } else {
            MemoryHistory::default()
        };

        Self {
            inner: CopyValue::new(RouterContextInner {
                subscribers: subscribers.clone(),

                internal_route: |route| R::from_str(route).is_ok(),

                site_map: R::SITE_MAP,

                history,
            }),
        }
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

    /// Go back to the previous location.
    ///
    /// Will fail silently if there is no previous location to go to.
    pub fn go_back(&self) {
        self.inner.peek().history.go_back();
        self.change_route();
    }

    /// Go back to the next location.
    ///
    /// Will fail silently if there is no next location to go to.
    pub fn go_forward(&self) {
        self.inner.peek().history.go_forward();
        self.change_route();
    }

    /// Push a new location.
    ///
    /// The previous location will be available to go back to.
    pub fn push(&self, target: impl Into<NavigationTarget>) -> Option<ExternalNavigationFailure> {
        let target = target.into();
        {
            let mut write = self.inner.write_unchecked();
            match target {
                NavigationTarget::Internal(p) => write.history.push(p),
                NavigationTarget::External(e) => return write.external(e),
            }
        }

        self.change_route();
        None
    }

    /// Replace the current location.
    ///
    /// The previous location will **not** be available to go back to.
    pub fn replace(
        &self,
        target: impl Into<NavigationTarget>,
    ) -> Option<ExternalNavigationFailure> {
        let target = target.into();
        {
            let mut write = self.inner.write_unchecked();
            match target {
                NavigationTarget::Internal(p) => write.history.replace(p),
                NavigationTarget::External(e) => return write.external(e),
            }
        }

        self.change_route();
        None
    }

    /// The route that is currently active.
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
            Err(err) => {
                error!("Parse route error: {err:?}");
                throw_error(ParseRouteError { message: err });
                "/".parse().unwrap_or_else(|err| panic!("{err}"))
            }
        }
    }

    /// The full route that is currently active. If this is called from inside a child router, this will always return the parent's view of the route.
    pub fn full_route_string(&self) -> String {
        let inner = self.inner.read();
        inner.subscribe_to_current_context();

        self.inner.peek_unchecked().history.current_route()
    }

    /// Get the site map of the router.
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

/// This context is set to the RouterConfig on_update method
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

    /// Go back to the previous location.
    ///
    /// Will fail silently if there is no previous location to go to.
    pub fn go_back(&self) {
        self.inner.go_back();
    }

    /// Go back to the next location.
    ///
    /// Will fail silently if there is no next location to go to.
    pub fn go_forward(&self) {
        self.inner.go_forward();
    }

    /// Push a new location.
    ///
    /// The previous location will be available to go back to.
    pub fn push(
        &self,
        target: impl Into<NavigationTarget<R>>,
    ) -> Option<ExternalNavigationFailure> {
        self.inner.push(target.into())
    }

    /// Replace the current location.
    ///
    /// The previous location will **not** be available to go back to.
    pub fn replace(
        &self,
        target: impl Into<NavigationTarget<R>>,
    ) -> Option<ExternalNavigationFailure> {
        self.inner.replace(target.into())
    }

    /// The route that is currently active.
    pub fn current(&self) -> R
    where
        R: Clone,
    {
        self.inner.current()
    }
}
