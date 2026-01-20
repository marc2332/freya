use std::{
    io::Read,
    sync::{
        Arc,
        Mutex,
        RwLock,
    },
};

use freya_core::prelude::{
    Platform,
    UserEvent,
    spawn,
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
    let (pty_tx, pty_rx) = std::sync::mpsc::channel::<(u16, u16)>();

    let id = TerminalId::new();
    let buffer = Arc::new(Mutex::new(TerminalBuffer::default()));
    let parser = Arc::new(RwLock::new(Parser::new(24, 80, 1000)));

    let parser_for_async = parser.clone();
    let tab_buffer = buffer.clone();
    let update_tx_clone = update_tx.clone();

    let platform = Platform::get();

    // Async task to update UI buffer when notified
    spawn(async move {
        loop {
            futures_util::select! {
                _ = update_rx.next().fuse() => {
                    if let Ok(p) = parser_for_async.read() {
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
                        };

                        if let Ok(mut buf) = tab_buffer.lock() {
                            *buf = new_buffer;
                            platform.send(UserEvent::RequestRedraw);
                        }
                    }
                }
                resize = resize_rx.next().fuse() => {
                    if let Some((rows, cols)) = resize {
                        if let Ok(mut p) = parser_for_async.write() {
                            p.screen_mut().set_size(rows, cols);
                        }
                        if let Ok(p) = parser_for_async.read() {
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
                            };

                            if let Ok(mut buf) = tab_buffer.lock() {
                                *buf = new_buffer;
                            }
                        }
                    }
                }
            }
        }
    });

    // Writer holder for PTY thread
    let writer_holder = Arc::new(Mutex::new(None::<Box<dyn std::io::Write + Send>>));

    // PTY thread - reads from PTY, processes through parser
    let writer_holder_for_pty = writer_holder.clone();
    blocking::unblock(move || {
        let pty_system = native_pty_system();
        match pty_system.openpty(PtySize::default()) {
            Ok(pair) => {
                if let Ok(w) = pair.master.take_writer() {
                    *writer_holder_for_pty.lock().unwrap() = Some(w);
                } else {
                    return;
                }

                if let Err(_e) = pair.slave.spawn_command(command) {
                    return;
                }

                match pair.master.try_clone_reader() {
                    Ok(mut reader) => {
                        let master_for_resize = pair.master;
                        blocking::unblock(move || {
                            for (rows, cols) in pty_rx {
                                let size = PtySize {
                                    rows,
                                    cols,
                                    pixel_width: 0,
                                    pixel_height: 0,
                                };
                                let _ = master_for_resize.resize(size);
                            }
                        })
                        .detach();

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
                                    if !responses.is_empty() {
                                        if let Ok(mut guard) = writer_holder_for_pty.lock() {
                                            if let Some(w) = guard.as_mut() {
                                                for response in responses {
                                                    let _ = w.write_all(&response);
                                                }
                                                let _ = w.flush();
                                            }
                                        }
                                    }

                                    let _ = update_tx_clone.unbounded_send(());
                                }
                                Err(_) => break,
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    })
    .detach();

    Ok(TerminalHandle {
        id,
        buffer,
        writer: writer_holder,
        resize_holder: Arc::new(Mutex::new(Some(resize_tx))),
        pty_resize_holder: Arc::new(Mutex::new(Some(pty_tx))),
    })
}
