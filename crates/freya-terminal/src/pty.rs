use std::{cell::RefCell, path::PathBuf, rc::Rc, time::Instant};

use freya_core::{
    notify::ArcNotify,
    prelude::{Platform, UserEvent, spawn_forever},
};
use futures_lite::AsyncReadExt;
use keyboard_types::Modifiers;
use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};
use termwiz::escape::{
    Action, CSI, OperatingSystemCommand,
    csi::{Cursor, Device},
    parser::Parser as TermwizParser,
};
use vt100::Parser;

use crate::{
    buffer::TerminalBuffer,
    handle::{TerminalCleaner, TerminalError, TerminalHandle, TerminalId},
};

/// Query the maximum scrollback available without disturbing the viewport.
/// Saves current scrollback, queries max, and restores.
pub(crate) fn query_max_scrollback(parser: &mut Parser) -> usize {
    let saved = parser.screen().scrollback();
    parser.screen_mut().set_scrollback(usize::MAX);
    let max = parser.screen().scrollback();
    parser.screen_mut().set_scrollback(saved);
    max
}

/// Extract visible cells from the parser at the current scrollback position.
pub(crate) fn extract_buffer(
    parser: &Parser,
    scroll_offset: usize,
    total_scrollback: usize,
) -> TerminalBuffer {
    let (rows, cols) = parser.screen().size();
    let rows_vec: Vec<Vec<vt100::Cell>> = (0..rows)
        .map(|r| {
            (0..cols)
                .filter_map(|c| parser.screen().cell(r, c).cloned())
                .collect()
        })
        .collect();
    let (cur_r, cur_c) = parser.screen().cursor_position();
    TerminalBuffer {
        rows: rows_vec,
        cursor_row: cur_r as usize,
        cursor_col: cur_c as usize,
        cols: cols as usize,
        rows_count: rows as usize,
        selection: None,
        scroll_offset,
        total_scrollback,
        cursor_visible: !parser.screen().hide_cursor(),
    }
}

/// Spawn a PTY and return a TerminalHandle.
pub(crate) fn spawn_pty(
    id: TerminalId,
    command: CommandBuilder,
    scrollback_size: usize,
) -> Result<TerminalHandle, TerminalError> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize::default())
        .map_err(|_| TerminalError::NotInitialized)?;

    pair.slave
        .spawn_command(command)
        .map_err(|_| TerminalError::NotInitialized)?;

    setup_terminal_from_master(id, pair.master, scrollback_size)
}

