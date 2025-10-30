#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/79236386")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/79236386")]

extern crate proc_macro;

use layout::Layout;
use nest::{
    Nest,
    NestId,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{
    __private::Span,
    ToTokens,
    format_ident,
    quote,
};
use redirect::Redirect;
use route::{
    Route,
    RouteType,
};
use segment::RouteSegment;
use syn::{
    Ident,
    Token,
    Type,
    parse::ParseStream,
    parse_macro_input,
};

use crate::{
    layout::LayoutId,
    route_tree::ParseRouteTree,
};

mod hash;
mod layout;
mod nest;
mod query;
mod redirect;
mod route;
mod route_tree;
mod segment;

#[doc(alias = "route")]
#[proc_macro_derive(
    Routable,
    attributes(route, nest, end_nest, layout, end_layout, redirect, child)
)]
pub fn routable(input: TokenStream) -> TokenStream {
    let routes_enum = parse_macro_input!(input as syn::ItemEnum);

    let route_enum = match RouteEnum::parse(routes_enum) {
        Ok(route_enum) => route_enum,
        Err(err) => return err.to_compile_error().into(),
    };

    let error_type = route_enum.error_type();
    let parse_impl = route_enum.parse_impl();
    let display_impl = route_enum.impl_display();
    let routable_impl = route_enum.routable_impl();

    (quote! {
        const _: () = {
            #error_type

            #display_impl

            #routable_impl

            #parse_impl
        };
    })
    .into()
}

struct RouteEnum {
    name: Ident,
    endpoints: Vec<RouteEndpoint>,
    nests: Vec<Nest>,
    layouts: Vec<Layout>,
    site_map: Vec<SiteMapSegment>,
}

