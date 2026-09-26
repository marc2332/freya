//! Types pertaining to navigation.

use std::{
    fmt::{
        Debug,
        Display,
    },
    str::FromStr,
};

use url::{
    ParseError,
    Url,
};

use crate::{
    components::child_router::consume_child_route_mapping,
    prelude::RouterContext,
    routable::Routable,
};

impl<R: Routable> From<R> for NavigationTarget {
    fn from(value: R) -> Self {
        // If this is a child route, map it to the root route first
        let mapping = consume_child_route_mapping();
        match mapping.as_ref() {
            Some(mapping) => NavigationTarget::Internal(mapping.format_route_as_root_route(value)),
            // Otherwise, just use the internal route
            None => NavigationTarget::Internal(value.to_string()),
        }
    }
}

/// A destination for router navigation.
///
/// Route variants and valid relative paths become [`Self::Internal`]. Absolute
/// URLs become [`Self::External`]. [`RouterContext::push`](crate::prelude::RouterContext::push)
/// and [`RouterContext::replace`](crate::prelude::RouterContext::replace) accept
/// values that convert into this type.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NavigationTarget<R = String> {
    /// Route handled by a router.
    ///
    /// ```rust
    /// # use freya::prelude::*;
    /// # use freya_router::prelude::*;
    /// # #[derive(PartialEq)]
    /// # struct Index;
    /// # impl Component for Index {
    /// #    fn render(&self) -> impl IntoElement {
    /// #        rect()
    /// #    }
    /// # }
    /// #[derive(Clone, Routable, PartialEq, Debug)]
    /// enum Route {
    ///     #[route("/")]
    ///     Index {},
    /// }
    /// let explicit = NavigationTarget::Internal(Route::Index {});
    /// let implicit: NavigationTarget<Route> = "/".parse().unwrap();
    /// assert_eq!(explicit, implicit);
    /// ```
    Internal(R),
    /// An absolute URL outside the current router.
    External(String),
}

impl<R: Routable> From<&str> for NavigationTarget<R> {
    fn from(value: &str) -> Self {
        value
            .parse()
            .unwrap_or_else(|_| Self::External(value.to_string()))
    }
}

impl<R: Routable> From<&String> for NavigationTarget<R> {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

impl<R: Routable> From<String> for NavigationTarget<R> {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl<R: Routable> From<R> for NavigationTarget<R> {
    fn from(value: R) -> Self {
        Self::Internal(value)
    }
}

impl From<&str> for NavigationTarget {
    fn from(value: &str) -> Self {
        match RouterContext::try_get() {
            Some(router) => match router.internal_route(value) {
                true => NavigationTarget::Internal(value.to_string()),
                false => NavigationTarget::External(value.to_string()),
            },
            None => NavigationTarget::External(value.to_string()),
        }
    }
}

impl From<String> for NavigationTarget {
    fn from(value: String) -> Self {
        match RouterContext::try_get() {
            Some(router) => match router.internal_route(&value) {
                true => NavigationTarget::Internal(value),
                false => NavigationTarget::External(value),
            },
            None => NavigationTarget::External(value),
        }
    }
}

impl<R: Routable> From<NavigationTarget<R>> for NavigationTarget {
    fn from(value: NavigationTarget<R>) -> Self {
        match value {
            NavigationTarget::Internal(r) => r.into(),
            NavigationTarget::External(s) => Self::External(s),
        }
    }
}

impl<R: Routable> Display for NavigationTarget<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NavigationTarget::Internal(r) => write!(f, "{r}"),
            NavigationTarget::External(s) => write!(f, "{s}"),
        }
    }
}

/// An error returned when parsing a [`NavigationTarget`] from a string.
pub enum NavigationTargetParseError<R: Routable> {
    /// The value is neither a valid absolute URL nor a valid internal route.
    InvalidUrl(ParseError),
    /// The relative URL does not match the route type.
    InvalidInternalURL(<R as FromStr>::Err),
}

impl<R: Routable> Debug for NavigationTargetParseError<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NavigationTargetParseError::InvalidUrl(e) => write!(f, "Invalid URL: {e}"),
            NavigationTargetParseError::InvalidInternalURL(_) => {
                write!(f, "Invalid internal URL")
            }
        }
    }
}

impl<R: Routable> FromStr for NavigationTarget<R> {
    type Err = NavigationTargetParseError<R>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Url::parse(s) {
            Ok(_) => Ok(Self::External(s.to_string())),
            Err(ParseError::RelativeUrlWithoutBase) => {
                Ok(Self::Internal(R::from_str(s).map_err(|e| {
                    NavigationTargetParseError::InvalidInternalURL(e)
                })?))
            }
            Err(e) => Err(NavigationTargetParseError::InvalidUrl(e)),
        }
    }
}
