use crate::models::{GcalcResult, Record};
use crate::GcalcError;
use plotters::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub(crate) struct Renderer;

impl Renderer {
    pub fn draw_chart(attr: PlotAttribute, data: &Vec<Record>) -> GcalcResult<()> {
        let root_area = SVGBackend::new(Path::new("out.svg"), attr.img_size).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        if data.is_empty() {
            return Err(GcalcError::PlotError("Plot data is empty".to_string()));
        }

        let mut max_cost = &data
            .iter()
            .max_by(|&x, &y| x.probability.partial_cmp(&y.probability).unwrap())
            .unwrap()
            .cost;

        // TO make chart look consistent
        if *max_cost == 0.0 {
            max_cost = &1.0;
        }

        let (ft, fs) = (attr.font_type, attr.font_size);
        let area_size = fs as f32 * 2.5;
        let column_count = data.len();

        let mut ctx = ChartBuilder::on(&root_area)
            .margin(10u32)
            .x_label_area_size(50.0f32)
            .y_label_area_size(area_size)
            .right_y_label_area_size(area_size)
            .caption(&attr.caption, (ft.as_str(), fs as f64))
            .build_cartesian_2d(1..column_count, 0f64..1.0)
            .map_err(|_| GcalcError::PlotError("Failed to create chart".to_string()))?
            .set_secondary_coord(1..column_count, 0f32..*max_cost);

        // Mesh configuration
        ctx.configure_mesh()
            .x_labels(20)
            .y_labels(20)
            .disable_x_mesh()
            .disable_y_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc(&attr.prob_caption)
            .label_style((ft.as_str(), fs as f64 * 0.5))
            .axis_desc_style((ft.as_str(), fs as f64))
            .draw()
            .map_err(|_| GcalcError::PlotError("Failed to configure mesh for chart".to_string()))?;

        ctx.configure_secondary_axes()
            .y_desc(&attr.cost_caption)
            .label_style((ft.as_str(), fs as f64 * 0.5))
            .axis_desc_style((ft.as_str(), fs as f64))
            .draw()
            .map_err(|_| {
                GcalcError::PlotError("Failed to configure secondary mesh for chart".to_string())
            })?;

        // Prob series
        ctx.draw_series(LineSeries::new(
            (1..)
                .zip(data.iter())
                .map(|(x, y)| (x, y.probability_src as f64)),
            Into::<ShapeStyle>::into(&RED).stroke_width(2).filled(),
        ))
        .map_err(|_| GcalcError::PlotError("Failed to embed data into a chart".to_string()))?;

        // Point
        ctx.draw_series(
            (1..)
                .zip(data.iter())
                .map(|(x, y)| Circle::new((x, y.probability_src as f64), 3, RED.filled())),
        )
        .unwrap();

        // Bar seires
        ctx.draw_secondary_series(LineSeries::new(
            (1usize..).zip(data.iter()).map(|(x, y)| (x, y.cost)),
            Into::<ShapeStyle>::into(&BLUE.mix(0.3)).stroke_width(2),
        ))
        .map_err(|_| GcalcError::PlotError("Failed to embed data into a chart".to_string()))?;

        // Point
        ctx.draw_secondary_series(
            (1..)
                .zip(data.iter())
                .map(|(x, y)| Circle::new((x, y.cost), 3, BLUE.mix(0.3).filled())),
        )
        .unwrap();

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlotAttribute {
    pub caption: String,
    pub prob_caption: String,
    pub cost_caption: String,
    pub font_type: String,
    pub font_size: u8,
    pub img_size: (u32, u32),
}

impl Default for PlotAttribute {
    fn default() -> Self {
        Self {
            caption: "Gcalc result".to_owned(),
            prob_caption: "Prob (r)".to_owned(),
            cost_caption: "Cost (b)".to_owned(),
            font_type: "Helvetica".to_owned(),
            font_size: 50,
            img_size: (1000, 1000),
        }
    }
}
