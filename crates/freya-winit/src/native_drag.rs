use freya_core::integration::*;
use futures_channel::oneshot;
use rustc_hash::FxHashMap;
use torin::prelude::CursorPoint;
use url::Url;
use winit::{
    data_transfer::{
        DataTransferId,
        DataTransferSend,
        DataTransferSendBuilder,
        SendData,
        TypeHint,
        TypedData,
    },
    event_loop::{
        ActiveEventLoop,
        AsyncRequestSerial,
        DndAction,
        DragIcon,
    },
    window::WindowId,
};

use crate::renderer::{
    NativeEvent,
    NativeEventProxy,
    NativeWindowEvent,
    NativeWindowEventAction,
};

pub(crate) struct IncomingDrag {
    pub(crate) id: DataTransferId,
    pub(crate) data: DragDataHandle,
    pub(crate) generation: u64,
    pub(crate) accepted: Option<NodeId>,
    pub(crate) dropped: bool,
}

pub(crate) struct PendingRead {
    pub(crate) transfer: DataTransferId,
    format: DragDataType,
    pub(crate) response: oneshot::Sender<Result<DragValue, DragError>>,
}

#[derive(Default)]
pub(crate) struct NativeDragState {
    pub(crate) incoming: Option<IncomingDrag>,
    pub(crate) reads: FxHashMap<AsyncRequestSerial, PendingRead>,
    pub(crate) outgoing: Option<(
        DataTransferId,
        oneshot::Sender<Result<NativeDragResult, DragError>>,
    )>,
}

impl NativeDragState {
    pub(crate) fn start(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        window: WindowId,
        data: DragData,
        actions: Vec<DropAction>,
        icon: Option<DragIcon>,
        response: oneshot::Sender<Result<NativeDragResult, DragError>>,
    ) {
        if self.outgoing.is_some() {
            let _ = response.send(Err(DragError::Platform(
                "A native drag is already active".into(),
            )));
            return;
        }

        let result = Self::build_send_data(data).and_then(|data| {
            if actions.is_empty() {
                return Err(DragError::InvalidData(
                    "A native drag needs an allowed action".into(),
                ));
            }
            let actions: Vec<_> = actions.into_iter().map(to_winit_action).collect();
            event_loop
                .start_drag(window, data, &actions, icon)
                .map_err(|error| DragError::Platform(error.to_string()))
        });
        match result {
            Ok(id) => {
                self.outgoing = Some((id, response));
            }
            Err(error) => {
                let _ = response.send(Err(error));
            }
        }
    }

    pub(crate) fn command(&mut self, event_loop: &dyn ActiveEventLoop, command: NativeDragCommand) {
        match command {
            NativeDragCommand::Read {
                transfer,
                format,
                response,
            } => {
                let transfer = DataTransferId::from_raw(transfer);
                if !self
                    .incoming
                    .as_ref()
                    .is_some_and(|incoming| incoming.id == transfer)
                {
                    let _ = response.send(Err(DragError::Expired));
                    return;
                }
                match event_loop.fetch_data_transfer(transfer, &to_type_hint(format)) {
                    Ok(serial) => {
                        self.reads.insert(
                            serial,
                            PendingRead {
                                transfer,
                                format,
                                response,
                            },
                        );
                    }
                    Err(error) => {
                        let _ = response.send(Err(DragError::Platform(error.to_string())));
                    }
                }
            }
            NativeDragCommand::Accept {
                transfer,
                generation,
                target,
                action,
            } => {
                if let Some(incoming) = &mut self.incoming
                    && incoming.id.into_raw() == transfer
                    && incoming.generation == generation
                    && !incoming.dropped
                    && event_loop
                        .set_valid_dnd_actions(incoming.id, &[to_winit_action(action)])
                        .is_ok()
                {
                    incoming.accepted = Some(target);
                }
            }
            command @ NativeDragCommand::Start { .. } => command.unsupported(),
        }
    }

    pub(crate) fn expire(&mut self) {
        if let Some(incoming) = self.incoming.take() {
            incoming.data.expire();
        }
        for (_, pending) in self.reads.drain() {
            let _ = pending.response.send(Err(DragError::Expired));
        }
    }

