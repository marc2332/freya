#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub enum SizeMode {
    #[default]
    Auto,
    Percentage(i32),
    Manual(i32),
}
struct NodeConstraints<'a> {
    tag: &'a str,
    width: SizeMode,
    height: SizeMode,
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
struct Viewport {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn calculate_viewport(width: SizeMode, height: SizeMode, mut viewport: Viewport) -> Viewport {
    match width {
        SizeMode::Manual(w) => {
            viewport.width = w;
        }
        SizeMode::Percentage(per) => {
            viewport.width = viewport.width / 100 * per;
        }
        SizeMode::Auto => {}
    }

    match height {
        SizeMode::Manual(h) => {
            viewport.height = h;
        }
        SizeMode::Percentage(per) => {
            viewport.height = viewport.height / 100 * per;
        }
        SizeMode::Auto => {}
    }

    viewport
}

fn calculate_size(
    node: &NodeConstraints,
    children: Vec<NodeConstraints>,
    viewport: Viewport,
) -> Viewport {
    let mut node_viewport = calculate_viewport(node.width, node.height, viewport);
    let mut inner_viewport = node_viewport;

    for child in &children {
        let child_result = calculate_size(child, vec![], inner_viewport);

        inner_viewport.y = child_result.y + child_result.height;
        inner_viewport.height -= child_result.height;

        inner_viewport.x = child_result.x + child_result.width;
        inner_viewport.width -= child_result.width;
    }

    if let SizeMode::Auto = node.width {
        node_viewport.width = inner_viewport.x - node_viewport.x;
    }

    if let SizeMode::Auto = node.height {
        node_viewport.height = inner_viewport.y - node_viewport.y;
    }

    node_viewport
}

#[cfg(test)]
mod test {
    use crate::{calculate_size, NodeConstraints, SizeMode, Viewport};

    #[test]
    fn percentage() {
        let result = calculate_size(
            &NodeConstraints {
                tag: "div",
                width: SizeMode::Percentage(100),
                height: SizeMode::Percentage(100),
            },
            vec![],
            Viewport {
                x: 0,
                y: 0,
                height: 300,
                width: 200,
            },
        );

        assert_eq!(result.height, 300);
        assert_eq!(result.width, 200);
    }

    #[test]
    fn manual() {
        let result = calculate_size(
            &NodeConstraints {
                tag: "div",
                width: SizeMode::Manual(250),
                height: SizeMode::Manual(150),
            },
            vec![],
            Viewport {
                x: 0,
                y: 0,
                height: 300,
                width: 200,
            },
        );

        assert_eq!(result.height, 150);
        assert_eq!(result.width, 250);
    }

    #[test]
    fn auto() {
        let result = calculate_size(
            &NodeConstraints {
                tag: "div",
                width: SizeMode::Auto,
                height: SizeMode::Auto,
            },
            vec![NodeConstraints {
                tag: "div",
                width: SizeMode::Manual(170),
                height: SizeMode::Percentage(25),
            }],
            Viewport {
                x: 0,
                y: 0,
                height: 300,
                width: 200,
            },
        );

        assert_eq!(result.height, 75);
        assert_eq!(result.width, 170);
    }
}
