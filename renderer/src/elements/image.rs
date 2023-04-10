use dioxus_native_core::real_dom::NodeImmutable;
use freya_dom::DioxusNode;
use freya_layout::RenderData;
use freya_node_state::{References, Style};
use skia_safe::{Canvas, Data, IRect, Image, Paint, Rect};

/// Render an `image` element
pub fn render_image(render_node: &RenderData, node_ref: &DioxusNode, canvas: &mut Canvas) {
    let node_style = node_ref.get::<Style>().unwrap();
    let node_references = node_ref.get::<References>().unwrap();

    let mut draw_img = |bytes: &[u8]| {
        let pic = Image::from_encoded(unsafe { Data::new_bytes(bytes) });
        if let Some(pic) = pic {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            canvas.draw_image_nine(
                pic,
                IRect::new(0, 0, 0, 0),
                Rect::new(
                    render_node.node_area.min_x(),
                    render_node.node_area.min_y(),
                    render_node.node_area.max_x(),
                    render_node.node_area.max_y(),
                ),
                skia_safe::FilterMode::Last,
                Some(&paint),
            );
        }
    };

    if let Some(image_ref) = &node_references.image_ref {
        let image_data = image_ref.0.lock().unwrap();
        if let Some(image_data) = image_data.as_ref() {
            draw_img(image_data)
        }
    } else if let Some(image_data) = &node_style.image_data {
        draw_img(image_data)
    }
}
