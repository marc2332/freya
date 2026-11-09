use std::io;

use freya_core::integration::*;
use futures_channel::oneshot;
use rustc_hash::FxHashMap;
use torin::prelude::CursorPoint;
use winit::{
    data_transfer::{
        DataTransferId,
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

use crate::data_transfer::{
    DataFormatExt,
    TransferDataExt,
};

pub(crate) struct IncomingDrag {
    pub(crate) id: DataTransferId,
    pub(crate) data: DragDataHandle,
    pub(crate) decision: Option<DragDecision>,
    pub(crate) accepted: Option<NodeId>,
    pub(crate) dropped: bool,
}

pub(crate) struct PendingRead {
    pub(crate) transfer: DataTransferId,
    format: DataFormat,
    pub(crate) response: oneshot::Sender<Result<DataValue, DragError>>,
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
        data: TransferData,
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

        let result = data
            .into_winit()
            .map_err(|error| DragError::InvalidData(error.to_string()))
            .and_then(|data| {
                if actions.is_empty() {
                    return Err(DragError::InvalidData(
                        "A native drag needs an allowed action".into(),
                    ));
                }
                let actions: Vec<_> = actions
                    .into_iter()
                    .map(|action| match action {
                        DropAction::Copy => DndAction::Copy,
                        DropAction::Move => DndAction::Move,
                        DropAction::Link => DndAction::Link,
                    })
                    .collect();
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

    pub(crate) fn process_pending(&mut self, event_loop: &dyn ActiveEventLoop) {
        let Some(incoming) = &mut self.incoming else {
            return;
        };

        if let Some(decision) = incoming.decision.take()
            && !incoming.dropped
        {
            let accepted = decision.accepted();
            let actions: Vec<_> = accepted
                .map(|(_, action)| match action {
                    DropAction::Copy => DndAction::Copy,
                    DropAction::Move => DndAction::Move,
                    DropAction::Link => DndAction::Link,
                })
                .into_iter()
                .collect();
            if event_loop
                .set_valid_dnd_actions(incoming.id, &actions)
                .is_ok()
            {
                incoming.accepted = accepted.map(|(target, _)| target);
            }
        }

        let Ok(requests) = incoming.data.take_read_requests() else {
            self.expire();
            return;
        };
        for DragReadRequest { format, response } in requests {
            match event_loop.fetch_data_transfer(incoming.id, &format.to_type_hint()) {
                Ok(serial) => {
                    self.reads.insert(
                        serial,
                        PendingRead {
                            transfer: incoming.id,
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
    }

    pub(crate) fn receive(
        &mut self,
        transfer: DataTransferId,
        serial: AsyncRequestSerial,
        value: &dyn TypedData,
    ) {
        let Some(pending) = self.reads.remove(&serial) else {
            return;
        };
        let result = if pending.transfer == transfer {
            match pending.format.read(value) {
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    self.reads.insert(serial, pending);
                    return;
                }
                result => result.map_err(|error| DragError::InvalidData(error.to_string())),
            }
        } else {
            Err(DragError::Expired)
        };
        let _ = pending.response.send(result);
    }

    pub(crate) fn expire(&mut self) {
        if let Some(incoming) = self.incoming.take() {
            incoming.data.expire();
        }
        for (_, pending) in self.reads.drain() {
            let _ = pending.response.send(Err(DragError::Expired));
        }
    }

    pub(crate) fn prepare_drag_over(&mut self, cursor: CursorPoint) -> Option<PlatformEvent> {
        let incoming = self.incoming.as_mut()?;
        if incoming.dropped {
            return None;
        }
        incoming.accepted = None;
        let decision = DragDecision::default();
        incoming.decision = Some(decision.clone());
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