/// Wire up a [`MasterPty`] (reader, writer, async tasks) into a [`TerminalHandle`].
///
/// This is the shared post-PTY-creation path used by both [`spawn_pty`] (which
/// opens its own PTY + spawns a command) and [`TerminalHandle::from_fd`] (which
/// wraps a daemon-provided fd).
pub(crate) fn setup_terminal_from_master(
    id: TerminalId,
    master: Box<dyn MasterPty + Send>,
    scrollback_size: usize,
) -> Result<TerminalHandle, TerminalError> {
    let (update_tx, mut update_rx) = futures_channel::mpsc::unbounded::<()>();

    let buffer = Rc::new(RefCell::new(TerminalBuffer::default()));
    let parser = Rc::new(RefCell::new(Parser::new(24, 80, scrollback_size)));
    let writer = Rc::new(RefCell::new(None::<Box<dyn std::io::Write + Send>>));
    let closer_notifier = ArcNotify::new();
    let output_notifier = ArcNotify::new();
    let title_notifier = ArcNotify::new();
    let cwd: Rc<RefCell<Option<PathBuf>>> = Rc::new(RefCell::new(None));
    let title: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let clipboard_content: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let clipboard_notifier = ArcNotify::new();

    let master_writer = master
        .take_writer()
        .map_err(|_| TerminalError::NotInitialized)?;
    *writer.borrow_mut() = Some(master_writer);

    let reader = master
        .try_clone_reader()
        .map_err(|_| TerminalError::NotInitialized)?;
    let mut reader = blocking::Unblock::new(reader);

    let master: Rc<RefCell<Box<dyn MasterPty + Send>>> = Rc::new(RefCell::new(master));

    let platform = Platform::get();
    let reader_task = spawn_forever({
        let parser = parser.clone();
        let buffer = buffer.clone();
        let closer_notifier = closer_notifier.clone();
        let writer = writer.clone();
        async move {
            use futures_lite::StreamExt;
            while let Some(()) = update_rx.next().await {
                let mut parser = parser.borrow_mut();
                let total_scrollback = query_max_scrollback(&mut parser);

                let mut buffer = buffer.borrow_mut();
                let old_total_scrollback = buffer.total_scrollback;
                let delta = total_scrollback.saturating_sub(old_total_scrollback);
                parser.screen_mut().set_scrollback(buffer.scroll_offset);
                let mut new_buffer =
                    extract_buffer(&parser, buffer.scroll_offset, total_scrollback);
                parser.screen_mut().set_scrollback(0);

                new_buffer.selection = buffer.selection.take().map(|mut selection| {
                    selection.start_scroll = selection.start_scroll.saturating_add(delta);
                    selection.end_scroll = selection.end_scroll.saturating_add(delta);
                    selection
                });
                *buffer = new_buffer;
                platform.send(UserEvent::RequestRedraw);
            }
            // Channel closed — PTY exited
            *writer.borrow_mut() = None;
            closer_notifier.notify();
            platform.send(UserEvent::RequestRedraw);
        }
    });

    let pty_task = spawn_forever({
        let writer = writer.clone();
        let parser = parser.clone();
        let output_notifier = output_notifier.clone();
        let cwd = cwd.clone();
        let title = title.clone();
        let title_notifier = title_notifier.clone();
        let clipboard_content = clipboard_content.clone();
        let clipboard_notifier = clipboard_notifier.clone();
        async move {
            let mut tw_parser = TermwizParser::new();
            loop {
                let mut buf = [0u8; 4096];

                match reader.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let data = &buf[..n];

                        parser.borrow_mut().process(data);

                        // Use termwiz to detect terminal queries and OSC sequences
                        let actions = tw_parser.parse_as_vec(data);
                        let mut responses: Vec<Vec<u8>> = Vec::new();

                        for action in actions {
                            match action {
                                Action::CSI(CSI::Device(dev)) => match *dev {
                                    Device::RequestPrimaryDeviceAttributes => {
                                        responses.push(b"\x1b[?62;22c".to_vec());
                                    }
                                    Device::RequestSecondaryDeviceAttributes => {
                                        responses.push(b"\x1b[>0;0;0c".to_vec());
                                    }
                                    Device::StatusReport => {
                                        responses.push(b"\x1b[0n".to_vec());
                                    }
                                    _ => {}
                                },
                                Action::CSI(CSI::Cursor(Cursor::RequestActivePositionReport)) => {
                                    let p = parser.borrow();
                                    let (row, col) = p.screen().cursor_position();
                                    let response = format!("\x1b[{};{}R", row + 1, col + 1);
                                    responses.push(response.into_bytes());
                                }
                                Action::OperatingSystemCommand(osc) => match *osc {
                                    OperatingSystemCommand::CurrentWorkingDirectory(url) => {
                                        // Strip file:// prefix if present
                                        let path =
                                            if let Some(stripped) = url.strip_prefix("file://") {
                                                // file:///path or file://hostname/path
                                                if let Some(rest) = stripped.strip_prefix('/') {
                                                    PathBuf::from(format!("/{rest}"))
                                                } else if let Some((_host, path)) =
                                                    stripped.split_once('/')
                                                {
                                                    PathBuf::from(format!("/{path}"))
                                                } else {
                                                    PathBuf::from(stripped)
                                                }
                                            } else {
                                                PathBuf::from(url)
                                            };
                                        *cwd.borrow_mut() = Some(path);
                                    }
                                    OperatingSystemCommand::SetWindowTitle(t)
                                    | OperatingSystemCommand::SetIconNameAndWindowTitle(t) => {
                                        *title.borrow_mut() = Some(t);
                                        title_notifier.notify();
                                    }
                                    OperatingSystemCommand::SetSelection(_sel, text) => {
                                        *clipboard_content.borrow_mut() = Some(text);
                                        clipboard_notifier.notify();
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }

                        if !responses.is_empty()
                            && let Some(writer) = &mut *writer.borrow_mut()
                        {
                            for response in responses {
                                let _ = writer.write_all(&response);
                            }
                            let _ = writer.flush();
                        }

                        let _ = update_tx.unbounded_send(());
                        output_notifier.notify();
                    }
                    Err(_) => break,
                }
            }
        }
    });

    Ok(TerminalHandle {
        closer_notifier: closer_notifier.clone(),
        cleaner: Rc::new(TerminalCleaner {
            writer: writer.clone(),
            reader_task,
            pty_task,
            closer_notifier,
        }),
        id,
        buffer,
        parser,
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
    })
}
