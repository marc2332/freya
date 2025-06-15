use crate::prelude::Routable;

/// Global configuration options for the router.
///
/// This implements [`Default`] and follows the builder pattern, so you can use it like this:
/// ```rust,no_run
/// # use freya_router::prelude::*;
/// # use freya::prelude::*;
/// # #[component]
/// # fn Index() -> Element {
/// #     VNode::empty()
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
    pub fn with_initial_path(self, initial_path: R) -> Self {
        Self {
            initial_path: Some(initial_path),
        }
    }
}
