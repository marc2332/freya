#![allow(clippy::unnecessary_cast)]
//! Support for offsetting the native macOS traffic-light window buttons.
//!
//! Port of the `set_traffic_light_inset` implementation from
//! <https://github.com/nicoburns/winit/tree/traffic-light-insets> (the follow-up to
//! winit PR #4466), adapted to run *outside* winit: stock winit 0.30 owns the window
//! delegate, so instead of delegate callbacks the overlay view doubles as the
//! tracking-area owner, and re-application is driven by the freya-winit event loop
//! (see [`TrafficLights`]). The API mirrors the winit PR (a logical offset from the
//! default position) so this can migrate to `WindowAttributesMacOS` when it lands.
//!
//! The buttons are moved in place with `setFrameOrigin:`; they must remain
//! subviews of the private titlebar view, otherwise the `_NSThemeWidget`s draw
//! at their default location and the window title reflows. Keeping them there
//! however means they stop being interactive once moved outside the titlebar, so
//! we work around the relevant AppKit quirks the same way browsers (Chromium,
//! Firefox) do:
//!
//! * Hover highlighting inside the titlebar is decided by an undocumented
//!   `_mouseInGroup:` message sent to the `NSThemeFrame`, using a cached group
//!   rectangle that is not refreshed when the buttons move. We swizzle it to
//!   answer live from the current button frames. See
//!   <https://stackoverflow.com/a/30417372>.
//! * That painter is bound to the titlebar and stops firing once the buttons are
//!   outside it. There we instead paint the glyphs via each button's own cell
//!   press-state painter (`setHighlighted:`), which is anchored to the button's
//!   frame (as Firefox does, see Bugzilla #2037914).
//! * Hit-testing is clipped to the titlebar, so a transparent overlay view over
//!   the buttons forwards clicks to them and owns the hover tracking area.

use std::{
    cell::{
        Cell,
        RefCell,
    },
    ptr,
};

use objc2::{
    AllocAnyThread,
    MainThreadMarker,
    MainThreadOnly,
    define_class,
    msg_send,
    rc::Retained,
    runtime::{
        AnyClass,
        AnyObject,
        Bool,
        Sel,
    },
    sel,
};
use objc2_app_kit::{
    NSApplication,
    NSButton,
    NSEventType,
    NSTrackingArea,
    NSTrackingAreaOptions,
    NSView,
    NSWindow,
    NSWindowButton,
    NSWindowOrderingMode,
};
use objc2_core_foundation::{
    CGPoint,
    CGRect,
    CGSize,
};
use raw_window_handle::{
    HasWindowHandle,
    RawWindowHandle,
};
use winit::{
    dpi::LogicalSize,
    window::Window,
};

/// Cached native default geometry of the traffic-light buttons, in the
/// coordinate space of their superview (the private titlebar view). AppKit does
/// not expose this as a dedicated struct. `spacing` is the horizontal delta
/// between adjacent buttons.
#[derive(Clone, Copy, Debug)]
struct TrafficLightBase {
    x: f64,
    y: f64,
    spacing: f64,
}

/// Per-window traffic-light inset controller, held by
/// [`AppWindow`](crate::window::AppWindow) when
/// [`WindowConfig::with_traffic_light_inset`](crate::config::WindowConfig::with_traffic_light_inset)
/// was set. Winit's delegate re-lays out the buttons on resize, fullscreen
/// transitions and title/style changes, so the event loop calls [`reapply`](Self::reapply)
/// on `WindowEvent::Resized` / `WindowEvent::Focused` to put them back.
pub(crate) struct TrafficLights {
    ns_window: Retained<NSWindow>,
    inset: LogicalSize<f64>,
    /// Cached native default geometry of the traffic-light buttons.
    base: Cell<Option<TrafficLightBase>>,
    /// Overlay that forwards clicks to inset traffic-lights outside the titlebar.
    overlay: RefCell<Option<Retained<TrafficLightOverlay>>>,
    /// Tracking area (on the frame view) that drives traffic-light hover.
    tracking: RefCell<Option<Retained<NSTrackingArea>>>,
    mtm: MainThreadMarker,
}

impl TrafficLights {
    /// Resolve the `NSWindow` behind a winit [`Window`] and apply the inset.
    /// Returns `None` off the main thread or when the handle is not AppKit.
    pub(crate) fn attach(window: &Window, inset: (f64, f64)) -> Option<Self> {
        let mtm = MainThreadMarker::new()?;
        let RawWindowHandle::AppKit(handle) = window.window_handle().ok()?.as_raw() else {
            return None;
        };
        let ns_view = unsafe { handle.ns_view.cast::<NSView>().as_ref() };
        let this = Self {
            ns_window: ns_view.window()?,
            inset: LogicalSize::new(inset.0, inset.1),
            base: Cell::new(None),
            overlay: RefCell::new(None),
            tracking: RefCell::new(None),
            mtm,
        };
        this.reapply();
        Some(this)
    }

