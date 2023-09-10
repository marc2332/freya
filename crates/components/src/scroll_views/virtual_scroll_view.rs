use std::ops::Range;

use dioxus::prelude::*;
use freya_common::NodeReferenceLayout;

use freya_elements::elements as dioxus_elements;
use freya_elements::events::{keyboard::Key, KeyboardEvent, MouseEvent, WheelEvent};
use freya_hooks::{use_focus, use_node};

use crate::{
    Axis, get_container_size, get_corrected_scroll_position,
    get_scroll_position_from_cursor, get_scroll_position_from_wheel, get_scrollbar_pos_and_size,
    is_scrollbar_visible, manage_key_event, SCROLL_SPEED_MULTIPLIER, ScrollBar, SCROLLBAR_SIZE, ScrollThumb,
};

type BuilderFunction<'a, T> = dyn Fn(
    (
        usize,
        usize,
        Scope<'a, VirtualScrollViewProps<'a, T>>,
        &'a Option<T>,
    ),
) -> LazyNodes<'a, 'a>;

pub enum ItemSize {
    Fixed(f32),
    PerItem(Box<dyn Fn(usize) -> f32>),
    Dynamic,
}

#[derive(Debug)]
enum Node {
    LeafRange(LeafRangeNode),
    Leaf(LeafNode),
    Inner(InnerNode),
}

#[derive(Debug)]
struct LeafNode {
    index: usize,
    size: f32,
}

#[derive(Debug)]
struct LeafRangeNode {
    range: Range<usize>,
}

#[derive(Debug)]
struct InnerNode {
    range: Range<usize>,
    total_real_size: f32,
    range_node_count: usize,
    children: Vec<Node>,
}

#[derive(Debug)]
struct SizeCache {
    nodes: Vec<Node>,
    chunk_size: usize,
}

impl Node {
    fn contains_index(&self, index: usize) -> bool {
        match self {
            Node::LeafRange(node) => {
                node.range.contains(&index)
            }
            Node::Leaf(node) => {
                node.index == index
            }
            Node::Inner(node) => {
                node.range.contains(&index)
            }
        }
    }

    fn is_inner(&self) -> bool {
        match self {
            Node::Inner(_) => true,
            _ => false
        }
    }

    fn is_leaf(&self) -> bool {
        match self {
            Node::Leaf(_) => true,
            _ => false
        }
    }

    fn is_leaf_range(&self) -> bool {
        match self {
            Node::LeafRange(_) => true,
            _ => false
        }
    }

    fn as_inner(&self) -> Option<&InnerNode> {
        match self {
            Node::Inner(inner) => Some(inner),
            _ => None
        }
    }

    fn as_leaf_range(&self) -> Option<&LeafRangeNode> {
        match self {
            Node::LeafRange(leaf_range) => Some(leaf_range),
            _ => None
        }
    }

    fn as_inner_mut(&mut self) -> Option<&mut InnerNode> {
        match self {
            Node::Inner(inner) => Some(inner),
            _ => None
        }
    }

    fn as_leaf_mut(&mut self) -> Option<&mut LeafNode> {
        match self {
            Node::Leaf(leaf) => Some(leaf),
            _ => None
        }
    }

    fn as_leaf_range_mut(&mut self) -> Option<&mut LeafRangeNode> {
        match self {
            Node::LeafRange(leaf_range) => Some(leaf_range),
            _ => None
        }
    }

    fn min_index(&self) -> usize {
        match self {
            Node::LeafRange(v) => {v.range.start}
            Node::Leaf(v) => {v.index}
            Node::Inner(v) => {v.range.start}
        }
    }

    fn max_index(&self) -> usize {
        match self {
            Node::LeafRange(v) => {v.range.end-1}
            Node::Leaf(v) => {v.index}
            Node::Inner(v) => {v.range.end-1}
        }
    }

    fn get_real_size(&self) -> f32 {
        match self {
            Node::LeafRange(_) => {0.0}
            Node::Leaf(v) => {v.size}
            Node::Inner(v) => {v.total_real_size}
        }
    }

    fn get_leaf_range_node_count(&self) -> usize {
        match self {
            Node::LeafRange(v) => {v.range.len()}
            Node::Leaf(_) => {0}
            Node::Inner(v) => {v.range_node_count}
        }
    }
}

