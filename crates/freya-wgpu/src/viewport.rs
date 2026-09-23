use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    rc::Rc,
};

use freya_core::{
    element::ElementExt,
    elements::image::ImageHandle,
    prelude::*,
    tree::DiffModifies,
};
use freya_engine::prelude::{
    FilterMode,
    MipmapMode,
    Paint,
    SamplingOptions,
    SkRect,
};
use torin::prelude::Size;

use crate::{
    context::WgpuContext,
    texture::{
        GpuTexture,
        GpuTextureDescriptor,
        GpuTextureFormat,
        GpuTextureUsage,
    },
};

type Painter = Rc<dyn Fn(WgpuFrame) -> Option<ImageHandle>>;

type Renderer = Rc<dyn Fn(&GpuTexture, WgpuFrame)>;

/// What a viewport knows when Freya paints it, with the size in physical pixels.
#[derive(Clone)]
pub struct WgpuFrame {
    pub context: WgpuContext,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
}

fn filled_layout() -> LayoutData {
    LayoutData {
        layout: torin::node::Node {
            width: Size::fill(),
            height: Size::fill(),
            ..Default::default()
        },
    }
}

/// Draws what an external renderer produces, running the painter once per painted frame.
pub struct WgpuViewport {
    layout: LayoutData,
    accessibility: AccessibilityData,
    context: Option<WgpuContext>,
    painter: Painter,
    children: Vec<Element>,
}

/// Build a viewport from a painter, see [`WgpuViewer`] for the managed version.
pub fn wgpu_viewport(painter: impl Fn(WgpuFrame) -> Option<ImageHandle> + 'static) -> WgpuViewport {
    WgpuViewport {
        layout: filled_layout(),
        accessibility: AccessibilityData::default(),
        context: WgpuContext::try_get(),
        painter: Rc::new(painter),
        children: Vec::new(),
    }
}

impl ElementExt for WgpuViewport {
    fn layout(&'_ self) -> Cow<'_, LayoutData> {
        Cow::Borrowed(&self.layout)
    }

    fn accessibility(&'_ self) -> Cow<'_, AccessibilityData> {
        Cow::Borrowed(&self.accessibility)
    }

    fn changed(&self, other: &Rc<dyn ElementExt>) -> bool {
        !self.diff(other).is_empty()
    }

    fn diff(&self, other: &Rc<dyn ElementExt>) -> DiffModifies {
        let Some(other) = (other.as_ref() as &dyn Any).downcast_ref::<Self>() else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if !Rc::ptr_eq(&self.painter, &other.painter) {
            diff.insert(DiffModifies::STYLE);
        }

        if self.layout != other.layout {
            diff.insert(DiffModifies::LAYOUT);
        }

        diff
    }

    fn render(&self, context: RenderContext) {
        let Some(wgpu_context) = &self.context else {
            return;
        };

        let area = context.layout_node.visible_area();
        let width = area.width().round().max(1.0) as u32;
        let height = area.height().round().max(1.0) as u32;

        let Some(handle) = (self.painter)(WgpuFrame {
            context: wgpu_context.clone(),
            width,
            height,
            scale_factor: context.scale_factor as f32,
        }) else {
            return;
        };

        let mut paint = Paint::default();
        paint.set_anti_alias(true);

        context.canvas.draw_image_rect_with_sampling_options(
            &handle.image,
            None,
            SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            SamplingOptions::new(FilterMode::Linear, MipmapMode::None),
            &paint,
        );
    }
}

impl LayoutExt for WgpuViewport {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl AccessibilityExt for WgpuViewport {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
    }
}

impl ChildrenExt for WgpuViewport {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl ContainerExt for WgpuViewport {}

impl ContainerWithContentExt for WgpuViewport {}

impl MaybeExt for WgpuViewport {}

impl From<WgpuViewport> for Element {
    fn from(mut element: WgpuViewport) -> Self {
        let elements = std::mem::take(&mut element.children);
        Element::Element {
            key: DiffKey::None,
            element: Rc::new(element),
            elements,
        }
    }
}

/// A viewport backed by one texture that always matches the area it fills.
///
/// # Example
///
/// ```rust, no_run
/// use freya_core::prelude::*;
/// use freya_wgpu::prelude::*;
///
/// fn app() -> impl IntoElement {
///     WgpuViewer::new(|texture, frame| {
///         // Record and submit a pass into `texture.view()` with `frame.context`.
///         let _ = (texture.view(), frame.width, frame.height);
///     })
/// }
/// ```
#[derive(Clone)]
pub struct WgpuViewer {
    layout: LayoutData,
    accessibility: AccessibilityData,
    format: GpuTextureFormat,
    usage: GpuTextureUsage,
    renderer: Renderer,
    children: Vec<Element>,
    key: DiffKey,
}

impl WgpuViewer {
    pub fn new(renderer: impl Fn(&GpuTexture, WgpuFrame) + 'static) -> Self {
        Self {
            layout: filled_layout(),
            accessibility: AccessibilityData::default(),
            format: GpuTextureFormat::Rgba8Unorm,
            usage: GpuTextureUsage::RenderAttachment,
            renderer: Rc::new(renderer),
            children: Vec::new(),
            key: DiffKey::None,
        }
    }

    pub fn format(mut self, format: GpuTextureFormat) -> Self {
        self.format = format;
        self
    }

    /// What the renderer does with the texture last, before Freya draws it.
    pub fn usage(mut self, usage: GpuTextureUsage) -> Self {
        self.usage = usage;
        self
    }
}

impl PartialEq for WgpuViewer {
    fn eq(&self, other: &Self) -> bool {
        self.layout == other.layout
            && self.accessibility == other.accessibility
            && self.format == other.format
            && self.usage == other.usage
            && self.children == other.children
            && self.key == other.key
            && Rc::ptr_eq(&self.renderer, &other.renderer)
    }
}

impl LayoutExt for WgpuViewer {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout
    }
}

impl AccessibilityExt for WgpuViewer {
    fn get_accessibility_data(&mut self) -> &mut AccessibilityData {
        &mut self.accessibility
    }
}

impl ChildrenExt for WgpuViewer {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl ContainerExt for WgpuViewer {}

impl ContainerWithContentExt for WgpuViewer {}

impl KeyExt for WgpuViewer {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for WgpuViewer {
    fn render(&self) -> impl IntoElement {
        let texture = use_hook(|| Rc::new(RefCell::new(None::<GpuTexture>)));

        let renderer = self.renderer.clone();
        let format = self.format;
        let usage = self.usage;

        wgpu_viewport(move |frame| {
            let descriptor = GpuTextureDescriptor {
                width: frame.width,
                height: frame.height,
                format,
                usage,
            };

            let mut slot = texture.borrow_mut();
            let matches = slot
                .as_ref()
                .is_some_and(|texture| texture.descriptor() == descriptor && texture.is_valid());
            if !matches {
                *slot = Some(GpuTexture::new(&frame.context, descriptor));
            }

            let texture = slot.as_ref()?;
            renderer(texture, frame);
            texture.image_handle()
        })
        .layout(self.layout.clone())
        .accessibility(self.accessibility.clone())
        .children(self.children.clone())
    }
}