    /// Re-assert the inset after AppKit reset the button positions.
    pub(crate) fn reapply(&self) {
        apply_inset(
            &self.ns_window,
            self.inset,
            &self.base,
            &self.overlay,
            &self.tracking,
            self.mtm,
        );
    }
}

/// The three standard window buttons, but only if they all exist and are visible.
fn traffic_light_buttons(
    window: &NSWindow,
) -> Option<(Retained<NSButton>, Retained<NSButton>, Retained<NSButton>)> {
    let close = window.standardWindowButton(NSWindowButton::CloseButton)?;
    let miniaturize = window.standardWindowButton(NSWindowButton::MiniaturizeButton)?;
    let zoom = window.standardWindowButton(NSWindowButton::ZoomButton)?;
    if close.isHidden() || miniaturize.isHidden() || zoom.isHidden() {
        return None;
    }
    Some((close, miniaturize, zoom))
}

/// The bounding box of the three buttons, expressed in `frame_view`'s coordinates.
fn group_rect(window: &NSWindow, frame_view: &NSView) -> Option<CGRect> {
    let (close, miniaturize, zoom) = traffic_light_buttons(window)?;
    let rect_in_frame = |button: &NSButton| {
        frame_view.convertRect_fromView(button.frame(), unsafe { button.superview() }.as_deref())
    };
    let mut group = rect_in_frame(&close);
    for r in [rect_in_frame(&miniaturize), rect_in_frame(&zoom)] {
        let min_x = group.origin.x.min(r.origin.x);
        let min_y = group.origin.y.min(r.origin.y);
        let max_x = (group.origin.x + group.size.width).max(r.origin.x + r.size.width);
        let max_y = (group.origin.y + group.size.height).max(r.origin.y + r.size.height);
        group = CGRect::new(
            CGPoint::new(min_x, min_y),
            CGSize::new(max_x - min_x, max_y - min_y),
        );
    }
    Some(group)
}

/// Whether a button's frame is fully inside its superview's (the titlebar's) bounds.
fn button_within_titlebar(button: &NSButton) -> bool {
    let Some(superview) = (unsafe { button.superview() }) else {
        return false;
    };
    let bounds = superview.bounds();
    let frame = button.frame();
    frame.origin.x >= bounds.origin.x - 0.5
        && frame.origin.y >= bounds.origin.y - 0.5
        && frame.origin.x + frame.size.width <= bounds.origin.x + bounds.size.width + 0.5
        && frame.origin.y + frame.size.height <= bounds.origin.y + bounds.size.height + 0.5
}

fn same_view(a: &NSView, b: &NSView) -> bool {
    ptr::eq(
        a as *const NSView as *const (),
        b as *const NSView as *const (),
    )
}

// ===== Hover glyphs =====

/// Swizzled replacement for `-[NSThemeFrame _mouseInGroup:]` that reports whether
/// the pointer is within the (possibly moved) traffic-light group.
unsafe extern "C-unwind" fn mouse_in_group(
    this: *mut AnyObject,
    _cmd: Sel,
    _button: *mut AnyObject,
) -> Bool {
    // `this` is an `NSThemeFrame`, which is an `NSView`.
    let frame_view = unsafe { &*(this as *const NSView) };
    let inside = frame_view.window().and_then(|window| {
        let group = group_rect(&window, frame_view)?;
        let mouse =
            frame_view.convertPoint_fromView(window.mouseLocationOutsideOfEventStream(), None);
        Some(
            mouse.x >= group.origin.x
                && mouse.x <= group.origin.x + group.size.width
                && mouse.y >= group.origin.y
                && mouse.y <= group.origin.y + group.size.height,
        )
    });
    Bool::new(inside.unwrap_or(false))
}

/// Install the `_mouseInGroup:` swizzle once. Computing the answer from the live
/// button frames keeps the behavior identical for un-inset windows.
fn ensure_hover_swizzle() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let Some(cls) = AnyClass::get(c"NSThemeFrame") else {
            return;
        };
        let Some(method) = cls.instance_method(sel!(_mouseInGroup:)) else {
            return;
        };
        let imp: objc2::runtime::Imp = unsafe {
            std::mem::transmute(
                mouse_in_group
                    as unsafe extern "C-unwind" fn(*mut AnyObject, Sel, *mut AnyObject) -> Bool,
            )
        };
        unsafe { method.set_implementation(imp) };
    });
}