impl LeafRangeNode {
    fn insert(&mut self, index: usize, value: f32) -> (LeafNode, Option<LeafRangeNode>) {
        assert!(self.range.contains(&index));
        let end = self.range.end;
        self.range.end = index;
        (LeafNode {
            size: value,
            index,
        }, if index+1 < end {
            Some(LeafRangeNode {
                range: index + 1..end,
            })
        } else {
            None
        })
    }
}

impl InnerNode {
    fn insert(&mut self, index: usize, value: f32, chunk_size: usize) -> (bool, Option<InnerNode>) {
        assert!(self.range.contains(&index));
        let child_index = self.children.iter().enumerate().find(|(_, node)| {
            node.contains_index(index)
        }).unwrap().0;

        let dirty = if self.children[child_index].is_inner() {
            let node = self.children[child_index].as_inner_mut().unwrap();
            let old_size = node.total_real_size;
            let old_range_count = node.range_node_count;
            let (dirty, new_node) = node.insert(index, value, chunk_size);
            self.total_real_size += node.total_real_size - old_size;
            self.range_node_count += node.range_node_count;
            self.range_node_count -= old_range_count;
            if let Some(new_node) = new_node {
                self.children.insert(child_index+1, Node::Inner(new_node));
            }
            dirty
        } else if self.children[child_index].is_leaf() {
            let node = self.children[child_index].as_leaf_mut().unwrap();
            let old_size = node.size;
            let dirty = node.size != value;
            node.size = value;
            self.total_real_size += value - old_size;
            dirty
        } else if self.children[child_index].is_leaf_range() {
            let node = self.children[child_index].as_leaf_range_mut().unwrap();
            let old_start = node.range.start;
            let old_end = node.range.end;
            let (leaf, leaf_range) = node.insert(index, value);
            let has_leaf_range = leaf_range.is_some();
            self.total_real_size += value;
            let additional_leaf_range_count = if has_leaf_range { 1 } else { 0 };
            self.range_node_count -= 1;
            if self.children.len() + 1 + additional_leaf_range_count <= chunk_size {
                self.children.insert(child_index + 1, Node::Leaf(leaf));
                if has_leaf_range {
                    self.children.insert(child_index + 2, Node::LeafRange(leaf_range.unwrap()));
                }
                if self.children[child_index].as_leaf_range_mut().unwrap().range.len() == 0 {
                    self.children.remove(child_index);
                }
            } else {
                let node = std::mem::replace(&mut self.children[child_index], Node::Inner(InnerNode {
                    range: old_start..old_end,
                    total_real_size: value,
                    range_node_count: old_end-old_start-1,
                    children: vec![],
                }));
                let new_node = self.children[child_index].as_inner_mut().unwrap();
                if node.as_leaf_range().unwrap().range.len() != 0 {
                    new_node.children.push(node);
                }
                new_node.children.push(Node::Leaf(leaf));
                if has_leaf_range {
                    new_node.children.push(Node::LeafRange(leaf_range.unwrap()));
                }
            }
            true
        } else {
            unreachable!()
        };

        if self.children.len() > chunk_size {
            let new_inner = self.split_children();
            (dirty, Some(new_inner))
        } else {
            (dirty, None)
        }
    }

    fn split_children(&mut self) -> Self {
        let children2 = self.children.split_off(self.children.len() / 2);
        let new_inner = InnerNode {
            range: children2.first().unwrap().min_index()..children2.last().unwrap().max_index()+1,
            total_real_size: children2.iter().map(Node::get_real_size).sum(),
            range_node_count: children2.iter().map(Node::get_leaf_range_node_count).sum(),
            children: vec![],
        };
        self.total_real_size -= new_inner.total_real_size;
        self.range = self.range.start..self.children.last().unwrap().max_index()+1;
        self.range_node_count -= new_inner.range_node_count;
        new_inner
    }
    fn get_total_size(&self, range_size: f32) -> f32 {
        self.total_real_size + self.range_node_count as f32 * range_size
    }

