#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    io::{
        Read,
        Write,
    },
    sync::{
        Arc,
        Mutex,
        RwLock,
    },
};

use freya::{
    engine::prelude::{
        Paint,
        PaintStyle,
        ParagraphBuilder,
        ParagraphStyle,
        SkRect,
        TextStyle,
    },
    prelude::*,
};
use freya_core::{
    data::LayoutData,
    element::ElementExt,
    tree::DiffModifies,
};
use futures_channel::mpsc::UnboundedSender;
use futures_lite::StreamExt;
use futures_util::FutureExt;
use keyboard_types::Modifiers;
use portable_pty::{
    CommandBuilder,
    PtySize,
    native_pty_system,
};
use torin::prelude::Length;
use vt100::Parser;

#[derive(Clone, PartialEq, Default)]
struct TerminalBuffer {
    rows: Vec<Vec<vt100::Cell>>,
    cursor_row: usize,
    cursor_col: usize,
    cols: usize,
    rows_count: usize,
}

struct TerminalElement {
    id: usize,
    layout_data: LayoutData,
    buffer: Arc<Mutex<TerminalBuffer>>,
    font_family: String,
    font_size: f32,
    fg: Color,
    bg: Color,
    resize_holder: std::sync::Arc<std::sync::Mutex<Option<UnboundedSender<(u16, u16)>>>>,
    pty_resize_holder:
        std::sync::Arc<std::sync::Mutex<Option<std::sync::mpsc::Sender<(u16, u16)>>>>,
}

impl PartialEq for TerminalElement {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.font_size == other.font_size
            && self.font_family == other.font_family
            && self.fg == other.fg
            && self.bg == other.bg
    }
}

impl TerminalElement {
    fn new(
        id: usize,
        buffer: Arc<Mutex<TerminalBuffer>>,
        resize_holder: std::sync::Arc<std::sync::Mutex<Option<UnboundedSender<(u16, u16)>>>>,
        pty_resize_holder: std::sync::Arc<
            std::sync::Mutex<Option<std::sync::mpsc::Sender<(u16, u16)>>>,
        >,
    ) -> Self {
        Self {
            id,
            layout_data: Default::default(),
            buffer,
            font_family: "Cascadia Code".to_string(),
            font_size: 14.,
            fg: (220, 220, 220).into(),
            bg: (10, 10, 10).into(),
            resize_holder,
            pty_resize_holder,
        }
    }

    /// Map vt100 colors (including 256-color palette indexes) to Color
    /// If `is_bg` is true, Default maps to background color instead of foreground
    fn map_vt100_color_with_default(&self, c: vt100::Color, is_bg: bool) -> Color {
        match c {
            vt100::Color::Default => {
                if is_bg {
                    self.bg
                } else {
                    self.fg
                }
            }
            vt100::Color::Rgb(r, g, b) => Color::from_rgb(r, g, b),
            vt100::Color::Idx(idx) => {
                let i = idx as usize;
                const ANSI: [(u8, u8, u8); 16] = [
                    (0, 0, 0),
                    (128, 0, 0),
                    (0, 128, 0),
                    (128, 128, 0),
                    (0, 0, 128),
                    (128, 0, 128),
                    (0, 128, 128),
                    (192, 192, 192),
                    (128, 128, 128),
                    (255, 0, 0),
                    (0, 255, 0),
                    (255, 255, 0),
                    (0, 0, 255),
                    (255, 0, 255),
                    (0, 255, 255),
                    (255, 255, 255),
                ];

                if i < 16 {
                    let (r, g, b) = ANSI[i];
                    return Color::from_rgb(r, g, b);
                }

                if i >= 16 && i <= 231 {
                    let v = i - 16;
                    let r = v / 36;
                    let g = (v / 6) % 6;
                    let b = v % 6;
                    let levels = [0u8, 95u8, 135u8, 175u8, 215u8, 255u8];
                    return Color::from_rgb(levels[r], levels[g], levels[b]);
                }

                if i >= 232 && i <= 255 {
                    let shade = 8 + ((i - 232) * 10) as u8;
                    return Color::from_rgb(shade, shade, shade);
                }

                if is_bg { self.bg } else { self.fg }
            }
        }
    }

    fn map_vt100_color(&self, c: vt100::Color) -> Color {
        self.map_vt100_color_with_default(c, false)
    }
}

