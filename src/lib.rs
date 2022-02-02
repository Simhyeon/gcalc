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