    fn largest_less_than(&self, value: f32, range_size: f32) -> (usize, f32) {
        let total_size = self.get_total_size(range_size);
        assert!(total_size >= value);
        let mut total = 0.0;
        let mut index = None;
        for (i, node) in self.children.iter().enumerate() {
            let size = match node {
                Node::LeafRange(v) => {
                    v.range.len() as f32 * range_size
                }
                Node::Leaf(v) => {
                    v.size
                }
                Node::Inner(v) => {
                    v.get_total_size(range_size)
                }
            };
            if total + size > value {
                index = Some(i);
                break;
            } else {
                total += size;
            }
        }
        let remaining = value - total;
        if let Some(index) = index {
            match &self.children[index] {
                Node::LeafRange(v) => {
                    (v.range.start + (remaining / range_size).floor() as usize, remaining)
                }
                Node::Leaf(v) => { (v.index, remaining) }
                Node::Inner(v) => { v.largest_less_than(remaining, range_size) }
            }
        } else {
            (self.range.end, remaining)
        }
    }
}

impl SizeCache {
    fn new(length: usize, item_size: &ItemSize, chunk_size: Option<usize>) -> Self {
        let chunk_size = if let Some(size) = chunk_size {
            size
        } else {
            100
        };
        let mut tree = SizeCache {
            chunk_size,
            nodes: vec![Node::Inner(InnerNode {
                range: 0..length,
                total_real_size: 0.0,
                range_node_count: length,
                children: vec![Node::LeafRange(LeafRangeNode {
                    range: 0..length,
                })],
            })],
        };
        match item_size {
            ItemSize::PerItem(func) => {
                for i in 0..length {
                    tree.insert(i, func(i));
                }
            }
            _ => {}
        }
        tree
    }

    fn insert(&mut self, index: usize, size: f32) -> bool {
        let root = self.nodes[0].as_inner_mut().unwrap();
        let (dirty, new_inner) = root.insert(index, size, self.chunk_size);
        if let Some(new_inner) = new_inner {
            let old_root = std::mem::replace(root, InnerNode {
                range: root.range.start..new_inner.range.end,
                total_real_size: root.total_real_size + new_inner.total_real_size,
                range_node_count: root.range_node_count + new_inner.range_node_count,
                children: vec![],
            });
            root.children.push(Node::Inner(old_root));
            root.children.push(Node::Inner(new_inner));
        }
        dirty

    }

    fn get_covering_range(&self, offset: f32, viewport_size: f32, item_size: &ItemSize) -> (Range<usize>, f32) {
        let root = self.nodes[0].as_inner().unwrap();
        match item_size {
            ItemSize::Dynamic => {
                if root.range_node_count == root.range.len() {
                    return (0..1.min(root.range.len()), 0.0);
                }
            }
            _ => {}
        }
        let range_size = match item_size {
            ItemSize::Fixed(size) => { *size }
            ItemSize::PerItem(_) => { 0.0 }
            ItemSize::Dynamic => {
                root.total_real_size / (root.range.len() - root.range_node_count) as f32
            }
        };
        let (start, render_offset) = if root.get_total_size(range_size) >= offset {
            root.largest_less_than(offset, range_size)
        } else {
            (root.range.end, 0.0)
        };
        let end = if root.get_total_size(range_size) >= offset + viewport_size {
            (root.largest_less_than(offset + viewport_size, range_size).0 + 1).min(root.range.len())
        } else {
            root.range.end
        };
        (start..end, render_offset)
    }

    fn get_total_size(&self, item_size: &ItemSize, viewport_size: f32) -> f32 {
        let root = self.nodes[0].as_inner().unwrap();
        match item_size {
            ItemSize::Dynamic => {
                if root.range_node_count == root.range.len() {
                    return viewport_size;
                }
            }
            _ => {}
        }
        let range_size = match item_size {
            ItemSize::Fixed(size) => { *size }
            ItemSize::PerItem(_) => { 0.0 }
            ItemSize::Dynamic => {
                root.total_real_size / (root.range.len() - root.range_node_count) as f32
            }
        };
        root.get_total_size(range_size)
    }
}

/// [`VirtualScrollView`] component properties.
#[derive(Props)]
pub struct VirtualScrollViewProps<'a, T: 'a> {
    /// Quantity of items in the VirtualScrollView.
    length: usize,
    /// Size of the items, height for vertical direction and width for horizontal.
    item_size: ItemSize,
    /// The item builder function.
    builder: Box<BuilderFunction<'a, T>>,
    /// Custom values to pass to the builder function.
    #[props(optional)]
    pub builder_values: Option<T>,
    /// Direction of the VirtualScrollView, `vertical` or `horizontal`.
    #[props(default = "vertical".to_string(), into)]
    pub direction: String,
    /// Height of the VirtualScrollView.
    #[props(default = "100%".to_string(), into)]
    pub height: String,
    /// Width of the VirtualScrollView.
    #[props(default = "100%".to_string(), into)]
    pub width: String,
    /// Padding of the VirtualScrollView.
    #[props(default = "0".to_string(), into)]
    pub padding: String,
    /// Show the scrollbar, visible by default.
    #[props(default = true, into)]
    pub show_scrollbar: bool,
    /// Enable scrolling with arrow keys.
    #[props(default = true, into)]
    pub scroll_with_arrows: bool,
}


