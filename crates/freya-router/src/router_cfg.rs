use crate::prelude::Routable;

/// Global configuration options for the router.
///
/// This implements [`Default`] and follows the builder pattern, so you can use it like this:
/// ```rust,no_run
/// # use freya_router::prelude::*;
/// # use freya_core::prelude::*;
/// # #[derive(PartialEq)]
/// # struct Index;
/// # impl Component for Index {
/// #    fn render(&self) -> impl IntoElement {
/// #        rect()
/// #    }
/// # }
/// #[derive(Clone, Routable)]
/// enum Route {
///     #[route("/")]
///     Index {},
/// }
///
/// let cfg = RouterConfig::<Route>::default().with_initial_path(Route::Index {});
/// ```
pub struct RouterConfig<R: Routable> {
    pub(crate) initial_path: Option<R>,
}

impl<R: Routable> Default for RouterConfig<R> {
    fn default() -> Self {
        Self { initial_path: None }
    }
}

impl<R: Routable> RouterConfig<R> {
    pub fn with_initial_path(mut self, initial_path: R) -> Self {
        self.initial_path = Some(initial_path);
        self
    }
}
