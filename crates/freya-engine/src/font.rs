use crate::skia::{
    FontArguments,
    Typeface,
    TypefaceFontProvider,
    VariationCoordinate,
    VariationPosition,
};

/// Registers a typeface and its available weight instances.
pub fn register_font_typeface(
    provider: &mut TypefaceFontProvider,
    font_name: &str,
    typeface: Typeface,
) {
    if let Some(weight_axis) = typeface.variation_design_parameters().and_then(|axes| {
        axes.into_iter()
            .find(|axis| axis.tag == VariationCoordinate::wght)
    }) {
        for weight in (100..=1000).step_by(100) {
            let weight = weight as f32;
            if weight < weight_axis.min
                || weight > weight_axis.max
                || weight == *typeface.font_style().weight() as f32
            {
                continue;
            }

            let coordinates = [VariationCoordinate {
                axis: VariationCoordinate::wght,
                value: weight,
            }];
            let arguments = FontArguments::new().set_variation_design_position(VariationPosition {
                coordinates: &coordinates,
            });
            if let Some(instance) = typeface.clone_with_arguments(&arguments) {
                provider.register_typeface(instance, Some(font_name));
            }
        }
    }

    provider.register_typeface(typeface, Some(font_name));
}
