use std::{
    cell::RefCell,
    path::PathBuf,
    rc::Rc,
    time::Instant,
};

use alacritty_terminal::{
    event::{
        Event as AlacrittyEvent,
        EventListener,
    },
    grid::Dimensions,
    term::{
        Config as TermConfig,
        Term,
    },
    vte::{
        Parser as VteParser,
        ansi::{
            Processor,
            StdSyncHandler,
        },
    },
};
use freya_core::{
    notify::ArcNotify,
    prelude::{
        Platform,
        UserEvent,
        spawn_forever,
    },
};
use futures_lite::AsyncReadExt;
use keyboard_types::Modifiers;
use portable_pty::{
    CommandBuilder,
    MasterPty,
    PtySize,
    native_pty_system,
};

use crate::{
    handle::{
        TerminalCleaner,
        TerminalError,
        TerminalHandle,
        TerminalId,
    },
    osc7::{
        CwdSink,
        parse_cwd_url,
    },
};

/// `Dimensions` impl passed to `Term::new` / `Term::resize`.
#[derive(Clone, Copy)]
pub(crate) struct TermSize {
    pub screen_lines: usize,
    pub columns: usize,
}

impl Dimensions for TermSize {
    fn total_lines(&self) -> usize {
        self.screen_lines
    }

    fn screen_lines(&self) -> usize {
        self.screen_lines
    }

    fn columns(&self) -> usize {
        self.columns
    }
}

/// Listener proxy passed into alacritty's `Term`. Routes its side-effects
/// (PtyWrite, Title, ClipboardStore) into the freya-side state.
#[derive(Clone)]
pub struct EventProxy {
    pub(crate) writer: Rc<RefCell<Option<Box<dyn std::io::Write + Send>>>>,
    pub(crate) title: Rc<RefCell<Option<String>>>,
    pub(crate) title_notifier: ArcNotify,
    pub(crate) clipboard_content: Rc<RefCell<Option<String>>>,
    pub(crate) clipboard_notifier: ArcNotify,
}

impl EventListener for EventProxy {
    fn send_event(&self, event: AlacrittyEvent) {
        match event {
            AlacrittyEvent::PtyWrite(text) => {
                if let Some(writer) = &mut *self.writer.borrow_mut() {
                    let _ = writer.write_all(text.as_bytes());
                    let _ = writer.flush();
                }
            }
            AlacrittyEvent::Title(t) => {
                *self.title.borrow_mut() = Some(t);
                self.title_notifier.notify();
            }
            AlacrittyEvent::ResetTitle => {
                *self.title.borrow_mut() = None;
                self.title_notifier.notify();
            }
            AlacrittyEvent::ClipboardStore(_, text) => {
                *self.clipboard_content.borrow_mut() = Some(text);
                self.clipboard_notifier.notify();
            }
            // Bell, MouseCursorDirty, ChildExit, ColorRequest, etc.
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
    let writer = Rc::new(RefCell::new(None::<Box<dyn std::io::Write + Send>>));
    let closer_notifier = ArcNotify::new();
    let output_notifier = ArcNotify::new();
    let title_notifier = ArcNotify::new();
    let cwd: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
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

    let term_config = TermConfig {
        scrolling_history: scrollback_size,
        ..TermConfig::default()
    };
    let initial_size = TermSize {
        screen_lines: 24,
        columns: 80,
    };
    let term = Rc::new(RefCell::new(Term::new(
        term_config,
        &initial_size,
        event_proxy,
    )));
    let processor: Rc<RefCell<Processor<StdSyncHandler>>> = Rc::new(RefCell::new(Processor::new()));

    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize::default())
        .map_err(|_| TerminalError::NotInitialized)?;
    let master_writer = pair
        .master
        .take_writer()
        .map_err(|_| TerminalError::NotInitialized)?;
    *writer.borrow_mut() = Some(master_writer);

    pair.slave
        .spawn_command(command)
        .map_err(|_| TerminalError::NotInitialized)?;
    let reader = pair
        .master
        .try_clone_reader()
        .map_err(|_| TerminalError::NotInitialized)?;
    let mut reader = blocking::Unblock::new(reader);

    let master: Rc<RefCell<Box<dyn MasterPty + Send>>> = Rc::new(RefCell::new(pair.master));

    let platform = Platform::get();
    let pty_task = spawn_forever({
        let term = term.clone();
        let writer = writer.clone();
        let closer_notifier = closer_notifier.clone();
        let output_notifier = output_notifier.clone();
        let cwd = cwd.clone();
        async move {
            // Side-channel parser for OSC 7 (cwd), which alacritty drops.
            let mut cwd_parser = VteParser::new();
            let mut cwd_sink = CwdSink::default();
            loop {
                let mut buf = [0u8; 4096];
                match reader.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let data = &buf[..n];
                        processor
                            .borrow_mut()
                            .advance(&mut *term.borrow_mut(), data);

                        cwd_parser.advance(&mut cwd_sink, data);
                        if let Some(url) = cwd_sink.take() {
                            *cwd.borrow_mut() = Some(parse_cwd_url(&url));
                        }

                        output_notifier.notify();
                        platform.send(UserEvent::RequestRedraw);
                    }
                    Err(_) => break,
                }
            }
            // PTY closed — drop the writer and notify observers.
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
        master,
        cwd,
        title,
        title_notifier,
        clipboard_content,
        clipboard_notifier,
        output_notifier,
        last_write_time: Rc::new(RefCell::new(Instant::now())),
        pressed_button: Rc::new(RefCell::new(None)),
        modifiers: Rc::new(RefCell::new(Modifiers::empty())),
        dragging_selection: Rc::new(RefCell::new(false)),
    })
}
