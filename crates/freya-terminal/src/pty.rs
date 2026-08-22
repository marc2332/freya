use std::{
    cell::RefCell,
    io::Write,
    rc::Rc,
    sync::mpsc::{
        Sender,
        channel,
    },
    time::Instant,
};

use async_io::Timer;
use freya_core::{
    notify::ArcNotify,
    prelude::{
        Platform,
        UserEvent,
        spawn_forever,
    },
};
use futures_lite::{
    AsyncReadExt,
    future,
};
use keyboard_types::Modifiers;
use portable_pty::{
    CommandBuilder,
    PtySize,
    native_pty_system,
};
use rio_vt::{
    ansi::CursorShape,
    crosswords::{
        Crosswords,
        CrosswordsSize,
    },
    event::{
        EventListener,
        RioEvent,
        WindowId,
    },
    performer::handler::Processor,
};

use crate::handle::{
    TerminalCleaner,
    TerminalError,
    TerminalHandle,
    TerminalId,
    TerminalInner,
};

/// Queues input for the PTY, written on its own thread.
pub(crate) struct PtyWriter {
    sender: Sender<Vec<u8>>,
}

impl PtyWriter {
    fn spawn(mut writer: Box<dyn Write + Send>) -> Self {
        let (sender, receiver) = channel::<Vec<u8>>();
        let _ = std::thread::Builder::new()
            .name("pty-writer".into())
            .spawn(move || {
                while let Ok(data) = receiver.recv() {
                    if writer
                        .write_all(&data)
                        .and_then(|_| writer.flush())
                        .is_err()
                    {
                        break;
                    }
                }
            });
        Self { sender }
    }

    pub(crate) fn write(&self, data: &[u8]) -> Result<(), TerminalError> {
        self.sender
            .send(data.to_vec())
            .map_err(|_| TerminalError::WriteError("PTY writer is closed".to_string()))
    }
}

/// Listener proxy passed into rio-vt's `Crosswords`. Routes its side-effects
/// (PtyWrite, Title, ClipboardStore) into the freya-side state.
#[derive(Clone)]
pub struct EventProxy {
    pub(crate) writer: Rc<RefCell<Option<PtyWriter>>>,
    pub(crate) title: Rc<RefCell<Option<String>>>,
    pub(crate) title_notifier: ArcNotify,
    pub(crate) clipboard_content: Rc<RefCell<Option<String>>>,
    pub(crate) clipboard_notifier: ArcNotify,
}

impl EventListener for EventProxy {
    fn send_event(&self, event: RioEvent, _window_id: WindowId) {
        match event {
            RioEvent::PtyWrite(_route, text) => {
                if let Some(writer) = &*self.writer.borrow() {
                    let _ = writer.write(text.as_bytes());
                }
            }
            RioEvent::Title(t) => {
                *self.title.borrow_mut() = Some(t);
                self.title_notifier.notify();
            }
            RioEvent::ResetTitle => {
                *self.title.borrow_mut() = None;
                self.title_notifier.notify();
            }
            RioEvent::ClipboardStore(_, text) => {
                *self.clipboard_content.borrow_mut() = Some(text);
                self.clipboard_notifier.notify();
            }
            // Bell, Wakeup, ColorRequest, etc.
            _ => {}
        }
    }
}

/// Spawn a PTY and return a `TerminalHandle`.
pub(crate) fn spawn_pty(
    id: TerminalId,
    command: CommandBuilder,
    scrollback_size: usize,
) -> Result<TerminalHandle, TerminalError> {
    let writer = Rc::new(RefCell::new(None::<PtyWriter>));
    let closer_notifier = ArcNotify::new();
    let output_notifier = ArcNotify::new();
    let title_notifier = ArcNotify::new();
    let title: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let clipboard_content: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let clipboard_notifier = ArcNotify::new();

    let event_proxy = EventProxy {
        writer: writer.clone(),
        title: title.clone(),
        title_notifier: title_notifier.clone(),
        clipboard_content: clipboard_content.clone(),
        clipboard_notifier: clipboard_notifier.clone(),
    };

    let term = Rc::new(RefCell::new(Crosswords::new(
        CrosswordsSize::new(80, 24),
        CursorShape::Block,
        event_proxy,
        WindowId::from(0),
        0,
        scrollback_size,
    )));

    let pair = native_pty_system()
        .openpty(PtySize::default())
        .map_err(|_| TerminalError::NotInitialized)?;
    *writer.borrow_mut() = Some(PtyWriter::spawn(
        pair.master
            .take_writer()
            .map_err(|_| TerminalError::NotInitialized)?,
    ));

    pair.slave
        .spawn_command(command)
        .map_err(|_| TerminalError::NotInitialized)?;
    let mut reader = blocking::Unblock::new(
        pair.master
            .try_clone_reader()
            .map_err(|_| TerminalError::NotInitialized)?,
    );

    let inner = Rc::new(RefCell::new(TerminalInner {
        master: pair.master,
        last_write_time: Instant::now(),
        pressed_button: None,
        modifiers: Modifiers::empty(),
    }));

    let platform = Platform::get();
    let pty_task = spawn_forever({
        let term = term.clone();
        let writer = writer.clone();
        let closer_notifier = closer_notifier.clone();
        let output_notifier = output_notifier.clone();
        async move {
            let mut processor = Processor::default();
            loop {
                let mut buf = [0u8; 4096];
                let read = async { Some(reader.read(&mut buf).await) };
                let result = if let Some(deadline) = processor.sync_timeout().sync_timeout() {
                    let expiry = async {
                        Timer::at(deadline).await;
                        None
                    };
                    future::or(read, expiry).await
                } else {
                    read.await
                };
                match result {
                    None => {
                        processor.stop_sync(&mut *term.borrow_mut());
                        output_notifier.notify();
                        platform.send(UserEvent::RequestRedraw);
                    }
                    Some(Ok(0)) | Some(Err(_)) => break,
                    Some(Ok(n)) => {
                        processor.advance(&mut *term.borrow_mut(), &buf[..n]);
                        output_notifier.notify();
                        platform.send(UserEvent::RequestRedraw);
                    }
                }
            }
            // PTY closed: drop the writer and notify observers.
            *writer.borrow_mut() = None;
            closer_notifier.notify();
            platform.send(UserEvent::RequestRedraw);
        }
    });

    Ok(TerminalHandle {
        closer_notifier: closer_notifier.clone(),
        cleaner: Rc::new(TerminalCleaner {
            writer: writer.clone(),
            pty_task,
            closer_notifier,
        }),
        id,
        term,
        writer,
        inner,
        title,
        title_notifier,
        clipboard_content,
        clipboard_notifier,
        output_notifier,
    })
}
