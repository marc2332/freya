use std::{
    cell::RefCell,
    rc::Rc,
    sync::Arc,
};

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
    StreamExt,
};
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
    let buffer = Rc::new(RefCell::new(TerminalBuffer::default()));
    let parser = Rc::new(RefCell::new(Parser::new(24, 80, 1000)));
    let writer = Rc::new(RefCell::new(None::<Box<dyn std::io::Write + Send>>));
    let closer_notifier = ArcNotify::new();

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
                            *writer.borrow_mut() = None;
                            closer_notifier.notify();
                            platform.send(UserEvent::RequestRedraw);
                            break;
                        }
                        let parser = parser.borrow();
                        let (rows, cols) = parser.screen().size();
                        let rows_vec: Vec<Vec<vt100::Cell>> = (0..rows)
                            .map(|r| {
                                (0..cols)
                                    .map(|c| parser.screen().cell(r, c).unwrap().clone())
                                    .collect()
                            })
                            .collect();

                        let (cur_r, cur_c) = parser.screen().cursor_position();
                        let new_buffer = TerminalBuffer {
                            rows: rows_vec,
                            cursor_row: cur_r as usize,
                            cursor_col: cur_c as usize,
                            cols: cols as usize,
                            rows_count: rows as usize,
                            selection: None,
                        };
                        *buffer.borrow_mut() = new_buffer;
                        platform.send(UserEvent::RequestRedraw);
                    }
                    resize = resize_rx.next().fuse() => {
                        if let Some((rows, cols)) = resize {
                            parser.borrow_mut().screen_mut().set_size(rows, cols);

                            // Resize parser
                            let parser = parser.borrow();
                            let (rows, cols) = parser.screen().size();
                            let rows_vec: Vec<Vec<vt100::Cell>> = (0..rows)
                                .map(|r| {
                                    (0..cols)
                                        .map(|c| parser.screen().cell(r, c).unwrap().clone())
                                        .collect()
                                })
                                .collect();

                            let (cur_r, cur_c) = parser.screen().cursor_position();
                            let new_buffer = TerminalBuffer {
                                rows: rows_vec,
                                cursor_row: cur_r as usize,
                                cursor_col: cur_c as usize,
                                cols: cols as usize,
                                rows_count: rows as usize,
                                selection: None,
                            };
                            *buffer.borrow_mut() = new_buffer;

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

    spawn_forever({
        let writer = writer.clone();
        async move {
            loop {
                let mut buf = [0u8; 4096];

                match reader.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let data = &buf[..n];

                        parser.borrow_mut().process(data);

                        let responses = check_for_terminal_queries(data, &parser.borrow());
                        if !responses.is_empty()
                            && let Some(writer) = &mut *writer.borrow_mut()
                        {
                            for response in responses {
                                let _ = writer.write_all(&response);
                            }
                            let _ = writer.flush();
                        }

                        let _ = update_tx.unbounded_send(());
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
            task,
            closer_notifier,
        }),
        id,
        buffer,
        writer,
        resize_sender: Arc::new(resize_tx),
    })
}
