//! [`CameraViewer`] component.

use freya_core::{
    elements::image::*,
    prelude::*,
};

use crate::{
    camera::CameraError,
    use_camera::UseCamera,
};

/// Live camera preview component.
///
/// Renders the latest frame produced by a [`UseCamera`] handle. While the
/// camera has not yet produced a frame the `placeholder` is rendered instead.
///
/// # Example
///
/// ```rust, no_run
/// use freya::{
///     camera::*,
///     prelude::*,
/// };
///
/// fn app() -> impl IntoElement {
///     let camera = use_camera(CameraConfig::default);
///     CameraViewer::new(camera)
/// }
/// ```
#[derive(PartialEq)]
pub struct CameraViewer {
    camera: UseCamera,

    layout: LayoutData,
    image_data: ImageData,
    accessibility: AccessibilityData,
    effect: EffectData,
    corner_radius: Option<CornerRadius>,

    children: Vec<Element>,
    loading_placeholder: Option<Element>,
    error_renderer: Option<Callback<CameraError, Element>>,

    key: DiffKey,
}

impl CameraViewer {
    pub fn new(camera: UseCamera) -> Self {
        Self {
            camera,
            layout: LayoutData::default(),
            image_data: ImageData::default(),
            accessibility: AccessibilityData::default(),
            effect: EffectData::default(),
            corner_radius: None,
            children: Vec::new(),
            loading_placeholder: None,
            error_renderer: None,
            key: DiffKey::None,
        }
    }

    pub fn corner_radius(mut self, corner_radius: impl Into<CornerRadius>) -> Self {
        self.corner_radius = Some(corner_radius.into());
        self
    }

    /// Custom element rendered while the camera has not yet produced a frame.
    pub fn loading_placeholder(mut self, placeholder: impl Into<Element>) -> Self {
        self.loading_placeholder = Some(placeholder.into());
        self
    }

    /// Custom element rendered when the camera fails before producing any frame.
    pub fn error_renderer(mut self, renderer: impl Into<Callback<CameraError, Element>>) -> Self {
        self.error_renderer = Some(renderer.into());
        self
    }
}

impl KeyExt for CameraViewer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl LayoutExt for CameraViewer {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl ContainerSizeExt for CameraViewer {}
impl ContainerWithContentExt for CameraViewer {}

impl ImageExt for CameraViewer {
    fn get_image_data(&mut self) -> &mut ImageData {
        &mut self.image_data
    }
}

impl AccessibilityExt for CameraViewer {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
    }
}

impl ChildrenExt for CameraViewer {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl EffectExt for CameraViewer {
    fn get_effect(&mut self) -> &mut EffectData {
        &mut self.effect
    }
}

impl Component for CameraViewer {
    fn render(&self) -> impl IntoElement {
        if let Some(holder) = self.camera.frame.read().clone() {
            return image(holder)
                .accessibility(self.accessibility.clone())
                .a11y_role(AccessibilityRole::Image)
                .a11y_focusable(true)
                .layout(self.layout.clone())
                .image_data(self.image_data.clone())
                .effect(self.effect.clone())
                .children(self.children.clone())
                .map(self.corner_radius, |img, corner_radius| {
                    img.corner_radius(corner_radius)
                })
                .into_element();
        }

        if let Some(renderer) = &self.error_renderer
            && let Some(err) = self.camera.error.read().clone()
        {
            return renderer.call(err);
        }

        rect()
            .layout(self.layout.clone())
            .center()
            .map(self.loading_placeholder.clone(), |r, placeholder| {
                r.child(placeholder)
            })
            .into_element()
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
