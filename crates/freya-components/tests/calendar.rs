#![cfg(feature = "calendar")]

use freya::prelude::*;
use freya_testing::prelude::*;

#[test]
fn grid_renders_seven_days_per_row() {
    fn app() -> impl IntoElement {
        let selected = CalendarDate::new(2024, 2, 29);
        let month = CalendarMonth::new(selected, Some(selected), WeekStart::Monday).unwrap();

        CalendarGrid::new(month, |day| {
            rect()
                .width(Size::px(35.))
                .height(Size::px(36.))
                .child(if day.selected {
                    format!("selected-{}-{}", day.date.month, day.date.day)
                } else {
                    format!("{}-{}", day.date.month, day.date.day)
                })
                .into()
        })
    }

    let test = launch_test(app);

    let position = |text: &str| {
        test.find(|node, element| {
            Label::try_downcast(element)
                .filter(|label| label.text.as_ref() == text)
                .map(|_| node)
        })
        .unwrap()
        .layout()
        .area
        .origin
    };

    let first = position("1-29");
    let seventh = position("2-4");
    let eighth = position("2-5");

    assert_eq!(first.y, seventh.y);
    assert_eq!(first.x, eighth.x);
    assert!(eighth.y > first.y);
    assert!(
        test.find(|_, element| {
            Label::try_downcast(element).filter(|label| label.text.as_ref() == "selected-2-29")
        })
        .is_some()
    );
}
