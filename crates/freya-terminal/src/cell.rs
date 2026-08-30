use rio_vt::{
    config::colors::{
        AnsiColor,
        ColorRgb,
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

/// A resolved terminal cell with the character, colors and attributes the renderer consumes.
pub(crate) struct TermCell {
    pub character: char,
    pub foreground: AnsiColor,
    pub background: AnsiColor,
    pub inverse: bool,
    pub wide: bool,
    pub wide_spacer: bool,
    pub zerowidth: Vec<char>,
    pub has_hyperlink: bool,
}

impl TermCell {
    pub(crate) fn from_square(square: &Square, styles: &[Style], extras: &ExtrasTable) -> Self {
        let (style, extra) = match square.content_tag() {
            ContentTag::Codepoint => (
                styles
                    .get(square.style_id() as usize)
                    .copied()
                    .unwrap_or_default(),
                square.extras_id().and_then(|id| extras.get(id)),
            ),
            ContentTag::BgPalette => (
                Style {
                    bg: AnsiColor::Indexed(square.bg_palette_index()),
                    ..Style::default()
                },
                None,
            ),
            ContentTag::BgRgb => {
                let (red, green, blue) = square.bg_rgb();
                (
                    Style {
                        bg: AnsiColor::Spec(ColorRgb {
                            r: red,
                            g: green,
                            b: blue,
                        }),
                        ..Style::default()
                    },
                    None,
                )
            }
        };

        TermCell {
            character: square.c(),
            foreground: style.fg,
            background: style.bg,
            inverse: style.flags.contains(StyleFlags::INVERSE),
            wide: matches!(square.wide(), Wide::Wide),
            wide_spacer: matches!(square.wide(), Wide::Spacer | Wide::LeadingSpacer),
            zerowidth: extra
                .map(|extra| extra.zerowidth.clone())
                .unwrap_or_default(),
            has_hyperlink: extra.is_some_and(|extra| extra.hyperlink.is_some()),
        }
    }
}

/// Resolve the viewport row at `viewport_row` (0 is the top of the visible area) into `cells`.
pub(crate) fn snapshot_row<T: EventListener>(
    term: &Crosswords<T>,
    viewport_row: usize,
    cells: &mut Vec<TermCell>,
) {
    let row = &term.grid[Line(viewport_row as i32 - term.display_offset() as i32)];
    let styles = term.grid.styles();
    let extras = &term.grid.extras_table;
    cells.clear();
    cells.extend(
        (0..term.columns())
            .map(|column| TermCell::from_square(&row[Column(column)], styles, extras)),
    );
}
