use crate::models::{GcalcResult, Record};
use crate::GcalcError;
use plotters::prelude::*;
use std::path::{PathBuf, Path};
use serde::{Deserialize, Serialize};

pub struct Renderer;

impl Renderer {
    pub fn draw_chart(&self, attr: PlotAttribute, data: &Vec<Record>) -> GcalcResult<()> {
        let root_area = BitMapBackend::new(Path::new("out.png"), attr.img_size).into_drawing_area();
        root_area.fill(&WHITE).unwrap();

        if data.len() == 0 {
            return Err(GcalcError::PlotError("Plot data is empty".to_string()));
        }

        let max = data
            .iter()
            .max_by(|&x, &y| x..partial_cmp(&y).unwrap())
            .unwrap_or(&0f64);

        let row_line_end = max.ceil() as f64;
        let column_count = attr.data.len() - 1;

        let mut ctx = ChartBuilder::on(&root_area)
            .x_label_area_size(attr.x_label_size)
            .y_label_area_size(attr.label_style)
            .caption(&attr.caption, ("helvetica", 20))
            .build_cartesian_2d(0..column_count, 0f64..row_line_end ).map_err(|_| GcalcError::PlotError(format!("Failed to create chart")))?;

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

        // TODO, Ok this works at least
        ctx.draw_series(LineSeries::new(
                (0..).zip(attr.data.iter()).map(|(x,y)| { (x,*y) }),
                &RED,
        )).map_err(|_| GcalcError::PlotError(format!("Failed to embed data into a chart")))?;

        Ok(())
    }

    //fn bar_chart_vertical(&self, out_file: PathBuf, plot_model: PlotModel) -> GcalcResult<()>{
        //let root_area = BitMapBackend::new(&out_file, plot_model.img_size).into_drawing_area();
        //root_area.fill(&WHITE).unwrap();

        //let max = plot_model.data
            //.iter()
            //.max_by(|&x, &y| x.partial_cmp(&y).unwrap())
            //.unwrap();

        //let row_count = plot_model.row_offset as u32 + max.ceil() as u32;
        //let column_count = plot_model.column_offset as u32 + plot_model.data.len() as u32 - 1;

        //let mut chart = ChartBuilder::on(&root_area)
            //.x_label_area_size(plot_model.x_label_size)
            //.y_label_area_size(plot_model.y_label_size)
            //.margin(plot_model.margin)
            //.caption(&plot_model.caption, (caption_font.as_str(), caption_size))
            //// Into_segmented makes number match column's center position
            //.build_cartesian_2d(
                //(0..column_count).into_segmented(), 
                //0u32..row_count
            //).map_err(|_| GcalcError::PlotError(format!("Failed to create chart")))?; 

        //// Mesh configuration
        //chart.configure_mesh()
            //// Remove x lines
            //.disable_x_mesh()
            //// Remove y lines
            //.disable_y_mesh()
            //.bold_line_style(&WHITE.mix(0.3))
            //.y_desc(plot_model.y_desc)
            //.x_desc(plot_model.x_desc)
            //.axis_desc_style((desc_font.as_str(), desc_size))
            //.draw().map_err(|_| GcalcError::PlotError(format!("Failed to configure mesh for chart")))?;

        //let data = plot_model.data.iter().map(|f| *f as u32).collect::<Vec<u32>>();
        //chart.draw_series((0..).zip(data.iter()).map(Self::draw_bar_v)).unwrap();

        //// To avoid the IO failure being ignored silently, we manually call the present function
        //root_area.present().expect("Unable to write result to file");

        //Ok(())
    //}

    //fn bar_chart_horizontal(&self, out_file: PathBuf, plot_model: PlotModel) -> GcalcResult<()>{
        //let root_area = BitMapBackend::new(&out_file, plot_model.img_size).into_drawing_area();
        //root_area.fill(&WHITE).unwrap();

        //let (caption_font, caption_size) = plot_model.caption_style;
        //let (desc_font, desc_size) = plot_model.desc_style;

        //let max = plot_model.data
            //.iter()
            //.max_by(|&x, &y| x.partial_cmp(&y).unwrap())
            //.unwrap();

        //let row_count = plot_model.row_offset as u32 + max.ceil() as u32;
        //let column_count = plot_model.column_offset as u32 + plot_model.data.len() as u32 - 1;

        //let mut chart = ChartBuilder::on(&root_area)
            //.x_label_area_size(plot_model.x_label_size)
            //.y_label_area_size(plot_model.y_label_size)
            //.margin(plot_model.margin)
            //.caption(&plot_model.caption, (caption_font.as_str(), caption_size))
            //// Into_segmented makes number match column's center position
            //.build_cartesian_2d(
                //0u32..row_count,
                //(0..column_count).into_segmented()
            //).map_err(|_| GcalcError::PlotError(format!("Failed to create chart")))?; 

        //// Mesh configuration
        //chart.configure_mesh()
            //// Remove x lines
            //.disable_x_mesh()
            //// Remove y lines
            //.disable_y_mesh()
            //.bold_line_style(&WHITE.mix(0.3))
            //.y_desc(plot_model.y_desc)
            //.x_desc(plot_model.x_desc)
            //.axis_desc_style((desc_font.as_str(), desc_size))
            //.draw().map_err(|_| GcalcError::PlotError(format!("Failed to configure mesh for chart")))?;

        //let data = plot_model.data.iter().map(|f| *f as u32).collect::<Vec<u32>>();

        //chart.draw_series((0..).zip(data.iter()).map(Self::draw_bar_h)).unwrap();

        //// To avoid the IO failure being ignored silently, we manually call the present function
        //root_area.present().expect("Unable to write result to file");

        //Ok(())
    //}

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
    prob_type: PlotType,
    cost_type: PlotType,
    caption: String,
    x_label: String,
    y_label: String,
    label_style: (String,i32),
    img_size: (u32,u32),
}

#[derive(Serialize, Deserialize,Clone,Copy, Debug)]
pub enum PlotType {
    Bar, // Bar horizontal
    Line, // Line series
}
