// FromStr but we own it so we can impl it on torin and skia_safe types.
pub trait Parse: Sized {
    type Err;

    fn parse(balue: &str, scale_factor: Option<f32>) -> Result<Self, Self::Err>;
}