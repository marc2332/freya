use freya_layout::{RenderData, SafeDOM};
use skia_safe::{Canvas, Data, IRect, Image, Paint, Rect};

/// Render an `image` element
pub fn render_image(canvas: &mut Canvas, node: &RenderData, dom: &SafeDOM) {
    let dioxus_node = node.get_node(dom);
    let mut draw_img = |bytes: &[u8]| {
        let pic = Image::from_encoded(unsafe { Data::new_bytes(bytes) });
        if let Some(pic) = pic {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            canvas.draw_image_nine(
                pic,
                IRect::new(0, 0, 0, 0),
                Rect::new(
                    node.node_area.x,
                    node.node_area.y,
                    node.node_area.x + node.node_area.width,
                    node.node_area.y + node.node_area.height,
                ),
                skia_safe::FilterMode::Last,
                Some(&paint),
            );
        }
    };

    if let Some(image_ref) = &dioxus_node.state.references.image_ref {
        let image_data = image_ref.0.lock().unwrap();
        if let Some(image_data) = image_data.as_ref() {
            draw_img(image_data)
        }
    } else if let Some(image_data) = &dioxus_node.state.style.image_data {
        draw_img(image_data)
    }
}
