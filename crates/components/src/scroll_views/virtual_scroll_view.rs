#![allow(clippy::type_complexity)]

use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::{keyboard::Key, KeyboardEvent, MouseEvent, WheelEvent};
use freya_hooks::{
    use_applied_theme, use_focus, use_node, ScrollBarThemeWith, ScrollViewThemeWith,
};
use std::ops::Range;

use crate::{
    get_container_size, get_corrected_scroll_position, get_scroll_position_from_cursor,
    get_scroll_position_from_wheel, get_scrollbar_pos_and_size, is_scrollbar_visible,
    manage_key_event, Axis, ScrollBar, ScrollThumb, SCROLL_SPEED_MULTIPLIER,
};

/// Properties for the [`VirtualScrollView`] component.
#[derive(Props, Clone)]
pub struct VirtualScrollViewProps<
    Builder: 'static + Clone + Fn(usize, &Option<BuilderArgs>) -> Element,
    BuilderArgs: Clone + 'static + PartialEq = (),
> {
    /// Theme override.
    pub theme: Option<ScrollViewThemeWith>,
    /// Theme override for the scrollbars.
    pub scrollbar_theme: Option<ScrollBarThemeWith>,
    /// Quantity of items in the VirtualScrollView.
    pub length: usize,
    /// Size of the items, height for vertical direction and width for horizontal.
    pub item_size: f32,
    /// The item builder function.
    pub builder: Builder,
    /// The values for the item builder function.
    #[props(into)]
    pub builder_args: Option<BuilderArgs>,
    /// Direction of the VirtualScrollView, `vertical` or `horizontal`.
    #[props(default = "vertical".to_string(), into)]
    pub direction: String,
    /// Show the scrollbar, visible by default.
    #[props(default = true, into)]
    pub show_scrollbar: bool,
    /// Enable scrolling with arrow keys.
    #[props(default = true, into)]
    pub scroll_with_arrows: bool,
    /// Cache elements or not, changing `builder_args` will invalidate the cache if enabled.
    /// Default is `true`.
    #[props(default = true, into)]
    pub cache_elements: bool,
}

impl<
        BuilderArgs: Clone + PartialEq,
        Builder: Clone + Fn(usize, &Option<BuilderArgs>) -> Element,
    > PartialEq for VirtualScrollViewProps<Builder, BuilderArgs>
{
    fn eq(&self, other: &Self) -> bool {
        self.theme == other.theme
            && self.length == other.length
            && self.item_size == other.item_size
            && self.direction == other.direction
            && self.show_scrollbar == other.show_scrollbar
            && self.scroll_with_arrows == other.scroll_with_arrows
            && self.builder_args == other.builder_args
    }
}

fn get_render_range(
    viewport_size: f32,
    scroll_position: f32,
    item_size: f32,
    item_length: f32,
) -> Range<usize> {
    let render_index_start = (-scroll_position) / item_size;
    let potentially_visible_length = viewport_size / item_size;
    let remaining_length = item_length - render_index_start;

    let render_index_end = if remaining_length <= potentially_visible_length {
        item_length
    } else {
        render_index_start + potentially_visible_length
    };

    render_index_start as usize..(render_index_end as usize)
}

/// One-direction scrollable area that dynamically builds and renders items based in their size and current available size,
/// this is intended for apps using large sets of data that need good performance.
///
/// Use cases: text editors, chats, etc.
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// # use std::rc::Rc;
/// fn app() -> Element {
///     rsx!(
///         VirtualScrollView {
///             show_scrollbar: true,
///             length: 5,
///             item_size: 80.0,
///             direction: "vertical",
///             builder: move |i, _other_args: &Option<()>| {
///                 rsx! {
///                     label {
///                         key: "{i}",
///                         height: "80",
///                         "Number {i}"
///                     }
///                 }
///             }
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn VirtualScrollView<
    Builder: Clone + Fn(usize, &Option<BuilderArgs>) -> Element,
    BuilderArgs: Clone + PartialEq,
