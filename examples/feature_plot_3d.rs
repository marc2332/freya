#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use freya::prelude::{
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
            Color,
            WHITE,
        },
    },
    *,
};

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn on_render(ctx: &mut RenderContext, (cursor_x, cursor_y): (f64, f64)) {
    let backend = PlotSkiaBackend::new(
        ctx.canvas,
        ctx.font_collection,
        ctx.layout_node.area.size.to_i32().to_tuple(),
    )
    .into_drawing_area();

    backend.fill(&WHITE).unwrap();

    let pitch = std::f64::consts::PI * (0.5 - cursor_y / ctx.layout_node.area.height() as f64);
    let yaw = std::f64::consts::PI * 2.0 * (cursor_x / ctx.layout_node.area.width() as f64 - 0.5);
    let scale = 0.4 + 0.6 * (1.0 - cursor_y / ctx.layout_node.area.height() as f64);

    let x_axis = (-3.0..3.0).step(0.1);
    let z_axis = (-3.0..3.0).step(0.1);

    let mut chart = ChartBuilder::on(&backend)
        .caption("3D Plot Test", ("sans", 20))
        .build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone())
        .unwrap();

    chart.with_projection(|mut pb| {
        pb.pitch = pitch;
        pb.yaw = yaw;
        pb.scale = scale;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()
        .unwrap();

    chart
        .draw_series(
            SurfaceSeries::xoz(
                (-30..30).map(|f| f as f64 / 10.0),
                (-30..30).map(|f| f as f64 / 10.0),
                |x, z| (x * x + z * z).cos(),
            )
            .style(BLUE.mix(0.2).filled()),
        )
        .unwrap()
        .label("Surface")
        .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));

    chart
        .draw_series(LineSeries::new(
            (-100..100)
                .map(|y| y as f64 / 40.0)
                .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
            &BLACK,
        ))
        .unwrap()
        .label("Line")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .draw()
        .unwrap();
}

fn app() -> impl IntoElement {
    let mut cursor_position = use_state(CursorPoint::default);

    let on_global_mouse_move = move |e: Event<MouseEventData>| {
        // Dont move when the cursor goes outside the window
        if e.global_location.to_tuple() != (-1., -1.) {
            cursor_position.set(e.global_location);
            let platform = Platform::get();
            platform.send(UserEvent::RequestRedraw);
        }
    };

    plot(RenderCallback::new(move |context| {
        on_render(context, cursor_position().to_tuple());
    }))
    .expanded()
    .on_global_mouse_move(on_global_mouse_move)
}
