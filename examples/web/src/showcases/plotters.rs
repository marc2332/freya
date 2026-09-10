use freya::{
    plot::{
        PlotSkiaBackend,
        plotters::{
            chart::ChartBuilder,
            prelude::{
                IntoDrawingArea,
                IntoLinspace,
                PathElement,
                Rectangle,
            },
            series::{
                LineSeries,
                SurfaceSeries,
            },
            style::{
                BLACK,
                BLUE,
                Color as PlotColor,
                WHITE,
            },
        },
    },
    prelude::*,
};

use crate::showcases::heading;

#[derive(PartialEq)]
pub struct PlottersShowcase;

impl Component for PlottersShowcase {
    fn render(&self) -> impl IntoElement {
        let mut cursor_position = use_state(CursorPoint::default);

        let on_global_pointer_move = move |event: Event<PointerEventData>| {
            if event.global_location().to_tuple() != (-1., -1.) {
                cursor_position.set(event.global_location());
                Platform::get().send(UserEvent::RequestRedraw);
            }
        };

        rect()
            .padding(24.)
            .spacing(20.)
            .expanded()
            .child(heading("Plotters", "Move the cursor to rotate the surface"))
            .child(
                rect()
                    .expanded()
                    .corner_radius(12.)
                    .overflow(Overflow::Clip)
                    .child(
                        canvas(RenderCallback::new(move |context| {
                            draw_plot(context, cursor_position().to_tuple());
                        }))
                        .expanded()
                        .on_global_pointer_move(on_global_pointer_move),
                    ),
            )
    }
}

fn draw_plot(context: &mut CanvasContext, (cursor_x, cursor_y): (f64, f64)) {
    let width = context.size.width as f64;
    let height = context.size.height as f64;

    let backend = PlotSkiaBackend::new(
        context.canvas,
        context.font_collection,
        (context.size.width as i32, context.size.height as i32),
    )
    .into_drawing_area();

    if backend.fill(&WHITE).is_err() {
        return;
    }

    let pitch = std::f64::consts::PI * (0.5 - cursor_y / height);
    let yaw = std::f64::consts::PI * 2.0 * (cursor_x / width - 0.5);
    let scale = 0.4 + 0.6 * (1.0 - cursor_y / height);

    let Ok(mut chart) = ChartBuilder::on(&backend)
        .caption("A 3D plot", ("sans", 20))
        .build_cartesian_3d((-3.0..3.0).step(0.1), -3.0..3.0, (-3.0..3.0).step(0.1))
    else {
        return;
    };

    chart.with_projection(|mut projection| {
        projection.pitch = pitch;
        projection.yaw = yaw;
        projection.scale = scale;
        projection.into_matrix()
    });

    let _ = chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw();

    if let Ok(series) = chart.draw_series(
        SurfaceSeries::xoz(
            (-30..30).map(|value| value as f64 / 10.0),
            (-30..30).map(|value| value as f64 / 10.0),
            |x, z| (x * x + z * z).cos(),
        )
        .style(BLUE.mix(0.2).filled()),
    ) {
        series.label("Surface").legend(|(x, y)| {
            Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
        });
    }

    if let Ok(series) = chart.draw_series(LineSeries::new(
        (-100..100)
            .map(|y| y as f64 / 40.0)
            .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
        &BLACK,
    )) {
        series
            .label("Line")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));
    }

    let _ = chart.configure_series_labels().border_style(BLACK).draw();
}
