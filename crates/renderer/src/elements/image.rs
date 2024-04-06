use freya_native_core::real_dom::NodeImmutable;

use freya_core::dom::DioxusNode;
use freya_engine::prelude::*;
use freya_node_state::{References, Style};
use torin::geometry::Area;

/// Render an `image` element
pub fn render_image(area: &Area, node_ref: &DioxusNode, canvas: &Canvas) {
    let node_style = node_ref.get::<Style>().unwrap();
    let node_references = node_ref.get::<References>().unwrap();

    let draw_img = |bytes: &[u8]| {
        let pic = Image::from_encoded(Data::new_copy(bytes));
        if let Some(pic) = pic {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            canvas.draw_image_nine(
                pic,
                IRect::new(0, 0, 0, 0),
                Rect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
                FilterMode::Last,
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
        draw_img(image_data.as_slice())
    }
}
