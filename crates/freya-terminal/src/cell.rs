//! Crate-local snapshot of a terminal cell.
//!
//! rio-vt stores cells as packed `Square`s whose colors, attributes,
//! hyperlinks and zero-width characters live in per-grid side tables.
//! The renderer and URL scanner work on plain row buffers, so each visible
//! row is resolved into `TermCell`s once per frame.

use rio_vt::{
    config::colors::{
        AnsiColor,
        ColorRgb,
        NamedColor,
    },
    crosswords::{
        Crosswords,
        grid::ExtrasTable,
        pos::{
            Column,
            Line,
        },
        square::{
            ContentTag,
            Square,
            Wide,
        },
        style::{
            Style,
            StyleFlags,
        },
    },
    event::EventListener,
};

/// A resolved terminal cell with the character, colors and attributes
/// the renderer consumes.
#[derive(Clone)]
pub(crate) struct TermCell {
    pub character: char,
    pub foreground: AnsiColor,
    pub background: AnsiColor,
    pub inverse: bool,
    pub wide: bool,
    pub wide_spacer: bool,
    pub zerowidth: Vec<char>,
    /// URI of the OSC 8 hyperlink attached to this cell, if any.
    pub hyperlink: Option<String>,
}

impl TermCell {
    pub(crate) fn from_square(square: &Square, styles: &[Style], extras: &ExtrasTable) -> Self {
        let (foreground, background, flags) = match square.content_tag() {
            ContentTag::Codepoint => {
                let style = styles
                    .get(square.style_id() as usize)
                    .copied()
                    .unwrap_or_default();
                (style.fg, style.bg, style.flags)
            }
            ContentTag::BgPalette => (
                AnsiColor::Named(NamedColor::Foreground),
                AnsiColor::Indexed(square.bg_palette_index()),
                StyleFlags::empty(),
            ),
            ContentTag::BgRgb => {
                let (red, green, blue) = square.bg_rgb();
                (
                    AnsiColor::Named(NamedColor::Foreground),
                    AnsiColor::Spec(ColorRgb {
                        r: red,
                        g: green,
                        b: blue,
                    }),
                    StyleFlags::empty(),
                )
            }
        };

        let (zerowidth, hyperlink) = square
            .extras_id()
            .and_then(|id| extras.get(id))
            .map(|extra| {
                (
                    extra.zerowidth.clone(),
                    extra.hyperlink.as_ref().map(|link| link.uri().to_owned()),
                )
            })
            .unwrap_or_default();

        TermCell {
            character: square.c(),
            foreground,
            background,
            inverse: flags.contains(StyleFlags::INVERSE),
            wide: matches!(square.wide(), Wide::Wide),
            wide_spacer: matches!(square.wide(), Wide::Spacer | Wide::LeadingSpacer),
            zerowidth,
            hyperlink,
        }
    }
}

/// Resolve the viewport row at `viewport_row` (0 = top of the visible area)
/// into `TermCell`s, indexing the grid directly so only that row is touched.
pub(crate) fn snapshot_row<T: EventListener>(
    term: &Crosswords<T>,
    viewport_row: usize,
) -> Vec<TermCell> {
    let line = Line(viewport_row as i32 - term.display_offset() as i32);
    let row = &term.grid[line];
    let styles = term.grid.style_set.styles();
    let extras = &term.grid.extras_table;
    (0..term.columns())
        .map(|column| TermCell::from_square(&row[Column(column)], styles, extras))
        .collect()
}
