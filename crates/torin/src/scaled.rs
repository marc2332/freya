use std::borrow::Cow;

pub trait Scaled {
    fn scale(&mut self, scale_factor: f32);

    fn scale_if_needed(&mut self, scale_factor: f32) {
        #[allow(clippy::float_cmp)]
        if scale_factor != 1.0 {
            self.scale(scale_factor);
        }
    }

    fn with_scale(&self, scale_factor: f32) -> Cow<Self>
    where
        Self: Clone,
    {
        #[allow(clippy::float_cmp)]
        if scale_factor == 1.0 {
            Cow::Borrowed(self)
        } else {
            let mut copy = self.clone();
            copy.scale(scale_factor);
            Cow::Owned(copy)
        }
    }
}