impl ElementExt for TerminalElement {
    fn diff(&self, other: &std::rc::Rc<dyn ElementExt>) -> DiffModifies {
        let Some(el) = (other.as_ref() as &dyn std::any::Any).downcast_ref::<TerminalElement>()
        else {
            return DiffModifies::all();
        };

        let mut diff = DiffModifies::empty();

        if self.font_size != el.font_size || self.font_family != el.font_family || self.id != el.id
        {
            diff.insert(DiffModifies::STYLE);
            diff.insert(DiffModifies::LAYOUT);
        }

        diff
    }

    fn layout(&'_ self) -> std::borrow::Cow<'_, LayoutData> {
        std::borrow::Cow::Borrowed(&self.layout_data)
    }

    fn should_hook_measurement(&self) -> bool {
        true
    }

    fn measure(
        &self,
        context: freya_core::element::LayoutContext,
    ) -> Option<(torin::prelude::Size2D, std::rc::Rc<dyn std::any::Any>)> {
        let mut measure_builder =
            ParagraphBuilder::new(&ParagraphStyle::default(), context.font_collection.clone());
        let mut text_style = TextStyle::new();
        text_style.set_font_size(self.font_size);
        text_style.set_font_families(&[self.font_family.as_str()]);
        measure_builder.push_style(&text_style);
        measure_builder.add_text("W");
        let mut measure_paragraph = measure_builder.build();
        measure_paragraph.layout(f32::MAX);
        let mut line_height = measure_paragraph.height();
        if line_height <= 0.0 || line_height.is_nan() {
            line_height = (self.font_size * 1.2).max(1.0);
        }

        let mut height = context.area_size.height;
        if height <= 0.0 {
            height = (line_height * 24.0).max(200.0);
        }

        let char_width = measure_paragraph.max_intrinsic_width();
        let mut target_cols = if char_width > 0.0 {
            (context.area_size.width / char_width).floor() as u16
        } else {
            1
        };
        if target_cols == 0 {
            target_cols = 1;
        }
        let mut target_rows = if line_height > 0.0 {
            (height / line_height).floor() as u16
        } else {
            1
        };
        if target_rows == 0 {
            target_rows = 1;
        }

        if let Ok(lock) = self.resize_holder.lock() {
            if let Some(tx) = lock.as_ref() {
                let _ = tx.unbounded_send((target_rows, target_cols));
            }
        }
        if let Ok(lock) = self.pty_resize_holder.lock() {
            if let Some(tx) = lock.as_ref() {
                let _ = tx.send((target_rows, target_cols));
            }
        }

        Some((
            torin::prelude::Size2D::new(context.area_size.width.max(100.0), height),
            std::rc::Rc::new(()),
        ))
    }

    fn render(&self, context: freya_core::element::RenderContext) {
        let area = context.layout_node.visible_area();

        let buffer = self.buffer.lock().unwrap();

        // Background
        let mut paint = Paint::default();
        paint.set_style(PaintStyle::Fill);
        paint.set_color(self.bg);
        context.canvas.draw_rect(
            SkRect::new(area.min_x(), area.min_y(), area.max_x(), area.max_y()),
            &paint,
        );

        // Text style
        let mut text_style = TextStyle::new();
        text_style.set_color(self.fg);
        text_style.set_font_families(&[self.font_family.as_str()]);
        text_style.set_font_size(self.font_size);

        // Measure char
        let mut measure_builder =
            ParagraphBuilder::new(&ParagraphStyle::default(), context.font_collection.clone());
        measure_builder.push_style(&text_style);
        measure_builder.add_text("W");
        let mut measure_paragraph = measure_builder.build();
        measure_paragraph.layout(f32::MAX);
        let char_width = measure_paragraph.max_intrinsic_width();
        let mut line_height = measure_paragraph.height();
        if line_height <= 0.0 || line_height.is_nan() {
            line_height = (self.font_size * 1.2).max(1.0);
        }

        // Render from top of area
        let mut y = area.min_y();

        for (row_idx, row) in buffer.rows.iter().enumerate() {
            if y + line_height > area.max_y() {
                break;
            }

            // First pass: draw background colors for each cell
            for (col_idx, cell) in row.iter().enumerate() {
                if cell.is_wide_continuation() {
                    continue;
                }
                let cell_bg = self.map_vt100_color_with_default(cell.bgcolor(), true);
                if cell_bg != self.bg {
                    let left = area.min_x() + (col_idx as f32) * char_width;
                    let top = y;
                    let cell_width = if cell.is_wide() {
                        char_width * 2.0
                    } else {
                        char_width
                    };
                    let right = left + cell_width;
                    let bottom = top + line_height;

                    let mut bg_paint = Paint::default();
                    bg_paint.set_style(PaintStyle::Fill);
                    bg_paint.set_color(cell_bg);
                    context
                        .canvas
                        .draw_rect(SkRect::new(left, top, right, bottom), &bg_paint);
                }
            }

            // Second pass: draw text
            let mut builder =
                ParagraphBuilder::new(&ParagraphStyle::default(), context.font_collection.clone());
            builder.push_style(&text_style);
            for cell in row.iter() {
                let text = if cell.has_contents() {
                    cell.contents()
                } else if cell.is_wide_continuation() {
                    continue;
                } else {
                    " "
                };
                let mut cell_style = TextStyle::new();
                cell_style.set_color(self.map_vt100_color(cell.fgcolor()));
                cell_style.set_font_families(&[self.font_family.as_str()]);
                cell_style.set_font_size(self.font_size);
                builder.push_style(&cell_style);
                builder.add_text(&text);
            }
            let mut paragraph = builder.build();
            paragraph.layout(f32::MAX);
            paragraph.paint(context.canvas, (area.min_x(), y));

            // Draw cursor
            if row_idx == buffer.cursor_row {
                let cursor_idx = buffer.cursor_col;
                let left = area.min_x() + (cursor_idx as f32) * char_width;
                let top = y;
                let right = left + char_width.max(1.0);
                let bottom = top + line_height.max(1.0);

                let mut cursor_paint = Paint::default();
                cursor_paint.set_style(PaintStyle::Fill);
                cursor_paint.set_color(self.fg);
                context
                    .canvas
                    .draw_rect(SkRect::new(left, top, right, bottom), &cursor_paint);

                let content = row
                    .get(cursor_idx)
                    .map(|cell| {
                        if cell.has_contents() {
                            cell.contents().to_string()
                        } else {
                            " ".to_string()
                        }
                    })
                    .unwrap_or_else(|| " ".to_string());

                let mut fg_text_style = TextStyle::new();
                fg_text_style.set_color(self.bg);
                fg_text_style.set_font_size(self.font_size);
                fg_text_style.set_font_families(&[self.font_family.as_str()]);
                let mut fg_builder = ParagraphBuilder::new(
                    &ParagraphStyle::default(),
                    context.font_collection.clone(),
                );
                fg_builder.push_style(&fg_text_style);
                fg_builder.add_text(&content);
                let mut fg_paragraph = fg_builder.build();
                fg_paragraph.layout((right - left).max(1.0));
                fg_paragraph.paint(context.canvas, (left, top));
            }

            y += line_height;
        }
    }
}

impl LayoutExt for TerminalElement {
    fn get_layout(&mut self) -> &mut LayoutData {
        &mut self.layout_data
    }
}
impl ContainerExt for TerminalElement {}

impl From<TerminalElement> for freya_core::element::Element {
    fn from(value: TerminalElement) -> Self {
        freya_core::element::Element::Element {
            key: freya_core::diff_key::DiffKey::None,
            element: std::rc::Rc::new(value),
            elements: Vec::new(),
        }
    }
}

/// Check for terminal queries in PTY output and return appropriate responses.
/// This handles DSR, DA, and other queries that shells like nushell send.
fn check_for_terminal_queries(data: &[u8], parser: &Arc<RwLock<Parser>>) -> Vec<Vec<u8>> {
    let mut responses = Vec::new();

    // DSR 6n - Cursor Position Report
    if data.windows(4).any(|w| w == b"\x1b[6n") {
        if let Ok(p) = parser.read() {
            let (row, col) = p.screen().cursor_position();
            let response = format!("\x1b[{};{}R", row + 1, col + 1);
            responses.push(response.into_bytes());
        }
    }

    // DSR ?6n - Extended Cursor Position Report
    if data.windows(5).any(|w| w == b"\x1b[?6n") {
        if let Ok(p) = parser.read() {
            let (row, col) = p.screen().cursor_position();
            let response = format!("\x1b[?{};{}R", row + 1, col + 1);
            responses.push(response.into_bytes());
        }
    }

    // DSR 5n - Device Status Report (terminal OK)
    if data.windows(4).any(|w| w == b"\x1b[5n") {
        responses.push(b"\x1b[0n".to_vec());
    }

    // DA1 - Primary Device Attributes
    if data.windows(3).any(|w| w == b"\x1b[c") || data.windows(4).any(|w| w == b"\x1b[0c") {
        responses.push(b"\x1b[?62;22c".to_vec());
    }

    // DA2 - Secondary Device Attributes
    if data.windows(4).any(|w| w == b"\x1b[>c") || data.windows(5).any(|w| w == b"\x1b[>0c") {
        responses.push(b"\x1b[>0;0;0c".to_vec());
    }

    responses
}

#[derive(Clone)]
struct TerminalTab {
    id: usize,
    name: String,
    buffer: Arc<Mutex<TerminalBuffer>>,
    writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
    resize_holder: Arc<Mutex<Option<UnboundedSender<(u16, u16)>>>>,
    pty_resize_holder: Arc<Mutex<Option<std::sync::mpsc::Sender<(u16, u16)>>>>,
    parser: Arc<RwLock<Option<Arc<RwLock<Parser>>>>>,
    update_tx: Arc<Mutex<Option<futures_channel::mpsc::UnboundedSender<()>>>>,
}

fn create_terminal_tab(id: usize, name: String) -> TerminalTab {
    TerminalTab {
        id,
        name,
        buffer: Arc::new(Mutex::new(TerminalBuffer::default())),
        writer: Arc::new(Mutex::new(None)),
        resize_holder: Arc::new(Mutex::new(None)),
        pty_resize_holder: Arc::new(Mutex::new(None)),
        parser: Arc::new(RwLock::new(None)),
        update_tx: Arc::new(Mutex::new(None)),
    }
}

fn initialize_terminal_tab(tab: &TerminalTab) {
    // Channel for notifying UI of updates
    let (tx, mut rx) = futures_channel::mpsc::unbounded::<()>();
    let (resize_tx, mut resize_rx) = futures_channel::mpsc::unbounded::<(u16, u16)>();
    let (pty_tx, pty_rx) = std::sync::mpsc::channel::<(u16, u16)>();

    *tab.resize_holder.lock().unwrap() = Some(resize_tx);
    *tab.pty_resize_holder.lock().unwrap() = Some(pty_tx);
    *tab.update_tx.lock().unwrap() = Some(tx.clone());

    // Create shared parser
    let parser = Arc::new(RwLock::new(Parser::new(24, 80, 1000)));
    *tab.parser.write().unwrap() = Some(parser.clone());

    let parser_for_async = parser.clone();
    let tab_buffer = tab.buffer.clone();

    // Async task to update UI buffer when notified
    spawn(async move {
        loop {
            futures_util::select! {
                _ = rx.next().fuse() => {
                    // PTY output received - update buffer from parser
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

                        // Update the buffer - this will trigger a re-render
                        if let Ok(mut buf) = tab_buffer.lock() {
                            *buf = new_buffer;
                            let platform = Platform::get();
                            platform.send(UserEvent::RequestRedraw);
                        }
                    }
                }
                resize = resize_rx.next().fuse() => {
                    if let Some((rows, cols)) = resize {
                        if let Ok(mut p) = parser_for_async.write() {
                            p.screen_mut().set_size(rows, cols);
                        }
                        // Update buffer after resize
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

    // PTY thread - reads from PTY, processes through parser, handles DSR synchronously
    let writer_holder_for_pty = tab.writer.clone();
    std::thread::spawn(move || {
        let pty_system = native_pty_system();
        match pty_system.openpty(PtySize::default()) {
            Ok(pair) => {
                if let Ok(w) = pair.master.take_writer() {
                    *writer_holder_for_pty.lock().unwrap() = Some(w);
                } else {
                    return;
                }

                let mut cmd = CommandBuilder::new("bash");
                cmd.env("TERM", "xterm-256color");
                cmd.env("COLORTERM", "truecolor");
                cmd.env("LANG", "en_US.UTF-8");

                if let Err(_e) = pair.slave.spawn_command(cmd) {
                    return;
                }

                match pair.master.try_clone_reader() {
                    Ok(mut reader) => {
                        // Resize thread
                        let master_for_resize = pair.master;
                        std::thread::spawn(move || {
                            for (rows, cols) in pty_rx {
                                let size = PtySize {
                                    rows,
                                    cols,
                                    pixel_width: 0,
                                    pixel_height: 0,
                                };
                                let _ = master_for_resize.resize(size);
                            }
                        });

                        // Main read loop - process data and handle DSR synchronously
                        let mut buf = [0u8; 4096];
                        loop {
                            match reader.read(&mut buf) {
                                Ok(0) => break,
                                Ok(n) => {
                                    let data = &buf[..n];

                                    // Process through parser FIRST
                                    if let Ok(mut p) = parser.write() {
                                        p.process(data);
                                    }

                                    // Check for terminal queries and respond IMMEDIATELY
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

                                    // Notify UI to update
                                    let _ = tx.unbounded_send(());
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
    });
}

fn terminal_tab_ui(tab: &TerminalTab, is_active: bool) -> impl IntoElement {
    let bg_color = if is_active {
        (100, 100, 150)
    } else {
        (60, 60, 60)
    };
    let text_color = if is_active {
        (255, 255, 255)
    } else {
        (200, 200, 200)
    };

    rect()
        .corner_radius(12.)
        .expanded()
        .center()
        .background(bg_color)
        .child(
            paragraph()
                .font_size(12.)
                .color(text_color)
                .span(tab.name.clone()),
        )
}

fn terminal_ui(tab: &TerminalTab) -> Element {
    Element::from(TerminalElement::new(
        tab.id,
        tab.buffer.clone(),
        tab.resize_holder.clone(),
        tab.pty_resize_holder.clone(),
    ))
}

fn app() -> impl IntoElement {
    let mut tabs =
        use_state::<Vec<TerminalTab>>(|| vec![create_terminal_tab(0, "Terminal 1".to_string())]);
    let mut active_tab = use_state(|| 0usize);
    let mut next_tab_id = use_state(|| 1usize);

    let focus = use_focus();

    // Initialize the first tab
    use_hook({
        move || {
            if let Some(tab) = tabs.read().get(0) {
                initialize_terminal_tab(tab);
            }
            focus.request_focus();
        }
    });

    let active_tab_idx = *active_tab.read();
    let tabs_len = tabs.read().len();

    rect().expanded().background((30, 30, 30)).child(
        // Main layout with sidebar and terminal area
        rect()
            .expanded()
            .direction(Direction::Horizontal)
            .child(
                // Sidebar with tabs
                rect()
                    .width(Size::Pixels(Length::new(200.0)))
                    .background((40, 40, 40))
                    .border(Border::new().width(1.0).fill((80, 80, 80)))
                    .child(
                        rect()
                            .expanded()
                            .direction(Direction::Vertical)
                            .spacing(4.)
                            .padding(4.)
                            .children(tabs.read().iter().enumerate().map(|(i, tab)| {
                                let tab_idx = i;
                                let is_active = tab_idx == active_tab_idx;
                                rect()
                                    .corner_radius(12.)
                                    .width(Size::fill())
                                    .height(Size::px(50.))
                                    .on_press(move |_| {
                                        active_tab.set(tab_idx);
                                        focus.request_focus();
                                    })
                                    .child(terminal_tab_ui(tab, is_active))
                                    .into()
                            }))
                            .child(
                                // New Tab button
                                rect()
                                    .corner_radius(12.)
                                    .width(Size::fill())
                                    .height(Size::px(50.))
                                    .background((70, 70, 70))
                                    .border(Border::new().width(1.0).fill((120, 120, 120)))
                                    .padding(8.0)
                                    .center()
                                    .on_press(move |_| {
                                        let id = *next_tab_id.read();
                                        next_tab_id.set(id + 1);
                                        let new_tab =
                                            create_terminal_tab(id, format!("Terminal {}", id + 1));
                                        let new_tab_clone = new_tab.clone();
                                        tabs.with_mut(|mut t| t.push(new_tab));
                                        active_tab.set(tabs.read().len() - 1);

                                        // Initialize the new tab
                                        initialize_terminal_tab(&new_tab_clone);
                                    })
                                    .child(
                                        paragraph()
                                            .font_size(12.)
                                            .color((200, 200, 200))
                                            .span("+ New Tab"),
                                    ),
                            )
                            .child(
                                label()
                                    .position(Position::new_absolute().bottom(10.))
                                    .text("Made with Freya! github.com/marc2332/freya")
                                    .font_size(13.)
                                    .color((255, 255, 255))
                                    .text_align(TextAlign::Center),
                            ),
                    ),
            )
            .child(
                // Main terminal area
                rect()
                    .expanded()
                    .background((10, 10, 10))
                    .a11y_id(focus.a11y_id())
                    .on_mouse_down(move |_| focus.request_focus())
                    .on_press(move |_| focus.request_focus())
                    .on_key_down({
                        let mut tabs = tabs.clone();
                        let mut active_tab = active_tab.clone();
                        let mut next_tab_id = next_tab_id.clone();
                        move |e: Event<freya_core::events::data::KeyboardEventData>| {
                            focus.request_focus();
                            let modifiers = e.modifiers;
                            let active_idx = *active_tab.read();

                            if modifiers.contains(Modifiers::CONTROL) {
                                match &e.key {
                                    Key::Character(c) if c == "t" => {
                                        // Ctrl+T: New tab
                                        let id = *next_tab_id.read();
                                        next_tab_id.set(id + 1);
                                        let new_tab =
                                            create_terminal_tab(id, format!("Terminal {}", id + 1));
                                        let new_tab_clone = new_tab.clone();
                                        tabs.with_mut(|mut t| t.push(new_tab));
                                        active_tab.set(tabs.read().len() - 1);

                                        // Initialize the new tab
                                        initialize_terminal_tab(&new_tab_clone);
                                        return;
                                    }
                                    Key::Character(c) if c == "w" && tabs_len > 1 => {
                                        // Ctrl+W: Close tab
                                        if active_idx < tabs.read().len() {
                                            tabs.with_mut(|mut t| {
                                                t.remove(active_idx);
                                            });
                                            if active_idx >= tabs.read().len() {
                                                active_tab.set(tabs.read().len().saturating_sub(1));
                                            }
                                        }
                                        return;
                                    }
                                    Key::Character(c) if c == "n" => {
                                        // Ctrl+N: Next tab
                                        if tabs_len > 1 {
                                            active_tab.set((active_idx + 1) % tabs_len);
                                        }
                                        return;
                                    }
                                    Key::Character(c) if c == "p" => {
                                        // Ctrl+P: Previous tab
                                        if tabs_len > 1 {
                                            active_tab.set(active_idx.saturating_sub(1));
                                        }
                                        return;
                                    }
                                    _ => {}
                                }
                            }

                            // Send input to active terminal
                            if let Some(tab) = tabs.read().get(active_idx) {
                                let to_write = if modifiers.contains(Modifiers::CONTROL)
                                    && matches!(&e.key, Key::Character(ch) if ch.len() == 1)
                                {
                                    if let Key::Character(ch) = &e.key {
                                        vec![ch.as_bytes()[0] & 0x1f]
                                    } else {
                                        return;
                                    }
                                } else if let Some(ch) = e.try_as_str() {
                                    ch.as_bytes().to_vec()
                                } else {
                                    match &e.key {
                                        Key::Named(NamedKey::Enter) => b"\r".to_vec(),
                                        Key::Named(NamedKey::Backspace) => vec![0x7f],
                                        Key::Named(NamedKey::Delete) => b"\x1b[3~".to_vec(),
                                        Key::Named(NamedKey::Tab) => b"\t".to_vec(),
                                        Key::Named(NamedKey::Escape) => vec![0x1b],
                                        Key::Named(NamedKey::ArrowUp) => b"\x1b[A".to_vec(),
                                        Key::Named(NamedKey::ArrowDown) => b"\x1b[B".to_vec(),
                                        Key::Named(NamedKey::ArrowLeft) => b"\x1b[D".to_vec(),
                                        Key::Named(NamedKey::ArrowRight) => b"\x1b[C".to_vec(),
                                        _ => return,
                                    }
                                };

                                match tab.writer.lock() {
                                    Ok(mut guard) => match guard.as_mut() {
                                        Some(w) => {
                                            let _ = w.write_all(&to_write);
                                            let _ = w.flush();
                                        }
                                        None => {}
                                    },
                                    Err(_) => {}
                                }
                            }
                        }
                    })
                    .child(
                        rect()
                            .padding(6.)
                            .child(match tabs.read().get(active_tab_idx) {
                                Some(active_terminal) => terminal_ui(active_terminal),
                                None => Element::from(paragraph().span("No terminal selected")),
                            }),
                    ),
            ),
    )
}

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app).with_size(900., 600.)));
}