>(
    props: VirtualScrollViewProps<Builder, BuilderArgs>,
) -> Element {
    let mut clicking_scrollbar = use_signal::<Option<(Axis, f64)>>(|| None);
    let mut clicking_shift = use_signal(|| false);
    let mut clicking_alt = use_signal(|| false);
    let mut scrolled_y = use_signal(|| 0);
    let mut scrolled_x = use_signal(|| 0);
    let (node_ref, size) = use_node();
    let mut focus = use_focus();
    let theme = use_applied_theme!(&props.theme, scroll_view);
    let scrollbar_theme = use_applied_theme!(&props.scrollbar_theme, scroll_bar);

    let padding = &theme.padding;
    let user_container_width = &theme.width;
    let user_container_height = &theme.height;
    let user_direction = &props.direction;
    let show_scrollbar = props.show_scrollbar;
    let items_length = props.length;
    let items_size = props.item_size;
    let scroll_with_arrows = props.scroll_with_arrows;

    let inner_size = items_size + (items_size * items_length as f32);

    let vertical_scrollbar_is_visible = user_direction != "horizontal"
        && is_scrollbar_visible(show_scrollbar, inner_size, size.area.height());
    let horizontal_scrollbar_is_visible = user_direction != "vertical"
        && is_scrollbar_visible(show_scrollbar, inner_size, size.area.width());

    let container_width = get_container_size(vertical_scrollbar_is_visible, &scrollbar_theme.size);
    let container_height =
        get_container_size(horizontal_scrollbar_is_visible, &scrollbar_theme.size);

    let corrected_scrolled_y =
        get_corrected_scroll_position(inner_size, size.area.height(), *scrolled_y.read() as f32);
    let corrected_scrolled_x =
        get_corrected_scroll_position(inner_size, size.area.width(), *scrolled_x.read() as f32);

    let (scrollbar_y, scrollbar_height) =
        get_scrollbar_pos_and_size(inner_size, size.area.height(), corrected_scrolled_y);
    let (scrollbar_x, scrollbar_width) =
        get_scrollbar_pos_and_size(inner_size, size.area.width(), corrected_scrolled_x);

    // Moves the Y axis when the user scrolls in the container
    let onwheel = move |e: WheelEvent| {
        let speed_multiplier = if *clicking_alt.peek() {
            SCROLL_SPEED_MULTIPLIER
        } else {
            1.0
        };

        let wheel_movement = e.get_delta_y() as f32 * speed_multiplier;

        if *clicking_shift.peek() {
            let scroll_position_x = get_scroll_position_from_wheel(
                wheel_movement,
                inner_size,
                size.area.width(),
                corrected_scrolled_x,
            );

            // Only scroll when there is still area to scroll
            if *scrolled_x.peek() != scroll_position_x {
                e.stop_propagation();
                *scrolled_x.write() = scroll_position_x;
            } else {
                return;
            }
        } else {
            let scroll_position_y = get_scroll_position_from_wheel(
                wheel_movement,
                inner_size,
                size.area.height(),
                corrected_scrolled_y,
            );

            // Only scroll when there is still area to scroll
            if *scrolled_y.peek() != scroll_position_y {
                e.stop_propagation();
                *scrolled_y.write() = scroll_position_y;
            } else {
                return;
            }
        }

        focus.focus();
    };

    // Drag the scrollbars
    let onmouseover = move |e: MouseEvent| {
        let clicking_scrollbar = clicking_scrollbar.peek();

        if let Some((Axis::Y, y)) = *clicking_scrollbar {
            let coordinates = e.get_element_coordinates();
            let cursor_y = coordinates.y - y - size.area.min_y() as f64;

            let scroll_position =
                get_scroll_position_from_cursor(cursor_y as f32, inner_size, size.area.height());

            *scrolled_y.write() = scroll_position;
        } else if let Some((Axis::X, x)) = *clicking_scrollbar {
            let coordinates = e.get_element_coordinates();
            let cursor_x = coordinates.x - x - size.area.min_x() as f64;

            let scroll_position =
                get_scroll_position_from_cursor(cursor_x as f32, inner_size, size.area.width());

            *scrolled_x.write() = scroll_position;
        }

        if clicking_scrollbar.is_some() {
            focus.focus();
        }
    };

    let onkeydown = move |e: KeyboardEvent| {
        match &e.key {
            Key::Shift => {
                clicking_shift.set(true);
            }
            Key::Alt => {
                clicking_alt.set(true);
            }
            k => {
                if !focus.is_focused() {
                    return;
                }

                if !scroll_with_arrows
                    && (k == &Key::ArrowUp
                        || k == &Key::ArrowRight
                        || k == &Key::ArrowDown
                        || k == &Key::ArrowLeft)
                {
                    return;
                }

                let x = corrected_scrolled_x;
                let y = corrected_scrolled_y;
                let inner_height = inner_size;
                let inner_width = inner_size;
                let viewport_height = size.area.height();
                let viewport_width = size.area.width();

                let (x, y) = manage_key_event(
                    e,
                    (x, y),
                    inner_height,
                    inner_width,
                    viewport_height,
                    viewport_width,
                );

                scrolled_x.set(x as i32);
                scrolled_y.set(y as i32);
            }
        };
    };

    let onkeyup = move |e: KeyboardEvent| {
        if e.key == Key::Shift {
            clicking_shift.set(false);
        } else if e.key == Key::Alt {
            clicking_alt.set(false);
        }
    };

    // Mark the Y axis scrollbar as the one being dragged
    let onmousedown_y = move |e: MouseEvent| {
        let coordinates = e.get_element_coordinates();
        *clicking_scrollbar.write() = Some((Axis::Y, coordinates.y));
    };

    // Mark the X axis scrollbar as the one being dragged
    let onmousedown_x = move |e: MouseEvent| {
        let coordinates = e.get_element_coordinates();
        *clicking_scrollbar.write() = Some((Axis::X, coordinates.x));
    };

    // Unmark any scrollbar
    let onclick = move |_: MouseEvent| {
        if clicking_scrollbar.peek().is_some() {
            *clicking_scrollbar.write() = None;
        }
    };

    let horizontal_scrollbar_size = if horizontal_scrollbar_is_visible {
        &scrollbar_theme.size
    } else {
        "0"
    };

    let vertical_scrollbar_size = if vertical_scrollbar_is_visible {
        &scrollbar_theme.size
    } else {
        "0"
    };

    let (viewport_size, scroll_position) = if user_direction == "vertical" {
        (size.area.height(), corrected_scrolled_y)
    } else {
        (size.area.width(), corrected_scrolled_x)
    };

    // Calculate from what to what items must be rendered
    let render_range = get_render_range(
        viewport_size,
        scroll_position,
        items_size,
        items_length as f32,
    );

    let children = if props.cache_elements {
        let children = use_memo(use_reactive(
            &(render_range, props.builder_args),
            move |(render_range, builder_args)| {
                render_range
                    .clone()
                    .map(|i| (props.builder)(i, &builder_args))
                    .collect::<Vec<Element>>()
            },
        ));
        rsx!({ children.read().iter() })
    } else {
        let children = render_range.map(|i| (props.builder)(i, &props.builder_args));
        rsx!({ children })
    };

    let is_scrolling_x = clicking_scrollbar
        .read()
        .as_ref()
        .map(|f| f.0 == Axis::X)
        .unwrap_or_default();
    let is_scrolling_y = clicking_scrollbar
        .read()
        .as_ref()
        .map(|f| f.0 == Axis::Y)
        .unwrap_or_default();

    rsx!(
        rect {
            role: "scrollView",
            overflow: "clip",
            direction: "horizontal",
            width: "{user_container_width}",
            height: "{user_container_height}",
            onglobalclick: onclick,
            onglobalmouseover: onmouseover,
            onkeydown,
            onkeyup,
            rect {
                direction: "vertical",
                width: "{container_width}",
                height: "{container_height}",
                rect {
                    overflow: "clip",
                    padding: "{padding}",
                    height: "100%",
                    width: "100%",
                    direction: "{user_direction}",
                    reference: node_ref,
                    onwheel: onwheel,
                    {children}
                }
                ScrollBar {
                    width: "100%",
                    height: "{horizontal_scrollbar_size}",
                    offset_x: "{scrollbar_x}",
                    clicking_scrollbar: is_scrolling_x,
                    theme: props.scrollbar_theme.clone(),
                    ScrollThumb {
                        clicking_scrollbar: is_scrolling_x,
                        onmousedown: onmousedown_x,
                        width: "{scrollbar_width}",
                        height: "100%",
                        theme: props.scrollbar_theme.clone(),
                    }
                }
            }
            ScrollBar {
                width: "{vertical_scrollbar_size}",
                height: "100%",
                offset_y: "{scrollbar_y}",
                clicking_scrollbar: is_scrolling_y,
                theme: props.scrollbar_theme.clone(),
                ScrollThumb {
                    clicking_scrollbar: is_scrolling_y,
                    onmousedown: onmousedown_y,
                    width: "100%",
                    height: "{scrollbar_height}",
                    theme: props.scrollbar_theme,
                }
            }
        }
    )
}

