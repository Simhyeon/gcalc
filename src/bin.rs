use gcalc::{Calculator, ProbType};
use gcalc::GcalcResult;

fn main() -> GcalcResult<()> {
    let ca = Calculator::new(0.3,10)?
        .prob_type(ProbType::Percentage)
        .precision(2)?;
    // ca.print_range(Some((0,10)))?;
    ca.print_until(0.9f32)?;
    // ca.print_required(0.9f32, Some( 1000f32 ))?;
    Ok(())
}
