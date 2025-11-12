//! # Routable

#![allow(non_snake_case)]
use std::{
    fmt::Display,
    iter::FlatMap,
    slice::Iter,
    str::FromStr,
};

use freya_core::integration::Element;

/// An error that occurs when parsing a route.
#[derive(Debug, PartialEq)]
pub struct RouteParseError<E: Display> {
    /// The attempted routes that failed to match.
    pub attempted_routes: Vec<E>,
}

impl<E: Display> Display for RouteParseError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Route did not match:\nAttempted Matches:\n")?;
        for (i, route) in self.attempted_routes.iter().enumerate() {
            writeln!(f, "{}) {route}", i + 1)?;
        }
        Ok(())
    }
}

#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "`FromQuery` is not implemented for `{Self}`",
        label = "spread query",
        note = "FromQuery is automatically implemented for types that implement `From<&str>`. You need to either implement From<&str> or implement FromQuery manually."
    )
)]
pub trait FromQuery {
    /// Create an instance of `Self` from a query string.
    fn from_query(query: &str) -> Self;
}

impl<T: for<'a> From<&'a str>> FromQuery for T {
    fn from_query(query: &str) -> Self {
        T::from(query)
    }
}

#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "`FromQueryArgument` is not implemented for `{Self}`",
        label = "query argument",
        note = "FromQueryArgument is automatically implemented for types that implement `FromStr` and `Default`. You need to either implement FromStr and Default or implement FromQueryArgument manually."
    )
)]
pub trait FromQueryArgument: Default {
    /// The error that can occur when parsing a query argument.
    type Err;

    /// Create an instance of `Self` from a query string.
    fn from_query_argument(argument: &str) -> Result<Self, Self::Err>;
}

impl<T: Default + FromStr> FromQueryArgument for T
where
    <T as FromStr>::Err: Display,
{
    type Err = <T as FromStr>::Err;

    fn from_query_argument(argument: &str) -> Result<Self, Self::Err> {
        match T::from_str(argument) {
            Ok(result) => Ok(result),
            Err(err) => Err(err),
        }
    }
}

#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "`FromHashFragment` is not implemented for `{Self}`",
        label = "hash fragment",
        note = "FromHashFragment is automatically implemented for types that implement `FromStr` and `Default`. You need to either implement FromStr and Default or implement FromHashFragment manually."
    )
)]
pub trait FromHashFragment {
    /// Create an instance of `Self` from a hash fragment.
    fn from_hash_fragment(hash: &str) -> Self;
}

impl<T> FromHashFragment for T
where
    T: FromStr + Default,
    T::Err: std::fmt::Display,
{
    fn from_hash_fragment(hash: &str) -> Self {
        T::from_str(hash).unwrap_or_default()
    }
}

#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "`FromRouteSegment` is not implemented for `{Self}`",
        label = "route segment",
        note = "FromRouteSegment is automatically implemented for types that implement `FromStr` and `Default`. You need to either implement FromStr and Default or implement FromRouteSegment manually."
    )
)]
pub trait FromRouteSegment: Sized {
    /// The error that can occur when parsing a route segment.
    type Err;

    /// Create an instance of `Self` from a route segment.
    fn from_route_segment(route: &str) -> Result<Self, Self::Err>;
}

impl<T: FromStr> FromRouteSegment for T
where
    <T as FromStr>::Err: Display,
{
    type Err = <T as FromStr>::Err;

    fn from_route_segment(route: &str) -> Result<Self, Self::Err> {
        T::from_str(route)
    }
}

#[test]
fn full_circle() {
    let route = "testing 1234 hello world";
    assert_eq!(String::from_route_segment(route).unwrap(), route);
}

pub trait ToRouteSegments {
    /// Display the route segments with each route segment separated by a `/`. This should not start with a `/`.
    fn display_route_segments(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

// Implement ToRouteSegments for any type that can turn &self into an iterator of &T where T: Display
impl<I, T: Display> ToRouteSegments for I
where
    for<'a> &'a I: IntoIterator<Item = &'a T>,
{
    fn display_route_segments(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for segment in self {
            write!(f, "/")?;
            let segment = segment.to_string();
            let encoded = urlencoding::encode(&segment);
            write!(f, "{encoded}")?;
        }
        Ok(())
    }
}

#[test]
fn to_route_segments() {
    struct DisplaysRoute;

    impl Display for DisplaysRoute {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let segments = vec!["hello", "world"];
            segments.display_route_segments(f)
        }
    }

    assert_eq!(DisplaysRoute.to_string(), "/hello/world");
}

#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "`FromRouteSegments` is not implemented for `{Self}`",
        label = "spread route segments",
        note = "FromRouteSegments is automatically implemented for types that implement `FromIterator` with an `Item` type that implements `Display`. You need to either implement FromIterator or implement FromRouteSegments manually."
    )
)]
pub trait FromRouteSegments: Sized {
    /// The error that can occur when parsing route segments.
    type Err: std::fmt::Display;

    /// Create an instance of `Self` from route segments.
    ///
    /// NOTE: This method must parse the output of `ToRouteSegments::display_route_segments` into the type `Self`.
    fn from_route_segments(segments: &[&str]) -> Result<Self, Self::Err>;
}

impl<I: std::iter::FromIterator<String>> FromRouteSegments for I {
    type Err = <String as FromRouteSegment>::Err;

    fn from_route_segments(segments: &[&str]) -> Result<Self, Self::Err> {
        segments
            .iter()
            .map(|s| String::from_route_segment(s))
            .collect()
    }
}

