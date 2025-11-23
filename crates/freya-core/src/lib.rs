pub mod accessibility;
pub mod animation_clock;
pub mod current_context;
pub mod cursor;
pub mod data;
pub mod diff_key;
pub mod element;
pub mod elements;
pub mod event_handler;
pub mod event_notifier;
pub mod events;
pub mod extended_hashmap;
pub mod helpers;
pub mod hooks;
pub mod layers;
pub mod lifecycle;
pub mod node_id;
pub mod notify;
pub mod path_element;
pub mod platform_state;
pub mod reactive_context;
pub mod render_pipeline;
pub mod rendering_ticker;
pub mod runner;
pub mod scope;
pub mod scope_id;
pub mod style;
pub mod text_cache;
pub mod tree;
pub mod tree_layout_adapter;
pub mod user_event;

/// Used by all end users.
pub mod prelude {
    pub use bytes::Bytes;
    pub use cursor_icon::CursorIcon;
    pub use keyboard_types::{
        Code,
        Key,
        Modifiers,
    };

    pub use crate::{
        accessibility::{
            focus::*,
            focus_strategy::*,
            focusable::*,
            id::{
                AccessibilityId,
                AccessibilityRole,
            },
            screen_reader::*,
        },
        animation_clock::AnimationClock,
        cursor::*,
        data::*,
        diff_key::DiffKey,
        element::RenderContext,
        element::{
            Element,
            FpRender,
            IntoElement,
            Render,
            RenderKey,
            RenderOwned,
        },
        elements::{
            extensions::*,
            image::{
                AspectRatio,
                ImageCover,
                // The image element is hidden on purpose as its a "low level" element, users should rather use the `ImageViewer` component.
                SamplingMode,
            },
            label::{
                Label,
                TextWidth,
                label,
            },
            paragraph::{
                Paragraph,
                ParagraphHolder,
                Span,
                paragraph,
            },
            rect::{
                Rect,
                rect,
            },
            svg::{
                Svg,
                svg,
            },
        },
        event_handler::{
            Callback,
            EventHandler,
        },
        event_notifier::EventNotifier,
        events::data::*,
        events::*,
        hooks::use_id::*,
        lifecycle::{
            base::*,
            context::*,
            effect::*,
            future_task::*,
            memo::*,
            reactive::*,
            state::*,
            task::*,
        },
        platform_state::*,
        reactive_context::ReactiveContext,
        rendering_ticker::RenderingTicker,
        style::{
            border::*,
            color::*,
            corner_radius::*,
            fill::*,
            font_slant::*,
            font_weight::*,
            font_width::*,
            gradient::*,
            shadow::*,
            text_align::*,
            text_height::*,
            text_overflow::*,
            text_shadow::*,
        },
        user_event::UserEvent,
    };
}

/// Used by renderers such as freya-testing, freya-winit or just integration crates.
pub mod integration {
    pub use rustc_hash::*;

    pub use crate::{
        accessibility::{
            dirty_nodes::*,
            focus_strategy::*,
            id::*,
            screen_reader::*,
            tree::*,
        },
        animation_clock::AnimationClock,
        data::*,
        element::*,
        elements::{
            extensions::*,
            label::LabelElement,
        },
        event_notifier::*,
        events::{
            data::*,
            executor::*,
            measurer::*,
            name::*,
            platform::*,
        },
        lifecycle::state::State,
        node_id::NodeId,
        platform_state::*,
        render_pipeline::RenderPipeline,
        rendering_ticker::*,
        runner::Runner,
        scope_id::ScopeId,
        style::default_fonts::default_fonts,
        tree::{
            DiffModifies,
            Tree,
        },
        user_event::*,
    };
}