impl RouteEnum {
    fn parse(data: syn::ItemEnum) -> syn::Result<Self> {
        let name = &data.ident;

        let mut site_map = Vec::new();
        let mut site_map_stack: Vec<Vec<SiteMapSegment>> = Vec::new();

        let mut endpoints = Vec::new();

        let mut layouts: Vec<Layout> = Vec::new();
        let mut layout_stack = Vec::new();

        let mut nests = Vec::new();
        let mut nest_stack = Vec::new();

        for variant in &data.variants {
            let mut excluded = Vec::new();
            // Apply the any nesting attributes in order
            for attr in &variant.attrs {
                if attr.path().is_ident("nest") {
                    let mut children_routes = Vec::new();
                    {
                        // add all of the variants of the enum to the children_routes until we hit an end_nest
                        let mut level = 0;
                        'o: for variant in &data.variants {
                            children_routes.push(variant.fields.clone());
                            for attr in &variant.attrs {
                                if attr.path().is_ident("nest") {
                                    level += 1;
                                } else if attr.path().is_ident("end_nest") {
                                    level -= 1;
                                    if level < 0 {
                                        break 'o;
                                    }
                                }
                            }
                        }
                    }

                    let nest_index = nests.len();

                    let parser = |input: ParseStream| {
                        Nest::parse(
                            input,
                            children_routes
                                .iter()
                                .filter_map(|f: &syn::Fields| match f {
                                    syn::Fields::Named(fields) => Some(fields.clone()),
                                    _ => None,
                                })
                                .collect(),
                            nest_index,
                        )
                    };
                    let nest = attr.parse_args_with(parser)?;

                    // add the current segment to the site map stack
                    let segments: Vec<_> = nest
                        .segments
                        .iter()
                        .map(|seg| {
                            let segment_type = seg.into();
                            SiteMapSegment {
                                segment_type,
                                children: Vec::new(),
                            }
                        })
                        .collect();
                    if !segments.is_empty() {
                        site_map_stack.push(segments);
                    }

                    nests.push(nest);
                    nest_stack.push(NestId(nest_index));
                } else if attr.path().is_ident("end_nest") {
                    nest_stack.pop();
                    // pop the current nest segment off the stack and add it to the parent or the site map
                    if let Some(segment) = site_map_stack.pop() {
                        let children = site_map_stack
                            .last_mut()
                            .map(|seg| &mut seg.last_mut().unwrap().children)
                            .unwrap_or(&mut site_map);

                        // Turn the list of segments in the segments stack into a tree
                        let mut iter = segment.into_iter().rev();
                        let mut current = iter.next().unwrap();
                        for mut segment in iter {
                            segment.children.push(current);
                            current = segment;
                        }

                        children.push(current);
                    }
                } else if attr.path().is_ident("layout") {
                    let parser = |input: ParseStream| {
                        let bang: Option<Token![!]> = input.parse().ok();
                        let exclude = bang.is_some();
                        Ok((exclude, Layout::parse(input, nest_stack.clone())?))
                    };
                    let (exclude, layout): (bool, Layout) = attr.parse_args_with(parser)?;

                    if exclude {
                        let Some(layout_index) = layouts.iter().position(|l| l.comp == layout.comp)
                        else {
                            return Err(syn::Error::new(
                                Span::call_site(),
                                "Attempted to exclude a layout that does not exist",
                            ));
                        };
                        excluded.push(LayoutId(layout_index));
                    } else {
                        let layout_index = layouts.len();
                        layouts.push(layout);
                        layout_stack.push(LayoutId(layout_index));
                    }
                } else if attr.path().is_ident("end_layout") {
                    layout_stack.pop();
                } else if attr.path().is_ident("redirect") {
                    let parser = |input: ParseStream| {
                        Redirect::parse(input, nest_stack.clone(), endpoints.len())
                    };
                    let redirect = attr.parse_args_with(parser)?;
                    endpoints.push(RouteEndpoint::Redirect(redirect));
                }
            }

            let active_nests = nest_stack.clone();
            let mut active_layouts = layout_stack.clone();
            active_layouts.retain(|&id| !excluded.contains(&id));

            let route = Route::parse(active_nests, active_layouts, variant.clone())?;

            // add the route to the site map
            let mut segment = SiteMapSegment::new(&route.segments);
            if let RouteType::Child(child) = &route.ty {
                let new_segment = SiteMapSegment {
                    segment_type: SegmentType::Child(child.ty.clone()),
                    children: Vec::new(),
                };
                match &mut segment {
                    Some(segment) => {
                        fn set_last_child_to(
                            segment: &mut SiteMapSegment,
                            new_segment: SiteMapSegment,
                        ) {
                            if let Some(last) = segment.children.last_mut() {
                                set_last_child_to(last, new_segment);
                            } else {
                                segment.children = vec![new_segment];
                            }
                        }
                        set_last_child_to(segment, new_segment);
                    }
                    None => {
                        segment = Some(new_segment);
                    }
                }
            }

            if let Some(segment) = segment {
                let parent = site_map_stack.last_mut();
                let children = match parent {
                    Some(parent) => &mut parent.last_mut().unwrap().children,
                    None => &mut site_map,
                };
                children.push(segment);
            }

            endpoints.push(RouteEndpoint::Route(route));
        }

        // pop any remaining site map segments
        while let Some(segment) = site_map_stack.pop() {
            let children = site_map_stack
                .last_mut()
                .map(|seg| &mut seg.last_mut().unwrap().children)
                .unwrap_or(&mut site_map);

            // Turn the list of segments in the segments stack into a tree
            let mut iter = segment.into_iter().rev();
            let mut current = iter.next().unwrap();
            for mut segment in iter {
                segment.children.push(current);
                current = segment;
            }

            children.push(current);
        }

        let myself = Self {
            name: name.clone(),
            endpoints,
            nests,
            layouts,
            site_map,
        };

