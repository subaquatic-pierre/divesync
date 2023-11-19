use core::utils::timestamp;

use plotters::prelude::*;

pub struct CliPlotter;

impl CliPlotter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn plot(&self) -> Result<(), Box<dyn std::error::Error>> {
        let filename = format!("plots/{}.png", timestamp());
        let root = BitMapBackend::new(&filename, (640, 480)).into_drawing_area();

        root.fill(&WHITE)?;
        let root = root.margin(10, 10, 10, 10);
        // After this point, we should be able to construct a chart context
        let mut chart = ChartBuilder::on(&root)
            // Set the caption of the chart
            .caption("Change the text here", ("sans-serif", 40).into_font())
            // Set the size of the label region
            .x_label_area_size(20)
            .y_label_area_size(40)
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_cartesian_2d(0f32..10f32, 0f32..10f32)?;

        // Then we can draw a mesh
        chart
            .configure_mesh()
            // We can customize the maximum number of labels allowed for each axis
            .x_labels(5)
            .y_labels(5)
            // We can also change the format of the label text
            .y_label_formatter(&|x| format!("{:.3}", x))
            .draw()?;

        // And we can draw something in the drawing area
        chart.draw_series(LineSeries::new(
            vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
            &RED,
        ))?;
        // Similarly, we can draw point series
        chart.draw_series(PointSeries::of_element(
            vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)],
            5,
            &RED,
            &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
            },
        ))?;

        root.present()?;
        Ok(())
    }
}
