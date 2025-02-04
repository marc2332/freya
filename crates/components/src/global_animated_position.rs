use std::{
    collections::HashMap,
    hash::Hash,
    sync::Arc,
    time::Duration,
};

use dioxus::prelude::*;
use dioxus_core::AttributeValue;
use freya_core::custom_attributes::{
    CustomAttributeValues,
    NodeReference,
    NodeReferenceLayout,
};
use freya_elements as dioxus_elements;
use freya_hooks::{
    use_animation_with_dependencies,
    AnimDirection,
    AnimNum,
    Ease,
    Function,
};
use tokio::sync::watch::channel;
use torin::prelude::Area;

#[derive(Clone)]
pub struct GlobalAnimatedPositions<T: Clone + PartialEq + 'static> {
    pub ids: Signal<HashMap<T, Area>>,
}

type InitCurrPrevSignals = (
    AttributeValue,
    Signal<Option<Area>>,
    ReadOnlySignal<Option<Area>>,
    ReadOnlySignal<Option<Area>>,
);

fn use_node_init_curr_prev<T: Clone + PartialEq + Hash + Eq + 'static>(
    id: T,
) -> InitCurrPrevSignals {
    let (tx, init_signal, curr_signal, prev_signal) = use_hook(|| {
        let (tx, mut rx) = channel::<NodeReferenceLayout>(NodeReferenceLayout::default());
        let mut ctx = consume_context::<GlobalAnimatedPositions<T>>();
        let init_signal = Signal::new(ctx.ids.write().remove(&id));
        let mut curr_signal = Signal::new(None);
        let mut prev_signal = Signal::new(None);

        spawn(async move {
            while rx.changed().await.is_ok() {
                if *curr_signal.peek() != Some(rx.borrow().clone().area) {
                    prev_signal.set(curr_signal());
                    curr_signal.set(Some(rx.borrow().clone().area));
                    ctx.ids.write().insert(id.clone(), curr_signal().unwrap());
                }
            }
        });

        (Arc::new(tx), init_signal, curr_signal, prev_signal)
    });

    (
        AttributeValue::any_value(CustomAttributeValues::Reference(NodeReference(tx))),
        init_signal,
        curr_signal.into(),
        prev_signal.into(),
    )
}

#[derive(Props, PartialEq, Clone)]
pub struct GlobalAnimatedPositionProvider {
    children: Element,
}

#[allow(non_snake_case)]
pub fn GlobalAnimatedPositionProvider<T: Clone + PartialEq + Hash + Eq + 'static>(
    GlobalAnimatedPositionProvider { children }: GlobalAnimatedPositionProvider,
) -> Element {
    use_context_provider(|| GlobalAnimatedPositions::<T> {
        ids: Signal::default(),
    });

    children
}

/// Animate an element position across time and space.
///
/// For that, the element must have an unique ID.
///
/// It must also be descendant of a [GlobalAnimatedPositionProvider].
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app() -> Element {
///     rsx!(
///         GlobalAnimatedPositionProvider::<i32> {
///             GlobalAnimatedPosition {
///                 id: 0,
///                 width: "100",
///                 height: "25",
///                 label {
///                     "Click this"
///                 }
///             }
///         }
///     )
/// }
/// ```
#[component]
pub fn GlobalAnimatedPosition<T: Clone + PartialEq + Hash + Eq + 'static>(
    children: Element,
    width: String,
    height: String,
    /// Unique ID to identify this element.
    id: T,
    #[props(default = Function::default())] function: Function,
    #[props(default = Duration::from_millis(250))] duration: Duration,
    #[props(default = Ease::default())] ease: Ease,
) -> Element {
    let mut render_element = use_signal(|| false);
    let (reference, mut init_size, size, old_size) = use_node_init_curr_prev(id);

    let animations = use_animation_with_dependencies(
        &(function, duration, ease),
        move |_conf, (function, duration, ease)| {
            let old_size = init_size.peek().unwrap_or(old_size().unwrap_or_default());
            let size = size().unwrap_or_default();
            (
                AnimNum::new(size.origin.x, old_size.origin.x)
                    .duration(duration)
                    .ease(ease)
                    .function(function),
                AnimNum::new(size.origin.y, old_size.origin.y)
                    .duration(duration)
                    .ease(ease)
                    .function(function),
            )
        },
    );

    use_effect(move || {
        if animations.is_running() {
            render_element.set(true);
        }
    });

    use_effect(move || {
        let has_size = size.read().is_some();
        let has_init_size = init_size.read().is_some();
        let has_old_size = old_size.read().is_some();
        if has_size && (has_old_size || has_init_size) {
            animations.run(AnimDirection::Reverse);
        } else if has_size {
            render_element.set(true);
        }
    });

    use_effect(move || {
        if animations.has_run_yet() {
            // Remove the init size when the first animation starts
            // This way the next time it is animated it will use the prev size
            init_size.set(None);
        }
    });

    let animations = animations.get();
    let (offset_x, offset_y) = &*animations.read();
    let offset_x = offset_x.read();
    let offset_y = offset_y.read();

    rsx!(
        rect {
            reference,
            width: "{width}",
            height: "{height}",
            rect {
                width: "0",
                height: "0",
                offset_x: "{offset_x}",
                offset_y: "{offset_y}",
                position: "global",
                if render_element() {
                    rect {
                        width: "{size.read().as_ref().unwrap().width()}",
                        height: "{size.read().as_ref().unwrap().height()}",
                        {children}
                    }
                }
            }
        }
    )
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn global_animated_position() {
        fn global_animated_position_app() -> Element {
            let mut padding = use_signal(|| (100., 100.));

            rsx!(
                GlobalAnimatedPositionProvider::<i32> {
                    rect {
                        padding: "{padding().0} {padding().1}",
                        onclick: move |_| {
                            padding.write().0 += 10.;
                            padding.write().1 += 10.;
                        },
                        GlobalAnimatedPosition {
                            width: "50",
                            height: "50",
                            function: Function::Linear,
                            id: 0
                        }
                    }
                }

            )
        }

        let mut utils = launch_test(global_animated_position_app);

        // Disable event loop ticker
        utils.config().event_loop_ticker = false;

        let root = utils.root();
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let get_positions = || {
            root.get(0)
                .get(0)
                .get(0)
                .get(0)
                .layout()
                .unwrap()
                .area
                .origin
        };

        assert_eq!(get_positions().x, 100.);
        assert_eq!(get_positions().y, 100.);

        utils.click_cursor((5.0, 5.0)).await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;
        tokio::time::sleep(Duration::from_millis(125)).await;
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        assert!(get_positions().x < 106.);
        assert!(get_positions().x > 105.);

        assert!(get_positions().y < 106.);
        assert!(get_positions().y > 105.);

        utils.config().event_loop_ticker = true;

        utils.wait_for_update().await;
        tokio::time::sleep(Duration::from_millis(125)).await;
        utils.wait_for_update().await;

        assert_eq!(get_positions().x, 110.);
    }
}
