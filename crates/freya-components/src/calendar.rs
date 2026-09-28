use chrono::{
    Datelike,
    Duration,
    Local,
    Months,
    NaiveDate,
    Weekday,
};
use freya_core::prelude::*;
use torin::{
    content::Content,
    size::Size,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekStart {
    Sunday,
    Monday,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// A date in the grid, including dates from adjacent months.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarDay {
    pub date: CalendarDate,
    pub in_month: bool,
    pub selected: bool,
}

/// The dates and weekdays needed to render a month.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarMonth {
    pub first_day: CalendarDate,
    pub previous_month: CalendarDate,
    pub next_month: CalendarDate,
    pub weekdays: [Weekday; 7],
    pub days: Vec<CalendarDay>,
}

impl CalendarMonth {
    pub fn new(
        view_date: CalendarDate,
        selected: Option<CalendarDate>,
        week_start: WeekStart,
    ) -> Option<Self> {
        let first_day = NaiveDate::from_ymd_opt(view_date.year, view_date.month, 1)?;
        let previous_month = first_day
            .checked_sub_months(Months::new(1))
            .unwrap_or(first_day);
        let next_month = first_day
            .checked_add_months(Months::new(1))
            .unwrap_or(first_day);

        let days_in_month = (1..=31).rev().find(|day| {
            NaiveDate::from_ymd_opt(first_day.year(), first_day.month(), *day).is_some()
        })?;
        let leading = match week_start {
            WeekStart::Sunday => first_day.weekday().num_days_from_sunday(),
            WeekStart::Monday => first_day.weekday().num_days_from_monday(),
        };
        let mut weekdays = [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ];
        if week_start == WeekStart::Sunday {
            weekdays.rotate_right(1);
        }

        let total_cells = (leading + days_in_month).div_ceil(7) * 7;
        let days = (0..total_cells)
            .filter_map(|index| {
                let date =
                    first_day.checked_add_signed(Duration::days(index as i64 - leading as i64))?;
                let in_month = date.month() == first_day.month();
                let date = CalendarDate::from(date);

                Some(CalendarDay {
                    date,
                    in_month,
                    selected: in_month && selected == Some(date),
                })
            })
            .collect();

        Some(Self {
            first_day: first_day.into(),
            previous_month: previous_month.into(),
            next_month: next_month.into(),
            weekdays,
            days,
        })
    }
}

/// A month grid whose cells are rendered by the caller.
#[derive(Clone, PartialEq)]
pub struct CalendarGrid {
    month: CalendarMonth,
    render_cell: Callback<CalendarDay, Element>,
    width: Size,
    key: DiffKey,
}

impl CalendarGrid {
    pub fn new(
        month: CalendarMonth,
        render_cell: impl FnMut(CalendarDay) -> Element + 'static,
    ) -> Self {
        Self {
            month,
            render_cell: render_cell.into(),
            width: Size::px(252.),
            key: DiffKey::None,
        }
    }

    /// Sets the grid width. Defaults to 252 pixels.
    pub fn width(mut self, width: impl Into<Size>) -> Self {
        self.width = width.into();
        self
    }
}

impl KeyExt for CalendarGrid {
    fn write_key(&mut self) -> &mut DiffKey {
        &mut self.key
    }
}

impl Component for CalendarGrid {
    fn render(&self) -> impl IntoElement {
        rect()
            .horizontal()
            .content(Content::wrap())
            .width(self.width.clone())
            .children(
                self.month
                    .days
                    .iter()
                    .map(|day| rect().key(day.date).child(self.render_cell.call(*day))),
            )
    }

    fn render_key(&self) -> DiffKey {
        self.key.clone().or(self.default_key())
    }
}
