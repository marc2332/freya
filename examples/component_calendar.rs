#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut selected = use_state(|| None::<CalendarDate>);
    let mut view_date = use_state(CalendarDate::now);
    let month = CalendarMonth::new(view_date(), selected(), WeekStart::Monday);

    rect()
        .expanded()
        .center()
        .map(month, |el, month| {
            let title = format!("{:02}/{}", month.first_day.month, month.first_day.year);
            let previous_month = month.previous_month;
            let next_month = month.next_month;

            el.child(
                rect()
                    .width(Size::px(252.))
                    .child(
                        rect()
                            .horizontal()
                            .content(Content::flex())
                            .child(
                                Button::new()
                                    .flat()
                                    .on_press(move |_| view_date.set(previous_month))
                                    .child("‹"),
                            )
                            .child(
                                label()
                                    .width(Size::flex(1.))
                                    .text_align(TextAlign::Center)
                                    .text(title),
                            )
                            .child(
                                Button::new()
                                    .flat()
                                    .on_press(move |_| view_date.set(next_month))
                                    .child("›"),
                            ),
                    )
                    .child(
                        rect()
                            .horizontal()
                            .content(Content::wrap())
                            .width(Size::px(252.))
                            .children(
                                ["Lun", "Mar", "Mié", "Jue", "Vie", "Sáb", "Dom"]
                                    .into_iter()
                                    .map(|name| {
                                        rect()
                                            .width(Size::px(36.))
                                            .height(Size::px(36.))
                                            .center()
                                            .child(name)
                                    }),
                            ),
                    )
                    .child(CalendarGrid::new(month, move |day| {
                        CalendarCell::new(day)
                            .on_change(move |date| selected.set(Some(date)))
                            .into()
                    })),
            )
        })
        .child(match selected() {
            Some(date) => format!("Selected: {}/{}/{}", date.day, date.month, date.year),
            None => "No date selected".to_string(),
        })
}