#[derive(Props)]
struct WrapperProps<'a> {
    node: VNode<'a>,
    index: usize,
    size_cache: &'a UseRef<SizeCache>,
    direction: &'a String,
}

#[allow(non_snake_case)]
fn Wrapper<'a>(cx: Scope<'a, WrapperProps<'a>>) -> Element {
    let index = cx.props.index;
    let (node, size) = use_node(cx);
    if size != NodeReferenceLayout::default() {
        if cx.props.size_cache.write_silent().insert(cx.props.index, if cx.props.direction == "horizontal" { size.inner.width } else { size.inner.height }) {
            cx.props.size_cache.needs_update();
        }
    }

    render!(
        rect {
            key: "{index+1}",
            reference: node,
            &cx.props.node
        }
    )
}

/// `VirtualScrollView` component.
///
/// # Props
/// See [`VirtualScrollViewProps`](VirtualScrollViewProps).
///
/// # Example
///
/// ```no_run
/// # use freya::prelude::*;
/// fn app(cx: Scope) -> Element {
///     render!(
///         VirtualScrollView {
///             width: "100%",
///             height: "100%",
///             show_scrollbar: true,
///             length: 5,
///             item_size: 80.0,
///             builder_values: (),
///             direction: "vertical",
///             builder: Box::new(move |(k, i, _, _)| {
///                 rsx! {
///                     label {
///                         key: "{k}",
///                         height: "80",
///                         "Number {i}"
///                     }
///                 }
///             })
///         }
///     )
/// }
/// ```
#[allow(non_snake_case)]
pub fn VirtualScrollView<'a, T>(cx: Scope<'a, VirtualScrollViewProps<'a, T>>) -> Element {
    let clicking_scrollbar = use_ref::<Option<(Axis, f64)>>(cx, || None);
    let clicking_shift = use_ref(cx, || false);
    let clicking_alt = use_ref(cx, || false);
    let scrolled_y = use_ref(cx, || 0);
    let scrolled_x = use_ref(cx, || 0);
    let sizeCache = use_ref(cx, || SizeCache::new(cx.props.length, &cx.props.item_size, None));
    let (node_ref, size) = use_node(cx);
    let focus = use_focus(cx);

    let padding = &cx.props.padding;
    let user_container_width = &cx.props.width;
    let user_container_height = &cx.props.height;
    let user_direction = &cx.props.direction;
    let show_scrollbar = cx.props.show_scrollbar;
    let scroll_with_arrows = cx.props.scroll_with_arrows;

    let inner_size = sizeCache.read().get_total_size(&cx.props.item_size, if user_direction == "vertical" {
        size.area.height()
    } else {
        size.area.width()
    });

    let vertical_scrollbar_is_visible = user_direction != "horizontal"
        && is_scrollbar_visible(show_scrollbar, inner_size, size.area.height());
    let horizontal_scrollbar_is_visible = user_direction != "vertical"
        && is_scrollbar_visible(show_scrollbar, inner_size, size.area.width());

    let container_width = get_container_size(vertical_scrollbar_is_visible);
    let container_height = get_container_size(horizontal_scrollbar_is_visible);

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
        let speed_multiplier = if *clicking_alt.read() {
            SCROLL_SPEED_MULTIPLIER
        } else {
            1.0
        };

        if !*clicking_shift.read() {
            let wheel_y = e.get_delta_y() as f32 * speed_multiplier;

            let scroll_position_y = get_scroll_position_from_wheel(
                wheel_y,
                inner_size,
                size.area.height(),
                corrected_scrolled_y,
            );

            scrolled_y.with_mut(|y| *y = scroll_position_y);
        }

        let wheel_x = if *clicking_shift.read() {
            e.get_delta_y() as f32
        } else {
            e.get_delta_x() as f32
        } * speed_multiplier;

        let scroll_position_x = get_scroll_position_from_wheel(
            wheel_x,
            inner_size,
            size.area.width(),
            corrected_scrolled_x,
        );

        scrolled_x.with_mut(|x| *x = scroll_position_x);

        focus.focus();
    };

    // Drag the scrollbars
    let onmouseover = move |e: MouseEvent| {
        let clicking_scrollbar = clicking_scrollbar.read();

        if let Some((Axis::Y, y)) = *clicking_scrollbar {
            let coordinates = e.get_element_coordinates();
            let cursor_y = coordinates.y - y - size.area.min_y() as f64;

            let scroll_position =
                get_scroll_position_from_cursor(cursor_y as f32, inner_size, size.area.height());

            scrolled_y.with_mut(|y| *y = scroll_position);
        } else if let Some((Axis::X, x)) = *clicking_scrollbar {
            let coordinates = e.get_element_coordinates();
            let cursor_x = coordinates.x - x - size.area.min_x() as f64;

            let scroll_position =
                get_scroll_position_from_cursor(cursor_x as f32, inner_size, size.area.width());

            scrolled_x.with_mut(|x| *x = scroll_position);
        }

        if clicking_scrollbar.is_some() {
            focus.focus();
        }
    };

    let onkeydown = move |e: KeyboardEvent| {
        if !focus.is_focused() {
            return;
        }

        match &e.key {
            Key::Shift => {
                clicking_shift.set(true);
            }
            Key::Alt => {
                clicking_alt.set(true);
            }
            k => {
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

    let onkeyup = |e: KeyboardEvent| {
        if e.key == Key::Shift {
            clicking_shift.set(false);
        } else if e.key == Key::Alt {
            clicking_alt.set(false);
        }
    };

    // Mark the Y axis scrollbar as the one being dragged
    let onmousedown_y = |e: MouseEvent| {
        let coordinates = e.get_element_coordinates();
        *clicking_scrollbar.write_silent() = Some((Axis::Y, coordinates.y));
    };

    // Mark the X axis scrollbar as the one being dragged
    let onmousedown_x = |e: MouseEvent| {
        let coordinates = e.get_element_coordinates();
        *clicking_scrollbar.write_silent() = Some((Axis::X, coordinates.x));
    };

    // Unmark any scrollbar
    let onclick = |_: MouseEvent| {
        *clicking_scrollbar.write_silent() = None;
    };

    let horizontal_scrollbar_size = if horizontal_scrollbar_is_visible {
        SCROLLBAR_SIZE
    } else {
        0
    };

    let vertical_scrollbar_size = if vertical_scrollbar_is_visible {
        SCROLLBAR_SIZE
    } else {
        0
    };

    let (viewport_size, scroll_position) = if user_direction == "vertical" {
        (size.area.height(), corrected_scrolled_y)
    } else {
        (size.area.width(), corrected_scrolled_x)
    };

    let (render_range, render_offset) = sizeCache.read().get_covering_range(-scroll_position, viewport_size, &cx.props.item_size);

    let children = render_range.map(|i| {
        let child = (cx.props.builder)((i + 1, i, cx, &cx.props.builder_values)).call(cx.scope);
        rsx!(
            Wrapper {
                key: "{i+1}",
                node: child,
                index: i,
                size_cache: sizeCache,
                direction: user_direction,
            }
        )
    });

    render!(
        rect {
            role: "scrollView",
            overflow: "clip",
            direction: "horizontal",
            width: "{user_container_width}",
            height: "{user_container_height}",
            onglobalclick: onclick, // TODO(marc2332): mouseup would be better
            onglobalmouseover: onmouseover,
            onkeydown: onkeydown,
            onkeyup: onkeyup,
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
                    rect {
                        margin: "{-render_offset} 0 0 0",
                        children
                    }
                }
                ScrollBar {
                    width: "100%",
                    height: "{horizontal_scrollbar_size}",
                    offset_x: "{scrollbar_x}",
                    ScrollThumb {
                        onmousedown: onmousedown_x,
                        width: "{scrollbar_width}",
                        height: "100%",
                    },
                }
            }
            ScrollBar {
                width: "{vertical_scrollbar_size}",
                height: "100%",
                offset_y: "{scrollbar_y}",
                ScrollThumb {
                    onmousedown: onmousedown_y,
                    width: "100%",
                    height: "{scrollbar_height}",
                }
            }
        }
    )
}
