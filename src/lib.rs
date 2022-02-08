/// # Gcalc
/// 
/// Gcalc is a game probability calculator.
/// 
/// ## Usage
/// 
/// ```rust
///
/// use gcalc::Calculator;
///
/// let mut calc = Calculator::new().expect("Failed to crate calculator")
///     .probability(0.1).expect("Invalid probability format")
///     .cost(100.0)
///     .out_file("file_to_write_values.csv");
///
/// calc.print_range(Some(10), None).expect("Failed to calculate");
///
/// calc.set_target_probability(0.9);
/// calc.print_conditional().expect("Failed to calculate");
///
/// ```
///

mod calc;
#[cfg(feature = "binary")]
pub mod cli;
mod csv;
mod error;
mod formatter;
mod models;
mod utils;
mod consts;
#[cfg(feature = "wasm")]
mod wasm;
#[cfg(feature = "plotters")]
mod plot;

pub use calc::{Calculator, TableFormat};
pub use error::GcalcError;
pub use models::{GcalcResult, ProbType};
