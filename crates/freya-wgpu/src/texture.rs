use freya_core::{
    elements::image::ImageHandle,
    prelude::Bytes,
};
use freya_engine::prelude::{
    AlphaType,
    BackendTexture,
    ColorSpace,
    ColorType,
    MutableTextureState,
    SurfaceOrigin,
    gpu_images,
};

use crate::context::{
    SampledTexture,
    WgpuContext,
};

/// The pixel formats both wgpu and Skia can read, any other one has to be converted first.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GpuTextureFormat {
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Bgra8Unorm,
}

impl GpuTextureFormat {
    pub fn as_wgpu(self) -> wgpu::TextureFormat {
        match self {
            Self::Rgba8Unorm => wgpu::TextureFormat::Rgba8Unorm,
            Self::Rgba8UnormSrgb => wgpu::TextureFormat::Rgba8UnormSrgb,
            Self::Bgra8Unorm => wgpu::TextureFormat::Bgra8Unorm,
        }
    }

    /// Skia rejects the texture unless this matches the format the image was created with.
    pub fn as_color_type(self) -> ColorType {
        match self {
            Self::Rgba8Unorm => ColorType::RGBA8888,
            Self::Rgba8UnormSrgb => ColorType::SRGBA8888,
            Self::Bgra8Unorm => ColorType::BGRA8888,
        }
    }

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    pub fn as_vulkan(self) -> freya_engine::prelude::vk::Format {
        use freya_engine::prelude::vk::Format;

        match self {
            Self::Rgba8Unorm => Format::R8G8B8A8_UNORM,
            Self::Rgba8UnormSrgb => Format::R8G8B8A8_SRGB,
            Self::Bgra8Unorm => Format::B8G8R8A8_UNORM,
        }
    }

    /// The colour space of the sampled contents, which the hardware already decoded for sRGB.
    pub fn as_color_space(self) -> ColorSpace {
        match self {
            Self::Rgba8UnormSrgb => ColorSpace::new_srgb_linear(),
            Self::Rgba8Unorm | Self::Bgra8Unorm => ColorSpace::new_srgb(),
        }
    }

    /// Match a wgpu format, returning `None` for the ones Skia cannot sample directly.
    pub fn from_wgpu(format: wgpu::TextureFormat) -> Option<Self> {
        match format {
            wgpu::TextureFormat::Rgba8Unorm => Some(Self::Rgba8Unorm),
            wgpu::TextureFormat::Rgba8UnormSrgb => Some(Self::Rgba8UnormSrgb),
            wgpu::TextureFormat::Bgra8Unorm => Some(Self::Bgra8Unorm),
            _ => None,
        }
    }
}

/// The state wgpu leaves a texture in once its work is submitted, only used on Vulkan.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GpuTextureUsage {
    /// The renderer finishes by drawing into the texture.
    #[default]
    RenderAttachment,
    /// The renderer finishes by sampling or reading the texture.
    ShaderRead,
    /// The renderer finishes by copying into the texture.
    CopyDestination,
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
impl GpuTextureUsage {
    pub fn as_vulkan_layout(self) -> freya_engine::prelude::vk::ImageLayout {
        use freya_engine::prelude::vk::ImageLayout;

        match self {
            Self::RenderAttachment => ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            Self::ShaderRead => ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            Self::CopyDestination => ImageLayout::TRANSFER_DST_OPTIMAL,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GpuTextureDescriptor {
    pub width: u32,
    pub height: u32,
    pub format: GpuTextureFormat,
    /// What the renderer does with the texture last, before Freya draws it.
    pub usage: GpuTextureUsage,
}

impl GpuTextureDescriptor {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            format: GpuTextureFormat::Rgba8Unorm,
            usage: GpuTextureUsage::default(),
        }
    }

    pub fn with_format(mut self, format: GpuTextureFormat) -> Self {
        self.format = format;
        self
    }

    pub fn with_usage(mut self, usage: GpuTextureUsage) -> Self {
        self.usage = usage;
        self
    }
}

/// A wgpu texture Freya can draw without any copy.
pub struct GpuTexture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    descriptor: GpuTextureDescriptor,
    context: WgpuContext,
    generation: u64,
}

impl Drop for GpuTexture {
    /// Hand the image to the context so it outlives the frames still reading it.
    fn drop(&mut self) {
        self.context.retire(self.texture.clone());
    }
}

impl GpuTexture {
    pub fn new(context: &WgpuContext, descriptor: GpuTextureDescriptor) -> Self {
        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("freya-wgpu shared texture"),
            size: wgpu::Extent3d {
                width: descriptor.width.max(1),
                height: descriptor.height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: descriptor.format.as_wgpu(),
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self::clear(context, &view);

        Self {
            texture,
            view,
            descriptor,
            generation: context.generation(),
            context: context.clone(),
        }
    }

    /// Gives the image a defined layout and contents before anything draws into it.
    fn clear(context: &WgpuContext, view: &wgpu::TextureView) {
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("freya-wgpu clear"),
            });

        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("freya-wgpu clear"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        context.queue.submit([encoder.finish()]);
    }

