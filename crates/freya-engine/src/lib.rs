mod skia;

pub mod prelude {
    mod source {
        pub use crate::skia::*;
    }

    pub use source::{
        BlurStyle as SkBlurStyle,
        Color as SkColor,
        Color4f as SkColor4f,
        Corner as SkCorner,
        Data as SkData,
        HSV as SkHSV,
        Image as SkImage,
        MaskFilter as SkMaskFilter,
        Matrix as SkMatrix,
        Paragraph as SkParagraph,
        Path as SkPath,
        PathFillType as SkPathFillType,
        Point as SkPoint,
        RGB as SkRGB,
        RRect as SkRRect,
        Rect as SkRect,
        Surface as SkSurface,
        TextAlign as SkTextAlign,
        TextDecoration as SkTextDecoration,
        TextHeightBehavior as SkTextHeightBehavior,
        TextShadow as SkTextShadow,
        *,
    };
}
