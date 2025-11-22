#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    any::Any,
    borrow::Cow,
    rc::Rc,
};

use freya::{
    engine::prelude::{
        Paint,
        SkRect,
    },
    prelude::*,
};
use freya_core::{
    element::ElementExt,
    tree::DiffModifies,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

#[derive(PartialEq)]
struct CoolElement {
    layout_data: LayoutData,
    color: Color,
}

impl ElementExt for CoolElement {
    fn diff(&self, other: &std::rc::Rc<dyn ElementExt>) -> DiffModifies {
        let Some(element) = (other.as_ref() as &dyn Any).downcast_ref::<CoolElement>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.color != element.color {
            diff.insert(DiffModifies::STYLE);
        }

        if self.layout_data != element.layout_data {
            diff.insert(DiffModifies::LAYOUT);
        }

        diff
    }

    fn layout(&'_ self) -> std::borrow::Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout_data)
    }

    fn render(&self, context: RenderContext) {
        let area = context.layout_node.visible_area();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_color(self.color);
        let corner_radius = 12.;
        context.canvas.draw_round_rect(
            SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            corner_radius,
            corner_radius,
            &paint,
        );
    }
}

impl LayoutExt for CoolElement {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout_data
    }
}
impl ContainerExt for CoolElement {}

impl From<CoolElement> for Element {
    fn from(value: CoolElement) -> Self {
        Element::Element {
            key: DiffKey::None,
            element: Rc::new(value),
            elements: Vec::new(),
        }
    }
}

fn cool_element(color: impl Into<Color>) -> CoolElement {
    CoolElement {
        color: color.into(),
        layout_data: LayoutData::default(),
    }
}

fn app() -> impl IntoElement {
    let mut r = use_state(|| 127u8);

    rect()
        .expanded()
        .center()
        .child(
            cool_element((r(), 0, 0))
                .width(Size::px(100.))
                .height(Size::px(100.)),
        )
        .child(Button::new().child("Increase").on_press(move |_| {
            r.with_mut(|mut v| *v = v.saturating_add(25));
        }))
        .child(Button::new().child("Decrease").on_press(move |_| {
            r.with_mut(|mut v| *v = v.saturating_sub(25));
        }))
}
