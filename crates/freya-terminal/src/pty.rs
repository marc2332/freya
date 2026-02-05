use std::{
    io::Read,
    sync::{
        Arc,
        Mutex,
        RwLock,
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
use futures_lite::StreamExt;
use futures_util::FutureExt;
use portable_pty::{
    CommandBuilder,
    PtySize,
    native_pty_system,
};
use vt100::Parser;

use crate::{
    buffer::TerminalBuffer,
    handle::{
        TerminalCleaner,
        TerminalError,
        TerminalHandle,
        TerminalId,
    },
    parser::check_for_terminal_queries,
};

/// Spawn a PTY and return a TerminalHandle
pub(crate) fn spawn_pty(command: CommandBuilder) -> Result<TerminalHandle, TerminalError> {
    let (update_tx, mut update_rx) = futures_channel::mpsc::unbounded::<()>();
    let (resize_tx, mut resize_rx) = futures_channel::mpsc::unbounded::<(u16, u16)>();

    let id = TerminalId::new();
    let buffer = Arc::new(Mutex::new(TerminalBuffer::default()));
    let parser = Arc::new(RwLock::new(Parser::new(24, 80, 1000)));
    let writer = Arc::new(Mutex::new(None::<Box<dyn std::io::Write + Send>>));
    let closer_notifier = ArcNotify::new();

    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize::default())
        .map_err(|_| TerminalError::NotInitialized)?;
    let master_writer = pair
        .master
        .take_writer()
        .map_err(|_| TerminalError::NotInitialized)?;
    *writer.lock().unwrap() = Some(master_writer);

    pair.slave
        .spawn_command(command)
        .map_err(|_| TerminalError::NotInitialized)?;
    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|_| TerminalError::NotInitialized)?;
    let platform = Platform::get();
    let task = spawn_forever({
        let parser = parser.clone();
        let buffer = buffer.clone();
        let closer_notifier = closer_notifier.clone();
        let writer = writer.clone();
        async move {
            loop {
                futures_util::select! {
                    update = update_rx.next().fuse() => {
                        if update.is_none() {
                            // Channel closed - PTY exited
                            *writer.lock().unwrap() = None;
                            closer_notifier.notify();
                            platform.send(UserEvent::RequestRedraw);
                            break;
                        }
                        if let Ok(p) = parser.read() {
                            let (rows, cols) = p.screen().size();
                            let rows_vec: Vec<Vec<vt100::Cell>> = (0..rows)
                                .map(|r| {
                                    (0..cols)
                                        .map(|c| p.screen().cell(r, c).unwrap().clone())
                                        .collect()
                                })
                                .collect();

                            let (cur_r, cur_c) = p.screen().cursor_position();
                            let new_buffer = TerminalBuffer {
                                rows: rows_vec,
                                cursor_row: cur_r as usize,
                                cursor_col: cur_c as usize,
                                cols: cols as usize,
                                rows_count: rows as usize,
                                selection: None,
                            };

                            if let Ok(mut buf) = buffer.lock() {
                                *buf = new_buffer;
                                platform.send(UserEvent::RequestRedraw);
                            }
                        }
                    }
                    resize = resize_rx.next().fuse() => {
                        if let Some((rows, cols)) = resize {
                            if let Ok(mut p) = parser.write() {
                                p.screen_mut().set_size(rows, cols);
                            }

                            // Resize parser
                            if let Ok(p) = parser.read() {
                                let (rows, cols) = p.screen().size();
                                let rows_vec: Vec<Vec<vt100::Cell>> = (0..rows)
                                    .map(|r| {
                                        (0..cols)
                                            .map(|c| p.screen().cell(r, c).unwrap().clone())
                                            .collect()
                                    })
                                    .collect();

                                let (cur_r, cur_c) = p.screen().cursor_position();
                                let new_buffer = TerminalBuffer {
                                    rows: rows_vec,
                                    cursor_row: cur_r as usize,
                                    cursor_col: cur_c as usize,
                                    cols: cols as usize,
                                    rows_count: rows as usize,
                                    selection: None,
                                };

                                if let Ok(mut buf) = buffer.lock() {
                                    *buf = new_buffer;
                                }
                            }

                            // Resize PTY
                            let size = PtySize {
                                rows,
                                cols,
                                pixel_width: 0,
                                pixel_height: 0,
                            };
                            let _ = pair.master.resize(size);
                        }
                    }
                }
            }
        }
    });

    blocking::unblock({
        let writer = writer.clone();
        move || {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let data = &buf[..n];

                        if let Ok(mut p) = parser.write() {
                            p.process(data);
                        }

                        let responses = check_for_terminal_queries(data, &parser);
                        if !responses.is_empty()
                            && let Ok(mut guard) = writer.lock()
                            && let Some(w) = guard.as_mut()
                        {
                            for response in responses {
                                let _ = w.write_all(&response);
                            }
                            let _ = w.flush();
                        }

                        let _ = update_tx.unbounded_send(());
                    }
                    Err(_) => break,
                }
            }
        }
    })
    .detach();
    Ok(TerminalHandle {
        closer_notifier: closer_notifier.clone(),
        cleaner: Arc::new(TerminalCleaner {
            writer: writer.clone(),
            task,
            closer_notifier,
        }),
        id,
        buffer,
        writer,
        resize_sender: Arc::new(resize_tx),
    })
}
