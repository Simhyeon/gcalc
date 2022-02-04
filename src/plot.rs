use crate::models::{GcalcResult, Record};
use crate::{GcalcError, ProbType};
use plotters::prelude::*;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::utils;

pub(crate) struct Renderer;

impl Renderer {
    pub fn draw_chart(attr: PlotAttribute, data: &Vec<Record>, prob_type: &ProbType) -> GcalcResult<()> {
        let root_area = SVGBackend::new(Path::new("out.svg"), attr.img_size).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        if data.len() == 0 {
            return Err(GcalcError::PlotError("Plot data is empty".to_string()));
        }

        let mut max_cost = &data
            .iter()
            .max_by(|&x, &y| {
                x.probability.partial_cmp(&y.probability).unwrap()
            }).unwrap().cost;

        // TO make chart look consistent
        if *max_cost == 0.0 {
            max_cost = &1.0;
        }

        let (ft,fs) = (attr.font_type, attr.font_size);
        let area_size = fs as f32 * 2.5;
        let column_count = data.len();

        let mut ctx = ChartBuilder::on(&root_area)
            .margin(10u32)
            .x_label_area_size(50.0)
            .y_label_area_size(area_size)
            .right_y_label_area_size(area_size)
            .caption(&attr.caption, (ft.as_str(),fs as f64))
            .build_cartesian_2d(1..column_count, 0f64..1.0 ).map_err(|_| GcalcError::PlotError(format!("Failed to create chart")))?
            .set_secondary_coord(1..column_count, 0f32..*max_cost);

        // Mesh configuration
        ctx.configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .bold_line_style(&WHITE.mix(0.3))
            .y_desc(&attr.prob_caption)
            .label_style((ft.as_str(),fs as f64 * 0.5))
            .axis_desc_style((ft.as_str(),fs as f64))
            .draw().map_err(|_| GcalcError::PlotError(format!("Failed to configure mesh for chart")))?;

        ctx
            .configure_secondary_axes()
            .y_desc(&attr.cost_caption)
            .label_style((ft.as_str(),fs as f64 * 0.5))
            .axis_desc_style((ft.as_str(),fs as f64 ))
            .draw().map_err(|_| GcalcError::PlotError(format!("Failed to configure secondary mesh for chart")))?;

        // Porb series
        ctx.draw_series(LineSeries::new(
                (1..).zip(data.iter()).map(|(x,y)| { 
                    (x,utils::extract_prob_from_string(&y.probability, prob_type).unwrap() as f64)
                }),
                Into::<ShapeStyle>::into(&RED).stroke_width(2).filled(),
        )).map_err(|_| GcalcError::PlotError(format!("Failed to embed data into a chart")))?;

        // Bar seires
        ctx.draw_secondary_series(
            LineSeries::new(
                (1usize..).zip(data.iter()).map(|(x,y)| { 
                    (x,y.cost)
                }),
                Into::<ShapeStyle>::into(&BLUE).stroke_width(2),
            )
        ).map_err(|_| GcalcError::PlotError(format!("Failed to embed data into a chart")))?;

        // Backup

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
    pub img_size: (u32,u32),
}

impl Default for PlotAttribute {
    fn default() -> Self {
        Self {
            caption: "Gcalc result".to_owned(),
            prob_caption: "Prob".to_owned(),
            cost_caption: "Cost".to_owned(),
            font_type: "Helvetica".to_owned(),
            font_size: 50,
            img_size: (1000,1000),
        }
    }
} 

#[derive(Serialize, Deserialize,Clone,Copy, Debug)]
pub enum PlotType {
    Bar, // Bar horizontal
    Line, // Line series
}
