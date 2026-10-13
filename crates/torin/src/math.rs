pub(crate) fn sin_cos(angle_radians: f32) -> (f32, f32) {
    #[cfg(feature = "std")]
    {
        (angle_radians.sin(), angle_radians.cos())
    }
    #[cfg(not(feature = "std"))]
    {
        (libm::sinf(angle_radians), libm::cosf(angle_radians))
    }
}
