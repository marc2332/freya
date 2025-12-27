/// Determines which day the week starts on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekStart {
    Sunday,
    Monday,
}

use chrono::{
    Datelike,
    Local,
    Month,
    NaiveDate,
};
use freya_core::prelude::*;
use torin::{
    content::Content,
    prelude::Alignment,
    size::Size,
};

use crate::{
    button::Button,
    get_theme,
    icons::arrow::ArrowIcon,
    theming::component_themes::{
        ButtonColorsThemePartialExt,
        ButtonLayoutThemePartialExt,
        CalendarTheme,
        CalendarThemePartial,
    },
};

/// A simple date representation for the calendar.
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
        let today = Local::now().date_naive();
        Self {
            year: today.year(),
            month: today.month(),
            day: today.day(),
        }
    }

    /// Returns the number of days in the given month.
    fn days_in_month(year: i32, month: u32) -> u32 {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        NaiveDate::from_ymd_opt(next_year, next_month, 1)
            .and_then(|d| d.pred_opt())
            .map(|d| d.day())
            .unwrap_or(30)
    }

    /// Returns the day of the week for the first day of the month.
    fn first_day_of_month(year: i32, month: u32, week_start: WeekStart) -> u32 {
        NaiveDate::from_ymd_opt(year, month, 1)
            .map(|d| match week_start {
                WeekStart::Sunday => d.weekday().num_days_from_sunday(),
                WeekStart::Monday => d.weekday().num_days_from_monday(),
            })
            .unwrap_or(0)
    }

    /// Returns the full name of the month.
    fn month_name(month: u32) -> String {
        Month::try_from(month as u8)
            .map(|m| m.name().to_string())
            .unwrap_or_else(|_| "Unknown".to_string())
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum CalendarDayStatus {
    #[default]
    Idle,
    Hovering,
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

impl Render for Calendar {
    fn render(&self) -> impl IntoElement {
        let theme = get_theme!(&self.theme, calendar);

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
        } = theme;

        let view_year = self.view_date.year;
        let view_month = self.view_date.month;

        let days_in_month = CalendarDate::days_in_month(view_year, view_month);
        let first_day = CalendarDate::first_day_of_month(view_year, view_month, self.week_start);
        let month_name = CalendarDate::month_name(view_month);

        // Previous month info for leading days
        let prev_month = if view_month == 1 { 12 } else { view_month - 1 };
        let prev_year = if view_month == 1 {
            view_year - 1
        } else {
            view_year
        };
        let days_in_prev_month = CalendarDate::days_in_month(prev_year, prev_month);

        let on_change = self.on_change.clone();
        let on_view_change = self.on_view_change.clone();
        let selected = self.selected;

        // Navigation handlers
        let on_prev = {
            let on_view_change = on_view_change.clone();
            move |_: Event<PressEventData>| {
                if let Some(handler) = &on_view_change {
                    let new_month = if view_month == 1 { 12 } else { view_month - 1 };
                    let new_year = if view_month == 1 {
                        view_year - 1
                    } else {
                        view_year
                    };
                    handler.call(CalendarDate::new(new_year, new_month, 1));
                }
            }
        };

        let on_next = {
            let on_view_change = on_view_change.clone();
            move |_: Event<PressEventData>| {
                if let Some(handler) = &on_view_change {
                    let new_month = if view_month == 12 { 1 } else { view_month + 1 };
                    let new_year = if view_month == 12 {
                        view_year + 1
                    } else {
                        view_year
                    };
                    handler.call(CalendarDate::new(new_year, new_month, 1));
                }
            }
        };

        let mut rows: Vec<Element> = Vec::new();

        let weekday_names = match self.week_start {
            WeekStart::Sunday => ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"],
            WeekStart::Monday => ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
        };
        let header_cells = weekday_names.iter().map(|day_name| {
            rect()
                .width(Size::px(36.))
                .height(Size::px(36.))
                .center()
                .child(label().text(*day_name).color(header_color).font_size(12.))
                .into()
        });
        rows.push(rect().horizontal().children_iter(header_cells).into());

        let mut current_day: i32 = 1 - first_day as i32;
        let total_days = first_day + days_in_month;
        let total_weeks = total_days.div_ceil(7);

        for _ in 0..total_weeks {
            let mut week_cells: Vec<Element> = Vec::new();

            for _ in 0..7 {
                if current_day < 1 {
                    // Previous month
                    let day = (days_in_prev_month as i32 + current_day) as u32;
                    week_cells.push(
                        CalendarDay::new()
                            .key(day)
                            .day(day)
                            .color(day_other_month_color)
                            .corner_radius(day_corner_radius)
                            .enabled(false)
                            .into(),
                    );
                } else if current_day as u32 > days_in_month {
                    // Next month
                    let day = current_day as u32 - days_in_month;
                    week_cells.push(
                        CalendarDay::new()
                            .key(day)
                            .day(day)
                            .color(day_other_month_color)
                            .corner_radius(day_corner_radius)
                            .enabled(false)
                            .into(),
                    );
                } else {
                    // Current month
                    let day = current_day as u32;
                    let date = CalendarDate::new(view_year, view_month, day);
                    let is_selected = selected == Some(date);

                    let on_change = on_change.clone();

                    let (background, hover_background) = if is_selected {
                        (day_selected_background, day_selected_background)
                    } else {
                        (day_background, day_hover_background)
                    };

                    week_cells.push(
                        CalendarDay::new()
                            .key(day)
                            .day(day)
                            .background(background)
                            .hover_background(hover_background)
                            .color(color)
                            .corner_radius(day_corner_radius)
                            .map(on_change, |el, on_change| {
                                el.on_press(move |_| on_change.call(date))
                            })
                            .into(),
                    );
                }

                current_day += 1;
            }

            rows.push(rect().horizontal().children(week_cells).into());
        }

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
                    .child(
                        NavButton::new()
                            .hover_background(nav_button_hover_background)
                            .on_press(on_prev)
                            .child(
                                ArrowIcon::new()
                                    .fill(color)
                                    .width(Size::px(16.))
                                    .height(Size::px(16.))
                                    .rotate(90.),
                            ),
                    )
                    .child(
                        label()
                            .width(Size::flex(1.))
                            .text_align(TextAlign::Center)
                            .text(format!("{} {}", month_name, view_year))
                            .color(header_color)
                            .max_lines(1)
                            .font_size(16.),
                    )
                    .child(
                        NavButton::new()
                            .hover_background(nav_button_hover_background)
                            .on_press(on_next)
                            .child(
                                ArrowIcon::new()
                                    .fill(color)
                                    .width(Size::px(16.))
                                    .height(Size::px(16.))
                                    .rotate(-90.),
                            ),
                    ),
            )
            .children(rows)
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}

#[derive(Clone, PartialEq)]
struct NavButton {
    hover_background: Color,
    children: Vec<Element>,
    on_press: Option<EventHandler<Event<PressEventData>>>,
}

impl NavButton {
    fn new() -> Self {
        Self {
            hover_background: Color::TRANSPARENT,
            children: Vec::new(),
            on_press: None,
        }
    }

    fn hover_background(mut self, hover_background: Color) -> Self {
        self.hover_background = hover_background;
        self
    }

    fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }
}

