use chrono::{
    Datelike,
    Duration,
    Local,
    Months,
    NaiveDate,
};
use freya_core::prelude::*;
use torin::{
    content::Content,
    gaps::Gaps,
    prelude::Alignment,
    size::Size,
};

use crate::{
    button::{
        Button,
        ButtonColorsThemePartialExt,
        ButtonLayoutThemePartialExt,
    },
    define_theme,
    get_theme,
    icons::arrow::ArrowIcon,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekStart {
    Sunday,
    Monday,
}

define_theme! {
    %[component]
    pub Calendar {
        %[fields]
        background: Color,
        day_background: Color,
        day_hover_background: Color,
        day_selected_background: Color,
        color: Color,
        day_other_month_color: Color,
        header_color: Color,
        corner_radius: CornerRadius,
        padding: Gaps,
        day_corner_radius: CornerRadius,
        nav_button_hover_background: Color,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl CalendarDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    /// Returns the current local date.
    pub fn now() -> Self {
        Local::now().date_naive().into()
    }
}

impl From<NaiveDate> for CalendarDate {
    fn from(date: NaiveDate) -> Self {
        Self::new(date.year(), date.month(), date.day())
    }
}

/// A calendar component for date selection.
///
/// # Example
///
/// ```rust
/// # use freya::prelude::*;
/// fn app() -> impl IntoElement {
///     let mut selected = use_state(|| None::<CalendarDate>);
///     let mut view_date = use_state(|| CalendarDate::new(2025, 1, 1));
///
///     Calendar::new()
///         .selected(selected())
///         .view_date(view_date())
///         .on_change(move |date| selected.set(Some(date)))
///         .on_view_change(move |date| view_date.set(date))
/// }
/// # use freya_testing::prelude::*;
/// # launch_doc(|| {
/// #   rect().center().expanded().child(app())
/// # }, "./images/gallery_calendar.png").with_hook(|_| {}).with_scale_factor(0.8).render();
/// ```
///
/// # Preview
///
/// ![Calendar Preview][gallery_calendar]
#[cfg_attr(feature = "docs", doc = embed_doc_image::embed_image!("gallery_calendar", "images/gallery_calendar.png"))]
#[derive(Clone, PartialEq)]
pub struct Calendar {
    pub(crate) theme: Option<CalendarThemePartial>,
    selected: Option<CalendarDate>,
    view_date: CalendarDate,
    week_start: WeekStart,
    on_change: Option<EventHandler<CalendarDate>>,
    on_view_change: Option<EventHandler<CalendarDate>>,
    key: DiffKey,
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            theme: None,
            selected: None,
            view_date: CalendarDate::now(),
            week_start: WeekStart::Monday,
            on_change: None,
            on_view_change: None,
            key: DiffKey::None,
        }
    }

    pub fn selected(mut self, selected: Option<CalendarDate>) -> Self {
        self.selected = selected;
        self
    }

    pub fn view_date(mut self, view_date: CalendarDate) -> Self {
        self.view_date = view_date;
        self
    }

    /// Set which day the week starts on (Sunday or Monday)
    pub fn week_start(mut self, week_start: WeekStart) -> Self {
        self.week_start = week_start;
        self
    }

    pub fn on_change(mut self, on_change: impl Into<EventHandler<CalendarDate>>) -> Self {
        self.on_change = Some(on_change.into());
        self
    }

    pub fn on_view_change(mut self, on_view_change: impl Into<EventHandler<CalendarDate>>) -> Self {
        self.on_view_change = Some(on_view_change.into());
        self
    }
}

impl KeyExt for Calendar {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for Calendar {
    fn render(&self) -> impl IntoElement {
        let CalendarTheme {
            background,
            day_background,
            day_hover_background,
            day_selected_background,
            color,
            day_other_month_color,
            header_color,
            corner_radius,
            padding,
            day_corner_radius,
            nav_button_hover_background,
        } = get_theme!(&self.theme, CalendarThemePreference, "calendar");

        let first_day = NaiveDate::from_ymd_opt(self.view_date.year, self.view_date.month, 1)
            .unwrap_or_default();
        let prev_month = first_day
            .checked_sub_months(Months::new(1))
            .unwrap_or(first_day);
        let next_month = first_day
            .checked_add_months(Months::new(1))
            .unwrap_or(first_day);
        let days_in_month = next_month.pred_opt().map(|d| d.day()).unwrap_or(30);

        let (leading, weekday_names) = match self.week_start {
            WeekStart::Sunday => (
                first_day.weekday().num_days_from_sunday(),
                ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"],
            ),
            WeekStart::Monday => (
                first_day.weekday().num_days_from_monday(),
                ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
            ),
        };
        let total_cells = (leading + days_in_month).div_ceil(7) * 7;

        let nav_button = |target: NaiveDate, rotate: f32| {
            let on_view_change = self.on_view_change.clone();
            Button::new()
                .flat()
                .width(Size::px(32.))
                .height(Size::px(32.))
                .hover_background(nav_button_hover_background)
                .on_press(move |_: Event<PressEventData>| {
                    if let Some(handler) = &on_view_change {
                        handler.call(target.into());
                    }
                })
                .child(
                    ArrowIcon::new()
                        .fill(color)
                        .width(Size::px(16.))
                        .height(Size::px(16.))
                        .rotate(rotate),
                )
        };

        let header_cells = weekday_names.iter().map(|name| {
            rect()
                .width(Size::px(36.))
                .height(Size::px(36.))
                .center()
                .child(label().text(*name).color(header_color).font_size(12.))
                .into()
        });

        let day_cells = (0..total_cells).map(|i| {
            let date = first_day
                .checked_add_signed(Duration::days(i as i64 - leading as i64))
                .unwrap_or(first_day);
            let in_month = date.month() == first_day.month();
            let is_selected = in_month && self.selected == Some(date.into());
            let on_change = self.on_change.clone();

            let (day_color, bg, hover_bg) = if is_selected {
                (color, day_selected_background, day_selected_background)
            } else if in_month {
                (color, day_background, day_hover_background)
            } else {
                (
                    day_other_month_color,
                    Color::TRANSPARENT,
                    Color::TRANSPARENT,
                )
            };

            Button::new()
                .key(date)
                .flat()
                .padding(0.)
                .enabled(in_month)
                .width(Size::px(36.))
                .height(Size::px(36.))
                .background(bg)
                .hover_background(hover_bg)
                .corner_radius(day_corner_radius)
                .maybe(in_month, |el| {
                    el.map(on_change, |el, on_change| {
                        el.on_press(move |_| on_change.call(date.into()))
                    })
                })
                .child(
                    label()
                        .text(date.day().to_string())
                        .color(day_color)
                        .font_size(14.),
                )
                .into()
        });

        rect()
            .background(background)
            .corner_radius(corner_radius)
            .padding(padding)
            .width(Size::px(280.))
            .child(
                rect()
                    .horizontal()
                    .width(Size::fill())
                    .padding((0., 0., 8., 0.))
                    .cross_align(Alignment::center())
                    .content(Content::flex())
                    .child(nav_button(prev_month, 90.))
                    .child(
                        label()
                            .width(Size::flex(1.))
                            .text_align(TextAlign::Center)
                            .text(first_day.format("%B %Y").to_string())
                            .color(header_color)
                            .max_lines(1)
                            .font_size(16.),
                    )
                    .child(nav_button(next_month, -90.)),
            )
            .child(
                rect()
                    .horizontal()
                    .content(Content::wrap())
                    .width(Size::fill())
                    .children(header_cells)
                    .children(day_cells),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
