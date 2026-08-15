use std::{
    str::FromStr,
    sync::{
        Arc,
        atomic::{
            AtomicBool,
            Ordering,
        },
    },
    time::Instant,
};

use atomic_refcell::AtomicRefCell;
use blitz_dom::{
    Document,
    DocumentConfig,
};
use blitz_html::HtmlDocument;
use blitz_paint::paint_scene;
use blitz_traits::{
    events::{
        BlitzKeyEvent,
        BlitzPointerEvent,
        BlitzPointerId,
        BlitzWheelDelta,
        BlitzWheelEvent,
        KeyState,
        MouseEventButton,
        MouseEventButtons,
        Point,
        PointerCoords,
        PointerDetails,
        UiEvent,
    },
    navigation::NavigationProvider,
    net::NetProvider,
    shell::{
        ColorScheme,
        ShellProvider,
        Viewport,
    },
};
use freya_core::prelude::{
    Code as FreyaCode,
    Key as FreyaKey,
    Modifiers as FreyaModifiers,
    MouseButton as FreyaMouseButton,
};
use freya_engine::prelude::Canvas;
use futures_channel::mpsc::UnboundedSender;
use keyboard_types::{
    Code as BlitzCode,
    Key as BlitzKey,
    Location as BlitzLocation,
    Modifiers as BlitzModifiers,
};
use reqwest::blocking::Client;
use skia_safe::{
    Matrix,
    Picture,
    PictureRecorder,
    Rect,
};
use smol_str::SmolStr;

use crate::{
    anyrender::{
        SkiaSceneCache,
        SkiaScenePainter,
    },
    net::{
        FreyaNavigationProvider,
        FreyaShellProvider,
        HttpNetProvider,
    },
};

/// Owns a parsed Blitz document and paints it directly into Freya's Skia canvas.
pub(crate) struct BlitzState {
    document: Option<HtmlDocument>,
    net_provider: Arc<dyn NetProvider>,
    shell_provider: Arc<dyn ShellProvider>,
    navigation_provider: Arc<dyn NavigationProvider>,
    cache: SkiaSceneCache,
    /// Draw commands replayed on unchanged frames.
    picture: Option<Picture>,
    paint_key: Option<(u32, u32, u32)>,
    /// Set when the document changes visually.
    redraw: Arc<AtomicBool>,
    buttons: MouseEventButtons,
    active_pointers: Arc<AtomicRefCell<Vec<BlitzPointerEvent>>>,
    created: Instant,
}

impl BlitzState {
    pub fn new(
        client: Client,
        wake: UnboundedSender<()>,
        navigate: UnboundedSender<String>,
    ) -> Self {
        let redraw = Arc::new(AtomicBool::new(true));
        Self {
            document: None,
            net_provider: Arc::new(HttpNetProvider { client }),
            shell_provider: Arc::new(FreyaShellProvider {
                redraw: redraw.clone(),
                wake,
            }),
            navigation_provider: Arc::new(FreyaNavigationProvider { navigate }),
            cache: SkiaSceneCache::new(),
            picture: None,
            paint_key: None,
            redraw,
            buttons: MouseEventButtons::empty(),
            active_pointers: Arc::new(AtomicRefCell::new(Vec::new())),
            created: Instant::now(),
        }
    }

    pub fn load(&mut self, html: &str, base_url: Option<String>) {
        let config = DocumentConfig {
            base_url,
            net_provider: Some(self.net_provider.clone()),
            shell_provider: Some(self.shell_provider.clone()),
            navigation_provider: Some(self.navigation_provider.clone()),
            ..Default::default()
        };
        self.document = Some(HtmlDocument::from_html(html, config));
        self.paint_key = None;
        self.shell_provider.request_redraw();
    }

    /// Paint the document into `canvas`, `x`/`y` and `width`/`height` are physical pixels.
    pub fn paint(&mut self, canvas: &Canvas, x: f32, y: f32, width: u32, height: u32, scale: f32) {
        let Some(document) = self.document.as_mut() else {
            return;
        };
        if width == 0 || height == 0 {
            return;
        }

        let key = (width, height, scale.to_bits());

        if self.redraw.swap(false, Ordering::Relaxed)
            || self.picture.is_none()
            || self.paint_key != Some(key)
        {
            if self.paint_key != Some(key) {
                document.set_viewport(Viewport::new(width, height, scale, ColorScheme::Light));
            }
            document.resolve(self.created.elapsed().as_secs_f64());

            if document.is_animating() {
                self.shell_provider.request_redraw();
            }

            let bounds = Rect::new(0.0, 0.0, width as f32, height as f32);
            let mut recorder = PictureRecorder::new();
            {
                let recording = recorder.begin_recording(bounds, false);
                let mut painter = SkiaScenePainter::new(recording, &mut self.cache);
                paint_scene(&mut painter, document, scale as f64, width, height, 0, 0);
            }
            self.cache.next_gen();

            self.picture = recorder.finish_recording_as_picture(None);
            self.paint_key = Some(key);
        }

        if let Some(picture) = &self.picture {
            canvas.draw_picture(picture, Some(&Matrix::translate((x, y))), None);
        }
    }