    pub fn texture(&self) -> &wgpu::Texture {
        &self.texture
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn descriptor(&self) -> GpuTextureDescriptor {
        self.descriptor
    }

    /// Whether the texture still belongs to the device Freya renders on.
    pub fn is_valid(&self) -> bool {
        self.context.is_shared() && self.generation == self.context.generation()
    }

    /// Wrap the texture as an image for the frame being painted, redone every frame.
    pub fn image_handle(&self) -> Option<ImageHandle> {
        if !self.is_valid() {
            return None;
        }

        let (backend_texture, restore_state) = self.backend_texture()?;

        let image = self.context.interop.with_context(|context| {
            gpu_images::borrow_texture_from(
                context,
                &backend_texture,
                SurfaceOrigin::TopLeft,
                self.descriptor.format.as_color_type(),
                AlphaType::Premul,
                self.descriptor.format.as_color_space(),
            )
        })??;

        self.context
            .frame
            .borrow_mut()
            .sampled
            .push(SampledTexture {
                _texture: self.texture.clone(),
                _image: image.clone(),
                backend_texture,
                restore_state,
            });

        Some(ImageHandle::new(image, Bytes::new()))
    }

    #[cfg(any(target_os = "linux", target_os = "windows"))]
    fn backend_texture(&self) -> Option<(BackendTexture, Option<MutableTextureState>)> {
        use ash::vk::Handle;
        use freya_engine::prelude::{
            backend_textures,
            vk,
            vk::mutable_texture_states,
        };

        let hal_texture = unsafe { self.texture.as_hal::<wgpu::hal::api::Vulkan>() }?;
        let raw_image = unsafe { hal_texture.raw_handle() };

        let layout = self.descriptor.usage.as_vulkan_layout();
        let queue_family_index = self.context.interop.queue_family_index()?;

        let image_info = unsafe {
            vk::ImageInfo::new(
                raw_image.as_raw() as _,
                vk::Alloc::default(),
                vk::ImageTiling::OPTIMAL,
                layout,
                self.descriptor.format.as_vulkan(),
                1,
                queue_family_index,
                None,
                None,
                vk::SharingMode::EXCLUSIVE,
            )
        };

        let backend_texture = unsafe {
            backend_textures::make_vk(
                (self.descriptor.width as i32, self.descriptor.height as i32),
                &image_info,
                "freya-wgpu shared texture",
            )
        };

        Some((
            backend_texture,
            Some(mutable_texture_states::new_vulkan(
                layout,
                queue_family_index,
            )),
        ))
    }

    #[cfg(target_os = "macos")]
    fn backend_texture(&self) -> Option<(BackendTexture, Option<MutableTextureState>)> {
        use freya_engine::prelude::{
            backend_textures,
            gpu::Mipmapped,
            mtl,
        };
        use objc2::runtime::ProtocolObject;

        let hal_texture = unsafe { self.texture.as_hal::<wgpu::hal::api::Metal>() }?;
        let raw_texture: *const ProtocolObject<dyn objc2_metal::MTLTexture> =
            hal_texture.raw_handle();

        let texture_info = unsafe { mtl::TextureInfo::new(raw_texture as mtl::Handle) };
        let backend_texture = unsafe {
            backend_textures::make_mtl(
                (self.descriptor.width as i32, self.descriptor.height as i32),
                Mipmapped::No,
                &texture_info,
                "freya-wgpu shared texture",
            )
        };

        Some((backend_texture, None))
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    fn backend_texture(&self) -> Option<(BackendTexture, Option<MutableTextureState>)> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn formats_round_trip_through_wgpu() {
        let formats = [
            GpuTextureFormat::Rgba8Unorm,
            GpuTextureFormat::Rgba8UnormSrgb,
            GpuTextureFormat::Bgra8Unorm,
        ];
        for format in formats {
            assert_eq!(GpuTextureFormat::from_wgpu(format.as_wgpu()), Some(format));
        }
    }

    #[test]
    fn unsupported_formats_are_rejected() {
        assert_eq!(
            GpuTextureFormat::from_wgpu(wgpu::TextureFormat::R8Unorm),
            None
        );
        assert_eq!(
            GpuTextureFormat::from_wgpu(wgpu::TextureFormat::Rgba16Float),
            None
        );
    }

    #[test]
    fn color_types_match_the_channel_order() {
        assert_eq!(
            GpuTextureFormat::Rgba8Unorm.as_color_type(),
            ColorType::RGBA8888
        );
        assert_eq!(
            GpuTextureFormat::Rgba8UnormSrgb.as_color_type(),
            ColorType::SRGBA8888
        );
    }

    #[test]
    fn descriptor_builders_keep_the_size() {
        let descriptor = GpuTextureDescriptor::new(320, 240)
            .with_format(GpuTextureFormat::Bgra8Unorm)
            .with_usage(GpuTextureUsage::ShaderRead);

        assert_eq!(descriptor.width, 320);
        assert_eq!(descriptor.height, 240);
        assert_eq!(descriptor.format, GpuTextureFormat::Bgra8Unorm);
        assert_eq!(descriptor.usage, GpuTextureUsage::ShaderRead);
    }
}
