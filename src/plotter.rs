use plotters::prelude::*;

const OUT_FILE_NAME: &str = "chart.png";
pub fn quick_plot<'a>(x_axis: &str, y_axis: &str, data: Vec<(u32, u32)>) -> Result<(), Box<dyn std::error::Error>>{
    let mut min_x = std::u32::MAX;
    let mut max_x = std::u32::MIN;
    let mut max_y = std::u32::MIN;

    data.iter().for_each(|item| {
        if item.0 < min_x {
            min_x = item.0;
        }
        if item.0 > max_x {
            max_x = item.0;
        }
        if item.1 > max_y {
            max_y = item.1;
        }
    });

    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .margin(15)
        .build_cartesian_2d((min_x..max_x).into_segmented(), 0u32..(max_y+1))?;


    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc(y_axis)
        .x_desc(x_axis)
        .axis_desc_style(("sans-serif", 15))
        .x_labels((max_x-min_x) as usize)
        .draw()?;


    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .margin(0)
            .data(data.into_iter())
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present()?;

    Ok(())
}