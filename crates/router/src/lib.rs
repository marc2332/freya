// cannot use forbid, because props derive macro generates #[allow(missing_docs)]
#![allow(non_snake_case)]

mod memory;

pub mod navigation;
pub mod routable;

/// Components interacting with the router.
pub mod components {
    mod outlet;
    pub use outlet::*;

    mod router;
    pub use router::*;

    #[doc(hidden)]
    pub mod child_router;

    pub mod activable_route;
    pub use activable_route::*;

    pub mod native_router;
    pub use native_router::*;
}

mod contexts {
    pub(crate) mod navigator;
    pub(crate) mod outlet;
    pub use outlet::{
        use_outlet_context,
        OutletContext,
    };
    pub(crate) mod router;
    pub use navigator::*;
    pub(crate) use router::*;
    pub use router::{
        GenericRouterContext,
        ParseRouteError,
        RouterContext,
    };
}

mod router_cfg;

/// Hooks for interacting with the router in components.
pub mod hooks {
    mod use_router;
    pub use use_router::*;

    mod use_route;
    pub use use_route::*;

    mod use_navigator;
    pub use use_navigator::*;
}

pub use hooks::router;

/// A collection of useful items most applications might need.
pub mod prelude {
    pub use freya_router_macro::Routable;

    pub use crate::{
        components::{
            ActivableRoute,
            NativeRouter,
            Outlet,
            Router,
            RouterProps,
        },
        contexts::*,
        hooks::*,
        memory::MemoryHistory,
        navigation::*,
        routable::*,
        router_cfg::RouterConfig,
    };

    #[doc(hidden)]
    /// A component with props used in the macro
    pub trait HasProps {
        /// The props type of the component.
        type Props;
    }

    impl<P> HasProps for dioxus_lib::prelude::Component<P> {
        type Props = P;
    }
}

mod utils {
    pub(crate) mod use_router_internal;
}

#[doc(hidden)]
pub mod exports {
    pub use urlencoding;
}
