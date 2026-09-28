#![cfg(feature = "calendar")]

use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn month_model_handles_week_starts_and_leap_years() {
    let selected = CalendarDate::new(2024, 2, 29);
    let monday = CalendarMonth::new(selected, Some(selected), WeekStart::Monday).unwrap();
    assert_eq!(monday.first_day, CalendarDate::new(2024, 2, 1));
    assert_eq!(monday.previous_month, CalendarDate::new(2024, 1, 1));
    assert_eq!(monday.next_month, CalendarDate::new(2024, 3, 1));
    assert_eq!(monday.days[0].date, CalendarDate::new(2024, 1, 29));
    assert_eq!(monday.days.len(), 35);
    assert!(
        monday
            .days
            .iter()
            .any(|day| day.date == selected && day.selected)
    );

    let sunday = CalendarMonth::new(selected, None, WeekStart::Sunday).unwrap();
    assert_eq!(sunday.days[0].date, CalendarDate::new(2024, 1, 28));
    assert_eq!(sunday.weekdays[0], chrono::Weekday::Sun);
    assert!(CalendarMonth::new(CalendarDate::new(2024, 13, 1), None, WeekStart::Monday).is_none());
}

#[test]
fn composed_grid_uses_custom_header_and_buttons() {
    fn app() -> impl IntoElement {
        let mut view_date = use_state(|| CalendarDate::new(2024, 3, 1));
        let month = CalendarMonth::new(view_date(), None, WeekStart::Monday).unwrap();
        let previous_month = month.previous_month;

        rect()
            .child(
                Button::new()
                    .on_press(move |_| view_date.set(previous_month))
                    .child("Previous"),
            )
            .child(format!("Month {}", month.first_day.month))
            .child("Lun")
            .child(CalendarGrid::new(month, |day| {
                CalendarCell::new(day).into()
            }))
    }

    let mut test = launch_test(app);
    assert!(
        test.find(|_, element| {
            Label::try_downcast(element).filter(|label| label.text.as_ref() == "Month 3")
        })
        .is_some()
    );
    assert!(
        test.find(|_, element| {
            Label::try_downcast(element).filter(|label| label.text.as_ref() == "Lun")
        })
        .is_some()
    );

    test.click_cursor((16., 16.));
    assert!(
        test.find(|_, element| {
            Label::try_downcast(element).filter(|label| label.text.as_ref() == "Month 2")
        })
        .is_some()
    );
}