    pub fn mouse_move(&mut self, x: f32, y: f32) {
        let event = self.pointer_event(x, y, MouseEventButton::Main);
        self.dispatch(UiEvent::PointerMove(event));
    }

    pub fn mouse_button(
        &mut self,
        x: f32,
        y: f32,
        button: Option<FreyaMouseButton>,
        pressed: bool,
    ) {
        let button = map_button(button);
        if pressed {
            self.buttons |= MouseEventButtons::from(button);
        } else {
            self.buttons -= MouseEventButtons::from(button);
        }
        let event = self.pointer_event(x, y, button);
        self.dispatch(if pressed {
            UiEvent::PointerDown(event)
        } else {
            UiEvent::PointerUp(event)
        });
        self.shell_provider.request_redraw();
    }

    pub fn wheel(&mut self, x: f32, y: f32, delta_x: f64, delta_y: f64) {
        let coords = self.coords(x, y);
        self.dispatch(UiEvent::Wheel(BlitzWheelEvent {
            delta: BlitzWheelDelta::Pixels(delta_x, delta_y),
            element: Point {
                x: coords.client_x,
                y: coords.client_y,
            },
            coords,
            buttons: self.buttons,
            mods: BlitzModifiers::empty(),
        }));
    }

    pub fn key(
        &mut self,
        key: &FreyaKey,
        code: &FreyaCode,
        modifiers: FreyaModifiers,
        pressed: bool,
    ) {
        let event = BlitzKeyEvent {
            key: convert_key(key),
            code: convert_code(code),
            modifiers: convert_modifiers(modifiers),
            location: BlitzLocation::Standard,
            is_auto_repeating: false,
            is_composing: false,
            state: if pressed {
                KeyState::Pressed
            } else {
                KeyState::Released
            },
            text: key_text(key),
        };
        self.dispatch(if pressed {
            UiEvent::KeyDown(event)
        } else {
            UiEvent::KeyUp(event)
        });
        self.shell_provider.request_redraw();
    }

    fn coords(&self, x: f32, y: f32) -> PointerCoords {
        let (scroll_x, scroll_y) = self
            .document
            .as_ref()
            .map(|document| {
                let scroll = document.viewport_scroll();
                (scroll.x as f32, scroll.y as f32)
            })
            .unwrap_or((0.0, 0.0));
        PointerCoords {
            screen_x: x,
            screen_y: y,
            client_x: x,
            client_y: y,
            page_x: x + scroll_x,
            page_y: y + scroll_y,
        }
    }

    fn pointer_event(&self, x: f32, y: f32, button: MouseEventButton) -> BlitzPointerEvent {
        let coords = self.coords(x, y);
        BlitzPointerEvent {
            id: BlitzPointerId::Mouse,
            is_primary: true,
            element: Point {
                x: coords.client_x,
                y: coords.client_y,
            },
            coords,
            button,
            buttons: self.buttons,
            mods: BlitzModifiers::empty(),
            details: PointerDetails::default(),
            active_pointers: self.active_pointers.clone(),
        }
    }

    fn dispatch(&mut self, event: UiEvent) {
        if let Some(document) = &mut self.document {
            document.handle_ui_event(event);
        }
    }
}

fn map_button(button: Option<FreyaMouseButton>) -> MouseEventButton {
    match button {
        Some(FreyaMouseButton::Right) => MouseEventButton::Secondary,
        Some(FreyaMouseButton::Middle) => MouseEventButton::Auxiliary,
        Some(FreyaMouseButton::Back) => MouseEventButton::Fourth,
        Some(FreyaMouseButton::Forward) => MouseEventButton::Fifth,
        _ => MouseEventButton::Main,
    }
}

fn convert_modifiers(modifiers: FreyaModifiers) -> BlitzModifiers {
    let mut out = BlitzModifiers::empty();
    if modifiers.contains(FreyaModifiers::CONTROL) {
        out |= BlitzModifiers::CONTROL;
    }
    if modifiers.contains(FreyaModifiers::SHIFT) {
        out |= BlitzModifiers::SHIFT;
    }
    if modifiers.contains(FreyaModifiers::ALT) {
        out |= BlitzModifiers::ALT;
    }
    if modifiers.contains(FreyaModifiers::META) {
        out |= BlitzModifiers::META;
    }
    out
}

fn convert_key(key: &FreyaKey) -> BlitzKey {
    BlitzKey::from_str(&key.to_string()).unwrap_or(BlitzKey::Unidentified)
}

fn convert_code(code: &FreyaCode) -> BlitzCode {
    BlitzCode::from_str(&code.to_string()).unwrap_or(BlitzCode::Unidentified)
}

fn key_text(key: &FreyaKey) -> Option<SmolStr> {
    match key {
        FreyaKey::Character(text) => Some(SmolStr::new(text)),
        _ => None,
    }
}