        Ok(myself)
    }

    fn impl_display(&self) -> TokenStream2 {
        let mut display_match = Vec::new();

        for route in &self.endpoints {
            if let RouteEndpoint::Route(route) = route {
                display_match.push(route.display_match(&self.nests));
            }
        }

        let name = &self.name;

        quote! {
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    #[allow(unused)]
                    match self {
                        #(#display_match)*
                    }
                    Ok(())
                }
            }
        }
    }

    fn parse_impl(&self) -> TokenStream2 {
        let tree = ParseRouteTree::new(&self.endpoints, &self.nests);
        let name = &self.name;

        let error_name = format_ident!("{}MatchError", self.name);
        let tokens = tree.roots.iter().map(|&id| {
            let route = tree.get(id).unwrap();
            route.to_tokens(&self.nests, &tree, self.name.clone(), error_name.clone())
        });

        quote! {
            impl<'a> core::convert::TryFrom<&'a str> for #name {
                type Error = <Self as std::str::FromStr>::Err;

                fn try_from(s: &'a str) -> ::std::result::Result<Self, Self::Error> {
                    s.parse()
                }
            }

            impl std::str::FromStr for #name {
                type Err = freya_router::routable::RouteParseError<#error_name>;

                fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                    let route = s;
                    let (route, hash) = route.split_once('#').unwrap_or((route, ""));
                    let (route, query) = route.split_once('?').unwrap_or((route, ""));
                    // Remove any trailing slashes. We parse /route/ and /route in the same way
                    // Note: we don't use trim because it includes more code
                    let route = route.strip_suffix('/').unwrap_or(route);
                    let query = freya_router::exports::urlencoding::decode(query).unwrap_or(query.into());
                    let hash = freya_router::exports::urlencoding::decode(hash).unwrap_or(hash.into());
                    let mut segments = route.split('/').map(|s| freya_router::exports::urlencoding::decode(s).unwrap_or(s.into()));
                    // skip the first empty segment
                    if s.starts_with('/') {
                        let _ = segments.next();
                    } else {
                        // if this route does not start with a slash, it is not a valid route
                        return Err(freya_router::routable::RouteParseError {
                            attempted_routes: Vec::new(),
                        });
                    }
                    let mut errors = Vec::new();

                    #(#tokens)*

                    Err(freya_router::routable::RouteParseError {
                        attempted_routes: errors,
                    })
                }
            }
        }
    }

    fn error_name(&self) -> Ident {
        Ident::new(&(self.name.to_string() + "MatchError"), Span::call_site())
    }

    fn error_type(&self) -> TokenStream2 {
        let match_error_name = self.error_name();

        let mut type_defs = Vec::new();
        let mut error_variants = Vec::new();
        let mut display_match = Vec::new();

        for endpoint in &self.endpoints {
            match endpoint {
                RouteEndpoint::Route(route) => {
                    let route_name = &route.route_name;

                    let error_name = route.error_ident();
                    let route_str = &route.route;
                    let comment = format!(
                        " An error that can occur when trying to parse the route [`{}::{}`] ('{}').",
                        self.name, route_name, route_str
                    );

                    error_variants.push(quote! {
                        #[doc = #comment]
                        #route_name(#error_name)
                    });
                    display_match.push(quote! { Self::#route_name(err) => write!(f, "Route '{}' ('{}') did not match:\n{}", stringify!(#route_name), #route_str, err)? });
                    type_defs.push(route.error_type());
                }
                RouteEndpoint::Redirect(redirect) => {
                    let error_variant = redirect.error_variant();
                    let error_name = redirect.error_ident();
                    let route_str = &redirect.route;
                    let comment = format!(
                        " An error that can occur when trying to parse the redirect '{}'.",
                        route_str.value()
                    );

                    error_variants.push(quote! {
                        #[doc = #comment]
                        #error_variant(#error_name)
                    });
                    display_match.push(quote! { Self::#error_variant(err) => write!(f, "Redirect '{}' ('{}') did not match:\n{}", stringify!(#error_name), #route_str, err)? });
                    type_defs.push(redirect.error_type());
                }
            }
        }

        for nest in &self.nests {
            let error_variant = nest.error_variant();
            let error_name = nest.error_ident();
            let route_str = &nest.route;
            let comment = format!(
                " An error that can occur when trying to parse the nested segment {error_name} ('{route_str}').",
            );

            error_variants.push(quote! {
                #[doc = #comment]
                #error_variant(#error_name)
            });
            display_match.push(quote! { Self::#error_variant(err) => write!(f, "Nest '{}' ('{}') did not match:\n{}", stringify!(#error_name), #route_str, err)? });
            type_defs.push(nest.error_type());
        }

        let comment = format!(
            " An error that can occur when trying to parse the route enum [`{}`].",
            self.name
        );

        quote! {
            #(#type_defs)*

            #[doc = #comment]
            #[allow(non_camel_case_types)]
            #[allow(clippy::derive_partial_eq_without_eq)]
            pub enum #match_error_name {
                #(#error_variants),*
            }

            impl std::fmt::Debug for #match_error_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}({})", stringify!(#match_error_name), self)
                }
            }

            impl std::fmt::Display for #match_error_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#display_match),*
                    }
                    Ok(())
                }
            }
        }
    }

    fn routable_impl(&self) -> TokenStream2 {
        let name = &self.name;
        let site_map = &self.site_map;

        let mut matches = Vec::new();

        // Collect all routes matches
        for route in &self.endpoints {
            if let RouteEndpoint::Route(route) = route {
                matches.push(route.routable_match(&self.layouts, &self.nests));
            }
        }

        quote! {
            impl freya_router::routable::Routable for #name where Self: Clone {
                const SITE_MAP: &'static [freya_router::routable::SiteMapSegment] = &[
                    #(#site_map,)*
                ];

                fn render(&self, level: usize) -> freya_core::prelude::Element {
                    let myself = self.clone();
                    match (level, myself) {
                        #(#matches)*
                        _ => unreachable!()
                    }
                }
            }
        }
    }
}

