use raw_window_handle::{
    HasDisplayHandle,
    RawDisplayHandle,
};
use wayland_client::{
    Connection,
    Dispatch,
    QueueHandle,
    backend::Backend,
    delegate_noop,
    protocol::wl_registry,
};
use wayland_protocols::wp::linux_dmabuf::zv1::client::{
    zwp_linux_dmabuf_feedback_v1::{
        self,
        ZwpLinuxDmabufFeedbackV1,
    },
    zwp_linux_dmabuf_v1::ZwpLinuxDmabufV1,
};
use winit::window::Window;

use crate::drivers::vulkan::DrmDevice;

#[derive(Default)]
struct WaylandFeedback {
    dmabuf: Option<ZwpLinuxDmabufV1>,
    feedback: Option<ZwpLinuxDmabufFeedbackV1>,
    main_device: Option<u64>,
}

/// Read the compositor's preferred DRM device.
pub(crate) fn preferred_device(window: &Window) -> Option<DrmDevice> {
    let RawDisplayHandle::Wayland(display) = window.display_handle().ok()?.as_raw() else {
        return None;
    };
    // The window keeps the borrowed display alive until this connection is dropped.
    let backend = unsafe { Backend::from_foreign_display(display.display.as_ptr().cast()) };
    let connection = Connection::from_backend(backend);
    let mut queue = connection.new_event_queue();
    let _registry = connection.display().get_registry(&queue.handle(), ());
    let mut state = WaylandFeedback::default();
    let result = queue
        .roundtrip(&mut state)
        .and_then(|_| queue.roundtrip(&mut state));

    if let Some(feedback) = state.feedback.take() {
        feedback.destroy();
    }
    if let Some(dmabuf) = state.dmabuf.take() {
        dmabuf.destroy();
    }
    let _ = connection.flush();

    if let Err(error) = result {
        tracing::warn!(%error, "Failed to read Wayland DMA-BUF feedback");
        return None;
    }

    let Some(device) = state.main_device else {
        tracing::info!("Wayland DMA-BUF feedback did not provide a preferred GPU");
        return None;
    };

    let major = libc::major(device as libc::dev_t);
    let minor = libc::minor(device as libc::dev_t);
    tracing::info!(major, minor, "Wayland compositor's preferred DRM device");

    Some(DrmDevice(major, minor))
}

impl Dispatch<wl_registry::WlRegistry, ()> for WaylandFeedback {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        queue: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
            && interface == "zwp_linux_dmabuf_v1"
            && version >= 4
            && state.dmabuf.is_none()
        {
            let dmabuf: ZwpLinuxDmabufV1 = registry.bind(name, 4, queue, ());
            state.feedback = Some(dmabuf.get_default_feedback(queue, ()));
            state.dmabuf = Some(dmabuf);
        }
    }
}

impl Dispatch<ZwpLinuxDmabufFeedbackV1, ()> for WaylandFeedback {
    fn event(
        state: &mut Self,
        _: &ZwpLinuxDmabufFeedbackV1,
        event: zwp_linux_dmabuf_feedback_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let zwp_linux_dmabuf_feedback_v1::Event::MainDevice { device } = event {
            state.main_device = device.try_into().ok().map(u64::from_ne_bytes);
        }
    }
}

delegate_noop!(WaylandFeedback: ignore ZwpLinuxDmabufV1);