    pub(crate) fn prepare_drag_over(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        window: WindowId,
        proxy: &NativeEventProxy,
        cursor: CursorPoint,
    ) -> Option<PlatformEvent> {
        let incoming = self.incoming.as_mut()?;
        if incoming.dropped {
            return None;
        }
        incoming.generation = incoming.generation.wrapping_add(1);
        incoming.accepted = None;
        let _ = event_loop.set_valid_dnd_actions(incoming.id, &[]);
        let transfer = incoming.id.into_raw();
        let generation = incoming.generation;
        let proxy = proxy.clone();
        let decision = DragDecision::new(move |target, action| {
            let command = NativeDragCommand::Accept {
                transfer,
                generation,
                target,
                action,
            };
            let _ = proxy.send_event(NativeEvent::Window(NativeWindowEvent {
                window_id: window,
                action: NativeWindowEventAction::User(UserEvent::Erased(SingleThreadErasedEvent(
                    Box::new(command),
                ))),
            }));
        });
        Some(PlatformEvent::Drag {
            name: DragEventName::Over,
            cursor,
            data: incoming.data.clone(),
            decision: Some(decision),
            target: None,
        })
    }
}

impl Drop for NativeDragState {
    fn drop(&mut self) {
        self.expire();
        if let Some((_, response)) = self.outgoing.take() {
            let _ = response.send(Err(DragError::Expired));
        }
    }
}

pub(crate) fn to_type_hint(format: DragDataType) -> TypeHint {
    match format {
        DragDataType::Files => TypeHint::UriList,
        DragDataType::Text => TypeHint::Plaintext,
        DragDataType::Html => TypeHint::Html,
        DragDataType::Rtf => TypeHint::Rtf,
        DragDataType::Image => TypeHint::Image {
            extension_hint: Some("png"),
        },
        DragDataType::Audio => TypeHint::Audio {
            extension_hint: Some("wav"),
        },
    }
}

fn to_winit_action(action: DropAction) -> DndAction {
    match action {
        DropAction::Copy => DndAction::Copy,
        DropAction::Move => DndAction::Move,
        DropAction::Link => DndAction::Link,
    }
}

impl PendingRead {
    pub(crate) fn read(&self, value: &dyn TypedData) -> Result<DragValue, DragError> {
        match self.format {
            DragDataType::Files => {
                let uris = value
                    .try_as_uris()
                    .map_err(|error| DragError::InvalidData(error.to_string()))?;
                let paths = uris
                    .into_iter()
                    .map(|uri| {
                        Url::parse(&uri)
                            .map_err(|error| DragError::InvalidData(error.to_string()))?
                            .to_file_path()
                            .map_err(|_| {
                                DragError::InvalidData("A dropped URI is not a local file".into())
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(DragValue::Files(paths))
            }
            DragDataType::Text | DragDataType::Html | DragDataType::Rtf => value
                .try_as_string()
                .map(DragValue::Text)
                .map_err(|error| DragError::InvalidData(error.to_string())),
            _ => value
                .try_as_bytes()
                .map(DragValue::Bytes)
                .map_err(|error| DragError::InvalidData(error.to_string())),
        }
    }
}

impl NativeDragState {
    fn build_send_data(data: DragData) -> Result<Box<dyn DataTransferSend>, DragError> {
        if data.formats.is_empty() {
            return Err(DragError::InvalidData(
                "A native drag needs at least one data format".into(),
            ));
        }
        let mut builder = DataTransferSendBuilder::new(());
        for (format, value) in data.formats {
            match (format, value) {
                (DragDataType::Files, DragValue::Files(paths)) => {
                    let uris = paths
                        .iter()
                        .map(|path| {
                            Url::from_file_path(path)
                                .map(|url| url.to_string())
                                .map_err(|_| {
                                    DragError::InvalidData(
                                        "Dragged file paths must be absolute".into(),
                                    )
                                })
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    builder.add_type(TypeHint::UriList, move |_, _| {
                        Some(SendData::Uris(uris.clone()))
                    });
                }
                (DragDataType::Files, _) => {
                    return Err(DragError::InvalidData(
                        "Files require a file path payload".into(),
                    ));
                }
                (format, DragValue::Text(text)) => {
                    builder.add_type(to_type_hint(format), move |_, _| Some(text.clone()));
                }
                (format, DragValue::Bytes(bytes)) => {
                    builder.add_type(to_type_hint(format), move |_, _| Some(bytes.clone()));
                }
                (_, DragValue::Files(_)) => {
                    return Err(DragError::InvalidData(
                        "File paths require the Files format".into(),
                    ));
                }
            }
        }
        Ok(builder.build())
    }
}