impl ChildrenExt for NavButton {
    fn get_children(&mut self) -> &mut Vec<Element> {
        &mut self.children
    }
}

impl Render for NavButton {
    fn render(&self) -> impl IntoElement {
        Button::new()
            .flat()
            .width(Size::px(32.))
            .height(Size::px(32.))
            .hover_background(self.hover_background)
            .map(self.on_press.clone(), |el, on_press| el.on_press(on_press))
            .children(self.children.clone())
    }
}

#[derive(Clone, PartialEq)]
struct CalendarDay {
    day: u32,
    background: Color,
    hover_background: Color,
    color: Color,
    corner_radius: CornerRadius,
    on_press: Option<EventHandler<Event<PressEventData>>>,
    enabled: bool,
    key: DiffKey,
}

impl CalendarDay {
    fn new() -> Self {
        Self {
            day: 1,
            background: Color::TRANSPARENT,
            hover_background: Color::TRANSPARENT,
            color: Color::BLACK,
            corner_radius: CornerRadius::default(),
            on_press: None,
            enabled: true,
            key: DiffKey::None,
        }
    }

    fn day(mut self, day: u32) -> Self {
        self.day = day;
        self
    }

    fn background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    fn hover_background(mut self, hover_background: Color) -> Self {
        self.hover_background = hover_background;
        self
    }

    fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    fn corner_radius(mut self, corner_radius: CornerRadius) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    fn on_press(mut self, on_press: impl Into<EventHandler<Event<PressEventData>>>) -> Self {
        self.on_press = Some(on_press.into());
        self
    }

    fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl KeyExt for CalendarDay {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Render for CalendarDay {
    fn render(&self) -> impl IntoElement {
        Button::new()
            .flat()
            .padding(0.)
            .enabled(self.enabled)
            .width(Size::px(36.))
            .height(Size::px(36.))
            .background(self.background)
            .hover_background(self.hover_background)
            .maybe(self.enabled, |el| {
                el.map(self.on_press.clone(), |el, on_press| el.on_press(on_press))
            })
            .child(
                label()
                    .text(self.day.to_string())
                    .color(self.color)
                    .font_size(14.),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