/// Updates the hover state of the traffic-light buttons.
///
/// Inside the titlebar we redraw the buttons and let the swizzled
/// `_mouseInGroup:` produce the native hover appearance. Outside the titlebar
/// that painter no longer fires, so we use the buttons' own highlight painter to
/// render the glyphs at their current location.
fn set_hover(window: &NSWindow, hovered: bool) {
    let Some((close, miniaturize, zoom)) = traffic_light_buttons(window) else {
        return;
    };
    let buttons = [&close, &miniaturize, &zoom];
    let in_titlebar = buttons.iter().all(|b| button_within_titlebar(b));
    for button in buttons {
        let button: &NSButton = button;
        if in_titlebar {
            // Native hover via the swizzled `_mouseInGroup:`; clear any leftover
            // press state from having previously been outside the titlebar.
            let _: () = unsafe { msg_send![button, setHighlighted: false] };
            let view: &NSView = button;
            view.setNeedsDisplay(true);
        } else {
            let _: () = unsafe { msg_send![button, setHighlighted: hovered] };
        }
    }
}

// ===== Overlay =====

define_class!(
    // SAFETY:
    // - NSView does not have any subclassing requirements beyond main-thread-only,
    //   which is inherited.
    // - `TrafficLightOverlay` does not implement `Drop`.
    #[unsafe(super = NSView)]
    #[name = "FreyaTrafficLightOverlay"]
    #[derive(Debug)]
    #[ivars = ()]
    struct TrafficLightOverlay;

    /// This documentation attribute makes rustfmt work for some reason?
    impl TrafficLightOverlay {
        // Forward clicks over a button to that button, even when it sits outside
        // the titlebar (where the titlebar's own hit-testing no longer reaches
        // it). Points that miss the buttons fall through (return `nil`).
        //
        // Only mouse-button events are forwarded. Returning the button (a view
        // outside this subtree) during AppKit's `cursorUpdate:` pass is fatal:
        // the button's default responder forwarding re-enters
        // `_nextResponderForEvent:`, which hit-tests the window again, reaches
        // this overlay again, and recurses until the stack overflows. Click
        // dispatch (`sendEvent:`) hit-tests exactly once, so it cannot loop.
        #[unsafe(method_id(hitTest:))]
        fn hit_test(&self, point: CGPoint) -> Option<Retained<NSView>> {
            if current_event_is_mouse_button(self.mtm()) {
                overlay_hit_test(self, point)
            } else {
                None
            }
        }

        // The overlay owns the hover tracking area (winit's delegate would in the
        // upstream patch, but we cannot add methods to it from out here).
        #[unsafe(method(mouseEntered:))]
        fn traffic_light_mouse_entered(&self, _event: Option<&AnyObject>) {
            if let Some(window) = self.window() {
                set_hover(&window, true);
            }
        }

        #[unsafe(method(mouseExited:))]
        fn traffic_light_mouse_exited(&self, _event: Option<&AnyObject>) {
            if let Some(window) = self.window() {
                set_hover(&window, false);
            }
        }
    }
);

impl TrafficLightOverlay {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = mtm.alloc().set_ivars(());
        unsafe { msg_send![super(this), init] }
    }
}

/// Whether the event AppKit is currently dispatching is a mouse button press or
/// release. Hit-tests for anything else (cursor updates, scrolls, tooltips) must
/// fall through the overlay.
fn current_event_is_mouse_button(mtm: MainThreadMarker) -> bool {
    let Some(event) = NSApplication::sharedApplication(mtm).currentEvent() else {
        return false;
    };
    matches!(
        event.r#type(),
        NSEventType::LeftMouseDown
            | NSEventType::LeftMouseUp
            | NSEventType::RightMouseDown
            | NSEventType::RightMouseUp
            | NSEventType::OtherMouseDown
            | NSEventType::OtherMouseUp
    )
}

/// Returns the traffic-light button under `point` (given in the overlay's
/// superview coordinates), or `None` so the event falls through.
fn overlay_hit_test(overlay: &NSView, point: CGPoint) -> Option<Retained<NSView>> {
    let window = overlay.window()?;
    let (close, miniaturize, zoom) = traffic_light_buttons(&window)?;
    let from = unsafe { overlay.superview() };
    for button in [close, miniaturize, zoom] {
        let local = button.convertPoint_fromView(point, from.as_deref());
        let bounds = button.bounds();
        if local.x >= bounds.origin.x
            && local.x <= bounds.origin.x + bounds.size.width
            && local.y >= bounds.origin.y
            && local.y <= bounds.origin.y + bounds.size.height
        {
            return Some(button.into_super().into_super());
        }
    }
    None
}

