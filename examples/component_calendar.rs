use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut selected = use_state(|| None::<CalendarDate>);
    let mut view_date = use_state(CalendarDate::now);

    rect()
        .expanded()
        .center()
        .child(
            rect().height(Size::px(325.)).child(
                Calendar::new()
                    .selected(selected())
                    .view_date(view_date())
                    .on_change(move |date| selected.set(Some(date)))
                    .on_view_change(move |date| view_date.set(date)),
            ),
        )
        .child(match selected() {
            Some(date) => format!("Selected: {}/{}/{}", date.month, date.day, date.year),
            None => "No date selected".to_string(),
        })
}
