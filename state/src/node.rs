use dioxus_native_core::node_ref::{AttributeMask, NodeMask, NodeView};
use dioxus_native_core::state::{ChildDepState, NodeDepState, State};
use dioxus_native_core_macro::{sorted_str_slice, State};
use skia_safe::Color;

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SizeMode {
    #[default]
    Auto,
    Percentage(i32),
    Manual(i32),
}

#[derive(Debug, Clone, State, Default)]
pub struct NodeState {
    #[child_dep_state(size)]
    pub size: Size,
    #[node_dep_state()]
    pub style: Style,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Size {
    // Support AUTO mode
    pub width: SizeMode,
    pub height: SizeMode,
    pub padding: (i32, i32, i32, i32),
}

impl ChildDepState for Size {
    // Size accepts a font size context
    type Ctx = ();
    // Size depends on the Size part of each child
    type DepState = Self;
    // Size only cares about the width, height, and text parts of the current node
    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!([
            "width", "height", "padding"
        ])))
        .with_text()
        .with_tag();
    fn reduce<'a>(
        &mut self,
        node: NodeView,
        mut children: impl Iterator<Item = &'a Self::DepState>,
        _ctx: &Self::Ctx,
    ) -> bool
    where
        Self::DepState: 'a,
    {
        let mut width = SizeMode::default();
        let mut height = SizeMode::default();
        let mut padding = (0, 0, 0, 0);

        // Text elements shouldn't be define by their children size but
        // by their text content if not specified otherwise
        if children.size_hint().0 > 0 && node.tag() != Some("p") {
            width = SizeMode::Manual(
                children
                    .by_ref()
                    .map(|item| {
                        if let SizeMode::Manual(width) = item.width {
                            width
                        } else {
                            0
                        }
                    })
                    .reduce(|accum, item| if accum >= item { accum } else { item })
                    .unwrap_or(0),
            );

            height = SizeMode::Manual(
                children
                    .map(|item| {
                        if let SizeMode::Manual(height) = item.height {
                            height
                        } else {
                            0
                        }
                    })
                    .reduce(|accum, item| if accum >= item { accum } else { item })
                    .unwrap_or(0),
            );
        }

        // if the node contains a width or height attribute it overrides the other size
        for a in node.attributes() {
            match a.name {
                "width" => {
                    let attr = a.value.to_string();
                    if &attr == "stretch" {
                        width = SizeMode::Percentage(100);
                    } else if &attr == "auto" {
                        width = SizeMode::Auto;
                    } else {
                        width = SizeMode::Manual(attr.parse().unwrap());
                    }
                }
                "height" => {
                    let attr = a.value.to_string();
                    if &attr == "stretch" {
                        height = SizeMode::Percentage(100);
                    } else if &attr == "auto" {
                        height = SizeMode::Auto;
                    } else {
                        height = SizeMode::Manual(attr.parse().unwrap());
                    }
                }
                "padding" => {
                    let total_padding: i32 = a.value.to_string().parse().unwrap();
                    let padding_for_side = total_padding / 2;
                    padding.0 = padding_for_side;
                    padding.1 = padding_for_side;
                    padding.2 = padding_for_side;
                    padding.3 = padding_for_side;
                }
                // because Size only depends on the width and height, no other attributes will be passed to the member
                _ => panic!(),
            }
        }
        // to determine what other parts of the dom need to be updated we return a boolean that marks if this member changed
        let changed = (width != self.width) || (height != self.height) || (padding != self.padding);
        *self = Self {
            width,
            height,
            padding,
        };
        changed
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Style {
    pub background: Color,
}

impl NodeDepState<()> for Style {
    type Ctx = ();

    const NODE_MASK: NodeMask =
        NodeMask::new_with_attrs(AttributeMask::Static(&sorted_str_slice!(["background"])))
            .with_text();
    fn reduce<'a>(&mut self, node: NodeView, _sibling: (), _ctx: &Self::Ctx) -> bool {
        let mut background = Color::TRANSPARENT;

        // if the node contains a width or height attribute it overrides the other size
        for attr in node.attributes() {
            match attr.name {
                "background" => {
                    let new_back = color_str(&attr.value.to_string());
                    if let Some(new_back) = new_back {
                        background = new_back;
                    }
                }
                _ => panic!(),
            }
        }
        // to determine what other parts of the dom need to be updated we return a boolean that marks if this member changed
        let changed = background != self.background;
        *self = Self { background };
        changed
    }
}

fn color_str(color: &str) -> Option<Color> {
    match color {
        "red" => Some(Color::RED),
        "green" => Some(Color::GREEN),
        "blue" => Some(Color::BLUE),
        "yellow" => Some(Color::YELLOW),
        "black" => Some(Color::BLACK),
        "gray" => Some(Color::GRAY),
        _ => None,
    }
}
