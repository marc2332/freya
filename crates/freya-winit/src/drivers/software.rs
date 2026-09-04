use std::num::NonZeroU32;
#[cfg(target_os = "windows")]
use std::time::{
    Duration,
    Instant,
};

use freya_engine::prelude::{
    AlphaType,
    ColorType,
    ImageInfo,
    Surface as SkiaSurface,
    wrap_pixels,
};
use raw_window_handle::{
    DisplayHandle,
    HandleError,
    HasDisplayHandle,
    HasWindowHandle,
    RawDisplayHandle,
    RawWindowHandle,
    WindowHandle,
};
#[cfg(target_os = "windows")]
use windows_sys::Win32::Graphics::Dwm::DwmFlush;
use winit::{
    dpi::PhysicalSize,
    event_loop::ActiveEventLoop,
    window::{
        Window,
        WindowAttributes,
    },
};

struct DisplayHandleWrapper(RawDisplayHandle);

impl HasDisplayHandle for DisplayHandleWrapper {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        // SAFETY: the window owning this handle outlives the driver.
        Ok(unsafe { DisplayHandle::borrow_raw(self.0) })
    }
}

struct WindowHandleWrapper(RawWindowHandle);

impl HasWindowHandle for WindowHandleWrapper {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        // SAFETY: the window owning this handle outlives the driver.
        Ok(unsafe { WindowHandle::borrow_raw(self.0) })
    }
}

/// Graphics driver that renders in software via Skia and presents through softbuffer.
pub struct SoftwareDriver {
    _context: softbuffer::Context<DisplayHandleWrapper>,
    surface: softbuffer::Surface<DisplayHandleWrapper, WindowHandleWrapper>,
    #[cfg(target_os = "windows")]
    last_present: Option<Instant>,
}

impl SoftwareDriver {
    pub fn new(
        event_loop: &ActiveEventLoop,
        window_attributes: WindowAttributes,
    ) -> Result<(Self, Window), Box<dyn std::error::Error>> {
        let window = event_loop.create_window(window_attributes)?;
        let driver = Self::from_window(&window)?;
        Ok((driver, window))
    }

    /// Build the driver on an existing window instead of creating a new one.
    pub fn from_window(window: &Window) -> Result<Self, Box<dyn std::error::Error>> {
        let display_handle = window.display_handle()?.as_raw();
        let window_handle = window.window_handle()?.as_raw();

        let context = softbuffer::Context::new(DisplayHandleWrapper(display_handle))
            .map_err(|err| format!("Could not create softbuffer context: {err}"))?;
        let mut surface = softbuffer::Surface::new(&context, WindowHandleWrapper(window_handle))
            .map_err(|err| format!("Could not create softbuffer surface: {err}"))?;

        let size = window.inner_size();
        if let (Some(width), Some(height)) =
            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
        {
            surface
                .resize(width, height)
                .map_err(|err| format!("Could not size softbuffer surface: {err}"))?;
        }

        Ok(Self {
            _context: context,
            surface,
            #[cfg(target_os = "windows")]
            last_present: None,
        })
    }

    /// Block until the display takes the frame, since softbuffer's GDI blit never waits for it.
    #[cfg(target_os = "windows")]
    fn wait_for_display(&mut self, window: &Window) {
        // SAFETY: `DwmFlush` takes no arguments and only blocks until the next composition.
        if unsafe { DwmFlush() } >= 0 {
            return;
        }

        let refresh_rate = window
            .current_monitor()
            .and_then(|monitor| monitor.refresh_rate_millihertz())
            .filter(|refresh_rate| *refresh_rate > 1_000)
            .unwrap_or(60_000);
        let frame_interval = Duration::from_secs_f64(1000.0 / refresh_rate as f64);

        if let Some(last_present) = self.last_present
            && let Some(remaining) = frame_interval.checked_sub(last_present.elapsed())
        {
            std::thread::sleep(remaining);
        }

        self.last_present = Some(Instant::now());
    }

    pub fn present(
        &mut self,
        size: PhysicalSize<u32>,
        window: &Window,
        render: impl FnOnce(&mut SkiaSurface),
    ) {
        let (Some(width), Some(height)) =
            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
        else {
            return;
        };

        let mut buffer = match self.surface.buffer_mut() {
            Ok(buffer) => buffer,
            Err(err) => {
                tracing::error!("Failed to acquire software buffer: {err:?}");
                return;
            }
        };

        let info = ImageInfo::new(
            (width.get() as i32, height.get() as i32),
            ColorType::BGRA8888,
            AlphaType::Premul,
            None,
        );
        let row_bytes = width.get() as usize * 4;
        let pixels: &mut [u32] = &mut buffer;
        // SAFETY: u32 is 4-aligned and we own the buffer for the full borrow.
        let bytes: &mut [u8] = unsafe {
            std::slice::from_raw_parts_mut(pixels.as_mut_ptr() as *mut u8, pixels.len() * 4)
        };

        match wrap_pixels(&info, bytes, Some(row_bytes), None) {
            Some(mut wrapped_surface) => render(&mut wrapped_surface),
            None => {
                tracing::error!("Failed to wrap software pixels into a Skia surface");
                return;
            }
        }

        window.pre_present_notify();
        if let Err(err) = buffer.present() {
            tracing::error!("Failed to present software buffer: {err:?}");
        }

        #[cfg(target_os = "windows")]
        self.wait_for_display(window);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let (Some(width), Some(height)) =
            (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
        else {
            return;
        };
        if let Err(err) = self.surface.resize(width, height) {
            tracing::error!("Failed to resize software surface: {err:?}");
        }
    }
}
