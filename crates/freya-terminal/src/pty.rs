use std::{
    cell::RefCell,
    rc::Rc,
    time::Instant,
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

/// Command to control terminal scrollback position.
#[derive(Debug, Clone)]
pub enum ScrollCommand {
    /// Scroll by a relative number of lines (positive = up, negative = down)
    Delta(i32),
    /// Jump to the bottom (most recent output)
    ToBottom,
}

/// Query the maximum scrollback available without disturbing the viewport.
/// Saves current scrollback, queries max, and restores.
fn query_max_scrollback(parser: &mut Parser) -> usize {
    let saved = parser.screen().scrollback();
    parser.screen_mut().set_scrollback(usize::MAX);
    let max = parser.screen().scrollback();
    parser.screen_mut().set_scrollback(saved);
    max
}

/// Extract visible cells from the parser at the current scrollback position.
fn extract_buffer(
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
    }
}

/// Spawn a PTY and return a TerminalHandle
pub(crate) fn spawn_pty(
    id: TerminalId,
    command: CommandBuilder,
    scrollback_size: usize,
) -> Result<TerminalHandle, TerminalError> {
    let (update_tx, mut update_rx) = futures_channel::mpsc::unbounded::<()>();
    let (resize_tx, mut resize_rx) = futures_channel::mpsc::unbounded::<(u16, u16)>();
    let (scroll_tx, mut scroll_rx) = futures_channel::mpsc::unbounded::<ScrollCommand>();

    let buffer = Rc::new(RefCell::new(TerminalBuffer::default()));
    let parser = Rc::new(RefCell::new(Parser::new(24, 80, scrollback_size)));
    let writer = Rc::new(RefCell::new(None::<Box<dyn std::io::Write + Send>>));
    let closer_notifier = ArcNotify::new();
    let output_notifier = ArcNotify::new();

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
    let reader_task = spawn_forever({
        let parser = parser.clone();
        let buffer = buffer.clone();
        let closer_notifier = closer_notifier.clone();
        let writer = writer.clone();
        async move {
            loop {
                futures_util::select! {
                     update = update_rx.next().fuse() => {
                        if update.is_none() {
                            *writer.borrow_mut() = None;
                            closer_notifier.notify();
                            platform.send(UserEvent::RequestRedraw);
                            break;
                        }
                        let mut parser = parser.borrow_mut();
                        let total_scrollback = query_max_scrollback(&mut parser);

                        let mut buffer = buffer.borrow_mut();
                        parser.screen_mut().set_scrollback(buffer.scroll_offset);
                        let mut new_buffer = extract_buffer(&parser, buffer.scroll_offset, total_scrollback);
                        parser.screen_mut().set_scrollback(0);

                        new_buffer.selection = buffer.selection.take();
                        *buffer = new_buffer;
                        platform.send(UserEvent::RequestRedraw);
                    }
                    resize = resize_rx.next().fuse() => {
                        if let Some((rows, cols)) = resize {
                            let mut parser = parser.borrow_mut();
                            parser.screen_mut().set_size(rows, cols);

                            let total_scrollback = query_max_scrollback(&mut parser);
                            let mut buffer = buffer.borrow_mut();
                            buffer.scroll_offset = buffer.scroll_offset.min(total_scrollback);

                            parser.screen_mut().set_scrollback(buffer.scroll_offset);
                            let mut new_buffer = extract_buffer(&parser, buffer.scroll_offset, total_scrollback);
                            parser.screen_mut().set_scrollback(0);

                            new_buffer.selection = buffer.selection.take();
                            *buffer = new_buffer;

                            let (rows, cols) = parser.screen().size();
                            let _ = pair.master.resize(PtySize {
                                rows,
                                cols,
                                pixel_width: 0,
                                pixel_height: 0,
                            });
                        }
                    }
                    scroll = scroll_rx.next().fuse() => {
                        if let Some(cmd) = scroll {
                            let mut parser = parser.borrow_mut();

                            if parser.screen().alternate_screen() {
                                continue;
                            }

                            let total_scrollback = query_max_scrollback(&mut parser);

                            let mut buffer = buffer.borrow_mut();
                            let offset = {
                                match cmd {
                                    ScrollCommand::Delta(_) => {
                                        buffer.scroll_offset = buffer.scroll_offset.min(total_scrollback);
                                    }
                                    ScrollCommand::ToBottom => {
                                        buffer.scroll_offset = 0;
                                    }
                                }
                                buffer.scroll_offset
                            };

                            parser.screen_mut().set_scrollback(offset);
                            let mut new_buffer = extract_buffer(&parser, offset, total_scrollback);
                            parser.screen_mut().set_scrollback(0);

                            new_buffer.selection = buffer.selection.take();
                            *buffer = new_buffer;
                            platform.send(UserEvent::RequestRedraw);
                        }
                    }
                }
            }
        }
    });

    let pty_task = spawn_forever({
        let writer = writer.clone();
        let parser = parser.clone();
        let output_notifier = output_notifier.clone();
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
        resize_sender: Rc::new(resize_tx),
        scroll_sender: Rc::new(scroll_tx),
        output_notifier,
        last_write_time: Rc::new(RefCell::new(Instant::now())),
    })
}
