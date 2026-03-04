use freya_core::prelude::*;
use freya_router::prelude::RouterContext;

#[derive(PartialEq)]
pub struct NativeRouter {
    children: Vec<Element>,
}

impl ChildrenExt for NativeRouter {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Default for NativeRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeRouter {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
}

impl Component for NativeRouter {
    fn render(&self) -> impl IntoElement {
        rect()
            .on_global_pointer_press(|e: Event<PointerEventData>| match e.button() {
                Some(MouseButton::Back) => RouterContext::get().go_back(),
                Some(MouseButton::Forward) => RouterContext::get().go_forward(),
                _ => {}
            })
            .children(self.children.clone())
    }
}