enum RouteEndpoint {
    Route(Route),
    Redirect(Redirect),
}

struct SiteMapSegment {
    pub segment_type: SegmentType,
    pub children: Vec<SiteMapSegment>,
}

impl SiteMapSegment {
    fn new(segments: &[RouteSegment]) -> Option<Self> {
        let mut current = None;
        // walk backwards through the new segments, adding children as we go
        for segment in segments.iter().rev() {
            let segment_type = segment.into();
            let mut segment = SiteMapSegment {
                segment_type,
                children: Vec::new(),
            };
            // if we have a current segment, add it as a child
            if let Some(current) = current.take() {
                segment.children.push(current)
            }
            current = Some(segment);
        }
        current
    }
}

impl ToTokens for SiteMapSegment {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let segment_type = &self.segment_type;
        let children = if let SegmentType::Child(ty) = &self.segment_type {
            quote! { #ty::SITE_MAP }
        } else {
            let children = self
                .children
                .iter()
                .map(|child| child.to_token_stream())
                .collect::<Vec<_>>();
            quote! {
                &[
                    #(#children,)*
                ]
            }
        };

        tokens.extend(quote! {
            freya_router::routable::SiteMapSegment {
                segment_type: #segment_type,
                children: #children,
            }
        });
    }
}

enum SegmentType {
    Static(String),
    Dynamic(String),
    CatchAll(String),
    Child(Type),
}

impl ToTokens for SegmentType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            SegmentType::Static(s) => {
                tokens.extend(quote! { freya_router::routable::SegmentType::Static(#s) })
            }
            SegmentType::Dynamic(s) => {
                tokens.extend(quote! { freya_router::routable::SegmentType::Dynamic(#s) })
            }
            SegmentType::CatchAll(s) => {
                tokens.extend(quote! { freya_router::routable::SegmentType::CatchAll(#s) })
            }
            SegmentType::Child(_) => {
                tokens.extend(quote! { freya_router::routable::SegmentType::Child })
            }
        }
    }
}

impl<'a> From<&'a RouteSegment> for SegmentType {
    fn from(value: &'a RouteSegment) -> Self {
        match value {
            RouteSegment::Static(s) => SegmentType::Static(s.to_string()),
            RouteSegment::Dynamic(s, _) => SegmentType::Dynamic(s.to_string()),
            RouteSegment::CatchAll(s, _) => SegmentType::CatchAll(s.to_string()),
        }
    }
}
