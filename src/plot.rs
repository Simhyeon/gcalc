use crate::models::{GcalcResult, Record};
use crate::{GcalcError, ProbType};
use plotters::prelude::*;
use std::path::{PathBuf, Path};
use serde::{Deserialize, Serialize};
use crate::utils;

pub(crate) struct Renderer;

impl Renderer {
    pub fn draw_chart(attr: PlotAttribute, data: &Vec<Record>, prob_type: &ProbType) -> GcalcResult<()> {
        let root_area = BitMapBackend::new(Path::new("out.png"), attr.img_size).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        if data.len() == 0 {
            return Err(GcalcError::PlotError("Plot data is empty".to_string()));
        }

        let max_cost = &data
            .iter()
            .max_by(|&x, &y| {
                x.probability.partial_cmp(&y.probability).unwrap()
            }).unwrap().cost;

        let column_count = data.len();

        let mut ctx = ChartBuilder::on(&root_area)
            .x_label_area_size(50.0f32)
            .y_label_area_size(50.0f32)
            .right_y_label_area_size(50f32)
            .caption(&attr.caption, ("helvetica", 20.0f64))
            .build_cartesian_2d(0..column_count, 0f64..1.0 ).map_err(|_| GcalcError::PlotError(format!("Failed to create chart")))?
            .set_secondary_coord(0..column_count, 0f32..*max_cost + 100f32);

        // Mesh configuration
        ctx.configure_mesh()
            // Remove x lines
            .disable_x_mesh()
            // Remove y lines
            .disable_y_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc(attr.y_label)
            .x_desc(attr.x_label)
            .draw().map_err(|_| GcalcError::PlotError(format!("Failed to configure mesh for chart")))?;

        ctx
            .configure_secondary_axes()
            .y_desc("Cost")
            .draw().map_err(|_| GcalcError::PlotError(format!("Failed to configure secondary mesh for chart")))?;

        // TODO, Ok this works at least
        ctx.draw_series(LineSeries::new(
                (0..).zip(data.iter()).map(|(x,y)| { 
                    (x,utils::extract_prob_from_string(&y.probability, prob_type).unwrap() as f64)
                }),
                Into::<ShapeStyle>::into(&RED).stroke_width(2).filled(),
        )).map_err(|_| GcalcError::PlotError(format!("Failed to embed data into a chart")))?;

        ctx.draw_secondary_series(LineSeries::new(
                (0usize..).zip(data.iter()).map(|(x,y)| { 
                    (x,y.cost)
                }),
                Into::<ShapeStyle>::into(&BLUE).stroke_width(2).filled(),
        )).map_err(|_| GcalcError::PlotError(format!("Failed to embed data into a chart")))?;

        Ok(())
    }

    fn draw_bar_v(tup : (u32, &u32)) -> Rectangle<(plotters::prelude::SegmentValue<u32>, u32)> {
        let (x,y) = tup;
        let x0 = SegmentValue::Exact(x);
        let x1 = SegmentValue::Exact(x + 1);
        let mut bar = Rectangle::new([(x0, 0), (x1, *y)], RED.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }

    fn draw_bar_h(tup: (u32, &u32)) -> Rectangle<(u32, plotters::prelude::SegmentValue<u32>)> {
        let (y,x) = tup;
        let mut bar = Rectangle::new([
            (0, SegmentValue::Exact(y)), 
            (*x, SegmentValue::Exact(y + 1))
        ], RED.filled());
        bar.set_margin(5, 5, 0, 0);
        bar
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlotAttribute {
    pub plot_type: PlotType,
    pub cost_type: PlotType,
    pub caption: String,
    pub x_label: String,
    pub y_label: String,
    pub label_style: (String,i32),
    pub img_size: (u32,u32),
}

#[derive(Serialize, Deserialize,Clone,Copy, Debug)]
pub enum PlotType {
    Bar, // Bar horizontal
    Line, // Line series
}
