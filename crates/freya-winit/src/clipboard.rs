#[cfg(not(target_os = "android"))]
use std::time::Duration;

#[cfg(target_os = "android")]
use freya_clipboard::android::AndroidClipboard;
use freya_clipboard::prelude::{
    Clipboard,
    ClipboardError,
};
#[cfg(not(target_os = "android"))]
use freya_clipboard::prelude::{
    ClipboardFuture,
    ClipboardProvider,
};
#[cfg(not(target_os = "android"))]
use freya_core::data_transfer::TransferData;
use freya_core::data_transfer::{
    DataFormat,
    DataValue,
};
#[cfg(not(target_os = "android"))]
use freya_core::{
    integration::{
        GlobalUserEvent,
        SingleThreadErasedEvent,
    },
    prelude::timer,
};
use futures_channel::oneshot;
#[cfg(not(target_os = "android"))]
use futures_util::{
    future::{
        Either,
        select,
    },
    pin_mut,
};
use winit::data_transfer::DataTransferId;

use crate::renderer::NativeEventProxy;
#[cfg(not(target_os = "android"))]
use crate::renderer::{
    NativeEvent,
    NativeGenericEvent,
    NativePlatformErasedEventAction,
};

pub(crate) struct PendingClipboardRead {
    pub(crate) transfer: DataTransferId,
    pub(crate) format: DataFormat,
    pub(crate) response: oneshot::Sender<Result<DataValue, ClipboardError>>,
}

pub(crate) fn create_clipboard(proxy: NativeEventProxy) -> Clipboard {
    #[cfg(target_os = "android")]
    {
        let _ = proxy;
        Clipboard::create(Some(Box::new(AndroidClipboard)))
    }
    #[cfg(not(target_os = "android"))]
    {
        Clipboard::create(Some(Box::new(WinitClipboard { proxy })))
    }
}

#[cfg(not(target_os = "android"))]
struct WinitClipboard {
    proxy: NativeEventProxy,
}

#[cfg(not(target_os = "android"))]
impl ClipboardProvider for WinitClipboard {
    fn get(&self, format: DataFormat) -> ClipboardFuture<DataValue> {
        let (response, receiver) = oneshot::channel();
        let command = NativePlatformErasedEventAction::ReadClipboard { format, response };
        let _ = self
            .proxy
            .send_event(NativeEvent::Generic(NativeGenericEvent::User(
                GlobalUserEvent::Erased(SingleThreadErasedEvent(Box::new(command))),
            )));

        Box::pin(async move {
            let read = async { receiver.await.map_err(|_| ClipboardError::FailedToRead)? };
            let timeout = async {
                timer(Duration::from_secs(5)).await;
                Err(ClipboardError::FailedToRead)
            };
            pin_mut!(read, timeout);
            match select(read, timeout).await {
                Either::Left((result, _)) | Either::Right((result, _)) => result,
            }
        })
    }

    fn set(&self, data: TransferData) -> ClipboardFuture<()> {
        let (response, receiver) = oneshot::channel();
        let command = NativePlatformErasedEventAction::WriteClipboard { data, response };
        let _ = self
            .proxy
            .send_event(NativeEvent::Generic(NativeGenericEvent::User(
                GlobalUserEvent::Erased(SingleThreadErasedEvent(Box::new(command))),
            )));
        Box::pin(async move { receiver.await.map_err(|_| ClipboardError::FailedToSet)? })
    }
}