/// A flattened version of [`Routable::SITE_MAP`].
/// This essentially represents a `Vec<Vec<SegmentType>>`, which you can collect it into.
type SiteMapFlattened<'a> = FlatMap<
    Iter<'a, SiteMapSegment>,
    Vec<Vec<SegmentType>>,
    fn(&SiteMapSegment) -> Vec<Vec<SegmentType>>,
>;

#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "`Routable` is not implemented for `{Self}`",
        label = "Route",
        note = "Routable should generally be derived using the `#[derive(Routable)]` macro."
    )
)]
pub trait Routable: FromStr<Err: Display> + Display + Clone + 'static {
    /// The error that can occur when parsing a route.
    const SITE_MAP: &'static [SiteMapSegment];

    /// Render the route at the given level
    fn render(&self, level: usize) -> Element;

    /// Checks if this route is a child of the given route.
    fn is_child_of(&self, other: &Self) -> bool {
        let self_str = self.to_string();
        let self_str = self_str
            .split_once('#')
            .map(|(route, _)| route)
            .unwrap_or(&self_str);
        let self_str = self_str
            .split_once('?')
            .map(|(route, _)| route)
            .unwrap_or(self_str);
        let self_str = self_str.trim_end_matches('/');
        let other_str = other.to_string();
        let other_str = other_str
            .split_once('#')
            .map(|(route, _)| route)
            .unwrap_or(&other_str);
        let other_str = other_str
            .split_once('?')
            .map(|(route, _)| route)
            .unwrap_or(other_str);
        let other_str = other_str.trim_end_matches('/');

        let mut self_segments = self_str.split('/');
        let mut other_segments = other_str.split('/');
        loop {
            match (self_segments.next(), other_segments.next()) {
                // If the two routes are the same length, or this route has less segments, then this segment
                // cannot be the child of the other segment
                (None, Some(_)) | (None, None) => {
                    return false;
                }
                // If two segments are not the same, then this segment cannot be the child of the other segment
                (Some(self_seg), Some(other_seg)) => {
                    if self_seg != other_seg {
                        return false;
                    }
                }
                // If the other route has less segments, then this route is the child of the other route
                (Some(_), None) => break,
            }
        }
        true
    }

    /// Get the parent route of this route.
    fn parent(&self) -> Option<Self> {
        let as_str = self.to_string();
        let (route_and_query, _) = as_str.split_once('#').unwrap_or((&as_str, ""));
        let (route, _) = route_and_query
            .split_once('?')
            .unwrap_or((route_and_query, ""));
        let route = route.trim_end_matches('/');
        let segments = route.split_inclusive('/');
        let segment_count = segments.clone().count();
        let new_route: String = segments.take(segment_count.saturating_sub(1)).collect();
        Self::from_str(&new_route).ok()
    }

    /// Returns a flattened version of [`Self::SITE_MAP`].
    fn flatten_site_map<'a>() -> SiteMapFlattened<'a> {
        Self::SITE_MAP.iter().flat_map(SiteMapSegment::flatten)
    }

    /// Gets a list of all the static routes.
    /// Example static route: `#[route("/static/route")]`
    fn static_routes() -> Vec<Self> {
        Self::flatten_site_map()
            .filter_map(|segments| {
                let mut route = String::new();
                for segment in segments.iter() {
                    match segment {
                        SegmentType::Static(s) => {
                            route.push('/');
                            route.push_str(s)
                        }
                        SegmentType::Child => {}
                        _ => return None,
                    }
                }

                route.parse().ok()
            })
            .collect()
    }
}

/// A type erased map of the site structure.
#[derive(Debug, Clone, PartialEq)]
pub struct SiteMapSegment {
    /// The type of the route segment.
    pub segment_type: SegmentType,
    /// The children of the route segment.
    pub children: &'static [SiteMapSegment],
}

impl SiteMapSegment {
    /// Take a map of the site structure and flatten it into a vector of routes.
    pub fn flatten(&self) -> Vec<Vec<SegmentType>> {
        let mut routes = Vec::new();
        self.flatten_inner(&mut routes, Vec::new());
        routes
    }

    fn flatten_inner(&self, routes: &mut Vec<Vec<SegmentType>>, current: Vec<SegmentType>) {
        let mut current = current;
        current.push(self.segment_type.clone());
        if self.children.is_empty() {
            routes.push(current);
        } else {
            for child in self.children {
                child.flatten_inner(routes, current.clone());
            }
        }
    }
}

/// The type of a route segment.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum SegmentType {
    /// A static route segment.
    Static(&'static str),
    /// A dynamic route segment.
    Dynamic(&'static str),
    /// A catch all route segment.
    CatchAll(&'static str),
    /// A child router.
    Child,
}

impl SegmentType {
    /// Try to convert this segment into a static segment.
    pub fn to_static(&self) -> Option<&'static str> {
        match self {
            SegmentType::Static(s) => Some(*s),
            _ => None,
        }
    }

    /// Try to convert this segment into a dynamic segment.
    pub fn to_dynamic(&self) -> Option<&'static str> {
        match self {
            SegmentType::Dynamic(s) => Some(*s),
            _ => None,
        }
    }

    /// Try to convert this segment into a catch all segment.
    pub fn to_catch_all(&self) -> Option<&'static str> {
        match self {
            SegmentType::CatchAll(s) => Some(*s),
            _ => None,
        }
    }

    /// Try to convert this segment into a child segment.
    pub fn to_child(&self) -> Option<()> {
        match self {
            SegmentType::Child => Some(()),
            _ => None,
        }
    }
}

impl Display for SegmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            SegmentType::Static(s) => write!(f, "/{s}"),
            SegmentType::Child => Ok(()),
            SegmentType::Dynamic(s) => write!(f, "/:{s}"),
            SegmentType::CatchAll(s) => write!(f, "/:..{s}"),
        }
    }
}