#[cfg(test)]
mod test {
    use freya::prelude::*;
    use freya_testing::prelude::*;

    #[tokio::test]
    pub async fn virtual_scroll_view_wheel() {
        fn virtual_scroll_view_wheel_app() -> Element {
            let values = use_signal(|| ["Hello, World!"].repeat(30));

            rsx!(VirtualScrollView {
                length: values.read().len(),
                item_size: 50.0,
                direction: "vertical",
                builder: move |index, _: &Option<()>| {
                    let value = values.read()[index];
                    rsx! {
                        label {
                            key: "{index}",
                            height: "50",
                            "{index} {value}"
                        }
                    }
                }
            })
        }

        let mut utils = launch_test(virtual_scroll_view_wheel_app);
        let root = utils.root();

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 10);

        // Check that visible items are from indexes 0 to 10, because 500 / 50 = 10.
        for (n, i) in (0..10).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }

        utils.push_event(PlatformEvent::Wheel {
            name: EventName::Wheel,
            scroll: (0., -300.).into(),
            cursor: (5., 5.).into(),
        });

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 10);

        // It has scrolled 300 pixels, which equals to 6 items since because 300 / 50 = 6
        // So we must start checking from 6 to +10, 16 in this case because 6 + 10 = 16
        for (n, i) in (6..16).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }
    }

    #[tokio::test]
    pub async fn virtual_scroll_view_scrollbar() {
        fn virtual_scroll_view_scrollar_app() -> Element {
            let values = use_signal(|| ["Hello, World!"].repeat(30));

            rsx!(VirtualScrollView {
                length: values.read().len(),
                item_size: 50.0,
                direction: "vertical",
                builder: move |index, _: &Option<()>| {
                    let value = values.read()[index];
                    rsx! {
                        label {
                            key: "{index}",
                            height: "50",
                            "{index} {value}"
                        }
                    }
                }
            })
        }

        let mut utils = launch_test(virtual_scroll_view_scrollar_app);
        let root = utils.root();

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 10);

        // Check that visible items are from indexes 0 to 10, because 500 / 50 = 10.
        for (n, i) in (0..10).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }

        // Simulate the user dragging the scrollbar
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (490., 20.).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseDown,
            cursor: (490., 20.).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::MouseOver,
            cursor: (490., 320.).into(),
            button: Some(MouseButton::Left),
        });
        utils.push_event(PlatformEvent::Mouse {
            name: EventName::Click,
            cursor: (490., 320.).into(),
            button: Some(MouseButton::Left),
        });

        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 10);

        // It has dragged the scrollbar 300 pixels
        for (n, i) in (18..28).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }

        // Scroll up with arrows
        for _ in 0..10 {
            utils.push_event(PlatformEvent::Keyboard {
                name: EventName::KeyDown,
                key: Key::ArrowUp,
                code: Code::ArrowUp,
                modifiers: Modifiers::default(),
            });
            utils.wait_for_update().await;
        }

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 10);

        for (n, i) in (0..10).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }

        // Scroll to the bottom with arrows
        utils.push_event(PlatformEvent::Keyboard {
            name: EventName::KeyDown,
            key: Key::End,
            code: Code::End,
            modifiers: Modifiers::default(),
        });
        utils.wait_for_update().await;
        utils.wait_for_update().await;

        let content = root.get(0).get(0).get(0);
        assert_eq!(content.children_ids().len(), 9);

        for (n, i) in (21..30).enumerate() {
            let child = content.get(n);
            assert_eq!(
                child.get(0).text(),
                Some(format!("{i} Hello, World!").as_str())
            );
        }
    }
}
