//! Provides a clipboard abstraction to access the target system's clipboard.

use freya_core::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum ClipboardError {
    FailedToRead,
    FailedToSet,
    NotAvailable,
}

/// Raw pixels of an image on the clipboard.
///
/// Four channels per pixel (red, green, blue, alpha), one byte each, **unpremultiplied**, in
/// row-major order: the pixel at `(x, y)` starts at `(y * width + x) * 4`. A `2x1` image is
/// therefore eight bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardImage {
    pub width: usize,
    pub height: usize,
    pub rgba: Vec<u8>,
}

impl ClipboardImage {
    /// Whether the buffer holds exactly the pixels the size claims, and at least one of them.
    ///
    /// Checked before an image reaches a provider, because a platform handed a short buffer
    /// reads past the pixels it was promised.
    fn is_well_formed(&self) -> bool {
        let expected = self
            .width
            .checked_mul(self.height)
            .and_then(|pixels| pixels.checked_mul(4));
        !self.rgba.is_empty() && expected == Some(self.rgba.len())
    }
}

/// What the platform integration supplies so [Clipboard] has somewhere to read and write.
///
/// Provided into the root context as a `Box<dyn ClipboardProvider>` by whichever integration is
/// driving the app ([ClipboardContext] is the one every desktop backend uses), so a host that
/// speaks to something other than the desktop clipboard has a seam to speak through.
///
/// **Text and images are one provider**, because they are one clipboard: two providers would be
/// two connections claiming the same selection, and on the platforms that serve a paste out of
/// the process that copied, whichever was written last would own it.
pub trait ClipboardProvider {
    fn get_text(&mut self) -> Result<String, ClipboardError>;

    fn set_text(&mut self, contents: String) -> Result<(), ClipboardError>;

    fn get_image(&mut self) -> Result<ClipboardImage, ClipboardError>;

    /// Write `image` to the clipboard. Its buffer is already known to match its stated size,
    /// because [Clipboard::set_image] checks that before dispatching here.
    fn set_image(&mut self, image: ClipboardImage) -> Result<(), ClipboardError>;
}

/// The desktop clipboard, over [arboard].
///
/// Held by the integration for as long as the app runs rather than opened per call: X11 has no
/// clipboard daemon of its own, so the pasting application reads from the copying one and a
/// handle dropped at the end of a copy takes what was copied with it.
///
/// # Linux
///
/// The `wayland-data-control` feature is on, so a session with `WAYLAND_DISPLAY` set talks to the
/// compositor over `wlr-data-control` / `ext-data-control` (wlroots, KDE, GNOME 48 and up).
/// Where the compositor does not offer either protocol, arboard warns and falls back to its X11
/// backend, which under a Wayland session reaches the compositor's XWayland bridge.
///
/// That is a different route from the `wl_data_device` one a text-only clipboard can take on its
/// own display connection. It is the trade for images: no crate speaks the standard protocol
/// *and* carries image data, and a second provider for text alone would put two connections on
/// one selection.
pub struct ClipboardContext(arboard::Clipboard);

impl ClipboardContext {
    /// Open the desktop clipboard, or [ClipboardError::NotAvailable] where there is none.
    pub fn new() -> Result<Self, ClipboardError> {
        arboard::Clipboard::new()
            .map(Self)
            .map_err(|_| ClipboardError::NotAvailable)
    }
}

impl ClipboardProvider for ClipboardContext {
    fn get_text(&mut self) -> Result<String, ClipboardError> {
        self.0.get_text().map_err(|_| ClipboardError::FailedToRead)
    }

    fn set_text(&mut self, contents: String) -> Result<(), ClipboardError> {
        self.0
            .set_text(contents)
            .map_err(|_| ClipboardError::FailedToSet)
    }

    fn get_image(&mut self) -> Result<ClipboardImage, ClipboardError> {
        let image = self
            .0
            .get_image()
            .map_err(|_| ClipboardError::FailedToRead)?;
        Ok(ClipboardImage {
            width: image.width,
            height: image.height,
            rgba: image.bytes.into_owned(),
        })
    }

