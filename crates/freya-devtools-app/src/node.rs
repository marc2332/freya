use freya::prelude::*;
use freya_core::integration::NodeId;

use crate::hooks::use_node_info;

#[derive(PartialEq)]
pub struct NodeElement {
    pub node_id: NodeId,
    pub window_id: u64,
    pub is_selected: bool,
    pub is_open: Option<bool>,
    pub on_selected: EventHandler<()>,
    pub on_arrow: EventHandler<()>,
}

impl Render for NodeElement {
    fn render_key(&self) -> DiffKey
    where
        Self: RenderKey,
    {
        DiffKey::from(&(self.node_id, self.window_id))
    }
    fn render(&self) -> impl IntoElement {
        let Some(node) = use_node_info(self.node_id, self.window_id) else {
            return rect().into_element();
        };

        let margin_left = ((node.height + 1) * 10) as f32 - 18.;
        let id = self.node_id.0;

        // TODO: Add a11y roles
        // let role = node.state.accessibility.builder.clone().and_then(|node| {
        //     let role = node.role();
        //     if role != Role::GenericContainer {
        //         Some(role)
        //     } else {
        //         None
        //     }
        // });

        let on_select = {
            let on_selected = self.on_selected.clone();
            move |_| on_selected.call(())
        };

        let on_open = {
            let handler = self.on_arrow.clone();
            let is_open = self.is_open;
            move |e: Event<PressEventData>| {
                if is_open.is_some() {
                    handler.call(());
                    e.stop_propagation();
                }
            }
        };

        let arrow_button = self.is_open.map(|is_open| {
            let arrow_degrees = if is_open { 0. } else { 270. };
            Button::new()
                .corner_radius(99.)
                .border_fill(Color::TRANSPARENT)
                .padding(Gaps::new_all(6.))
                .background(Color::TRANSPARENT)
                .on_press(on_open)
                .child(ArrowIcon::new().rotate(arrow_degrees).fill(Color::WHITE))
        });

        Button::new()
            .corner_radius(99.)
            .width(Size::fill())
            .height(Size::px(27.))
            .border_fill(Color::TRANSPARENT)
            .background(if self.is_selected {
                (40, 40, 40).into()
            } else {
                Color::TRANSPARENT
            })
            .hover_background(if self.is_selected {
                (40, 40, 40).into()
            } else {
                Color::from((45, 45, 45))
            })
            .on_press(on_select)
            .child(
                rect()
                    .offset_x(margin_left)
                    .direction(Direction::Horizontal)
                    .width(Size::fill())
                    .cross_align(Alignment::center())
                    .child(rect().width(Size::px(25.)).maybe_child(arrow_button))
                    .child(
                        paragraph()
                            .max_lines(1)
                            .font_size(14.)
                            // TODO: Add text overflow
                            .text_overflow(TextOverflow::Ellipsis)
                            .span(
                                Span::new(if node.is_window {
                                    "Window".to_string()
                                // TODO: Add a11 roles
                                // } else if let Some(role) = role {
                                //     format!("{role:?}")
                                } else {
                                    "element".to_string()
                                })
                                .color(Color::WHITE),
                            )
                            .span(
                                Span::new(if node.is_window {
                                    format!(", id: {}", self.window_id)
                                } else {
                                    format!(", id: {}", id)
                                })
                                .color(Color::from_rgb(200, 200, 200)),
                            ),
                    ),
            )
            .into()
    }
}