// ===== Entry point =====

/// Applies `inset` to the traffic-light buttons of `window`, and keeps the hover
/// swizzle, overlay and tracking area in sync. Does nothing if the buttons are
/// missing or hidden.
fn apply_inset(
    window: &NSWindow,
    inset: LogicalSize<f64>,
    base_cell: &Cell<Option<TrafficLightBase>>,
    overlay_cell: &RefCell<Option<Retained<TrafficLightOverlay>>>,
    tracking_cell: &RefCell<Option<Retained<NSTrackingArea>>>,
    mtm: MainThreadMarker,
) {
    ensure_hover_swizzle();

    let Some((close, miniaturize, zoom)) = traffic_light_buttons(window) else {
        return;
    };

    let close_rect = close.frame();
    let spacing = miniaturize.frame().origin.x - close_rect.origin.x;
    let current = TrafficLightBase {
        x: close_rect.origin.x,
        y: close_rect.origin.y,
        spacing,
    };

    // If the frames no longer match cached base + inset (AppKit reset them),
    // refresh the base from the current default geometry.
    let base = match base_cell.get() {
        Some(base) => {
            let expected_x = base.x + inset.width;
            let expected_y = base.y - inset.height;
            let drift = (close_rect.origin.x - expected_x).abs() > 0.5
                || (close_rect.origin.y - expected_y).abs() > 0.5
                || (spacing - base.spacing).abs() > 0.5;
            if drift {
                base_cell.set(Some(current));
                current
            } else {
                base
            }
        }
        None => {
            base_cell.set(Some(current));
            current
        }
    };

    let target_y = base.y - inset.height;
    for (index, button) in [&close, &miniaturize, &zoom].into_iter().enumerate() {
        let button: &NSView = button;
        button.setFrameOrigin(CGPoint::new(
            base.x + inset.width + (index as f64 * base.spacing),
            target_y,
        ));
    }

    // The overlay must exist before the tracking area: it is the area's owner.
    update_overlay(window, overlay_cell, mtm);
    if let Some(overlay) = overlay_cell.borrow().as_deref() {
        update_hover_tracking(window, overlay, tracking_cell);
    }
}

/// (Re)installs a tracking area over the current button group so the buttons are
/// redrawn/hover-lit when the pointer enters, wherever they are. Installed on the
/// frame view (which spans the whole window); adding a bare tracking area there
/// does not disturb the titlebar, unlike adding a subview.
fn update_hover_tracking(
    window: &NSWindow,
    owner: &AnyObject,
    tracking_cell: &RefCell<Option<Retained<NSTrackingArea>>>,
) {
    let Some(frame_view) = window
        .contentView()
        .and_then(|content| unsafe { content.superview() })
    else {
        return;
    };
    if let Some(old) = tracking_cell.borrow_mut().take() {
        frame_view.removeTrackingArea(&old);
    }
    let Some(rect) = group_rect(window, &frame_view) else {
        return;
    };
    let options =
        NSTrackingAreaOptions::MouseEnteredAndExited | NSTrackingAreaOptions::ActiveAlways;
    let area = unsafe {
        NSTrackingArea::initWithRect_options_owner_userInfo(
            NSTrackingArea::alloc(),
            rect,
            options,
            Some(owner),
            None,
        )
    };
    frame_view.addTrackingArea(&area);
    *tracking_cell.borrow_mut() = Some(area);
}

/// (Re)positions the transparent overlay over the current button group. The
/// overlay forwards clicks to the buttons and owns the hover tracking area.
fn update_overlay(
    window: &NSWindow,
    overlay_cell: &RefCell<Option<Retained<TrafficLightOverlay>>>,
    mtm: MainThreadMarker,
) {
    let Some(content_view) = window.contentView() else {
        return;
    };
    let Some(rect) = group_rect(window, &content_view) else {
        return;
    };

    let overlay = overlay_cell
        .borrow_mut()
        .get_or_insert_with(|| TrafficLightOverlay::new(mtm))
        .clone();

    // Keep the overlay in the content view, in front, so it doesn't disturb the
    // titlebar (adding subviews to the frame view resets the button positions).
    if unsafe { overlay.superview() }
        .as_deref()
        .is_none_or(|sv| !same_view(sv, &content_view))
    {
        content_view.addSubview_positioned_relativeTo(&overlay, NSWindowOrderingMode::Above, None);
    }
    overlay.setFrame(rect);
}
