#[cfg(feature = "mocked-engine")]
mod mocked;

#[cfg(feature = "skia-engine")]
mod skia;

pub mod prelude {
    mod source {
        #[cfg(all(feature = "mocked-engine", not(feature = "skia-engine")))]
        pub use crate::mocked::*;
        #[cfg(feature = "skia-engine")]
        pub use crate::skia::*;
    }

    pub use source::{
        ArcSize as SkArcSize,
        BlurStyle as SkBlurStyle,
        Color as SkColor,
        Color4f as SkColor4f,
        Corner as SkCorner,
        Data as SkData,
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
        TextHeightBehavior as SkTextHeightBehavior,
        TextShadow as SkTextShadow,
        *,
    };
}