    fn set_image(&mut self, image: ClipboardImage) -> Result<(), ClipboardError> {
        self.0
            .set_image(arboard::ImageData {
                width: image.width,
                height: image.height,
                bytes: image.rgba.into(),
            })
            .map_err(|_| ClipboardError::FailedToSet)
    }
}

/// Access the clipboard.
///
/// # Examples
///
/// ```rust,no_run
/// use freya_clipboard::prelude::Clipboard;
///
/// // Read the clipboard content
/// if let Ok(content) = Clipboard::get() {
///     println!("{}", content);
/// }
///
/// // Write to the clipboard
/// Clipboard::set("Hello, Freya!".to_string());
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct Clipboard;

impl Clipboard {
    pub(crate) fn create_or_create() -> State<Option<Box<dyn ClipboardProvider>>> {
        consume_root_context()
    }

    // Read from the clipboard
    pub fn get() -> Result<String, ClipboardError> {
        Self::create_or_create()
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .get_text()
    }

    // Write to the clipboard
    pub fn set(contents: String) -> Result<(), ClipboardError> {
        Self::create_or_create()
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .set_text(contents)
    }

    /// Read an image from the clipboard.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use freya_clipboard::prelude::Clipboard;
    ///
    /// if let Ok(image) = Clipboard::get_image() {
    ///     println!("{}x{}", image.width, image.height);
    /// }
    /// ```
    pub fn get_image() -> Result<ClipboardImage, ClipboardError> {
        Self::create_or_create()
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .get_image()
    }

    /// Write an image to the clipboard.
    ///
    /// The pixel buffer has to match the stated size ([ClipboardImage]), or the write is refused
    /// here rather than handed to a provider to interpret.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use freya_clipboard::prelude::{
    ///     Clipboard,
    ///     ClipboardImage,
    /// };
    ///
    /// // A single red pixel
    /// let _ = Clipboard::set_image(ClipboardImage {
    ///     width: 1,
    ///     height: 1,
    ///     rgba: vec![255, 0, 0, 255],
    /// });
    /// ```
    pub fn set_image(image: ClipboardImage) -> Result<(), ClipboardError> {
        if !image.is_well_formed() {
            return Err(ClipboardError::FailedToSet);
        }
        Self::create_or_create()
            .write()
            .as_mut()
            .ok_or(ClipboardError::NotAvailable)?
            .set_image(image)
    }
}

#[cfg(test)]
mod tests {
    use crate::clipboard::{
        ClipboardContext,
        ClipboardImage,
        ClipboardProvider,
    };

    /// A buffer that does not match the stated size never reaches a provider: the platform would
    /// read past the pixels it was promised.
    ///
    /// Asserted against the guard rather than through [Clipboard](crate::clipboard::Clipboard),
    /// which reads a root context that a unit test has no runner to provide.
    #[test]
    fn an_image_whose_buffer_disagrees_with_its_size_is_refused() {
        let image = |width, height, rgba: Vec<u8>| ClipboardImage {
            width,
            height,
            rgba,
        };

        assert!(image(2, 1, vec![255, 0, 0, 255, 0, 255, 0, 255]).is_well_formed());
        assert!(!image(2, 1, vec![255, 0, 0, 255]).is_well_formed());
        assert!(!image(1, 1, vec![255, 0, 0, 255, 0]).is_well_formed());
        assert!(!image(0, 0, Vec::new()).is_well_formed());
        // The size alone would overflow the multiply, so there is no length that could match it.
        assert!(!image(usize::MAX, 2, vec![0; 8]).is_well_formed());
    }

    /// The desktop clipboard is a [ClipboardProvider] — that is what the platform integrations
    /// box up and provide. Compile-time only: opening a real clipboard is the integration's job,
    /// and a headless test host has none.
    #[test]
    fn the_desktop_clipboard_is_a_provider() {
        fn provider<T: ClipboardProvider + 'static>() {}
        provider::<ClipboardContext>();
    }
}
