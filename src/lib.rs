mod calc;
#[cfg(feature = "binary")]
pub mod cli;
mod csv;
mod error;
mod formatter;
mod models;
mod utils;
mod consts;

pub use calc::{Calculator, ProbType, TableFormat};
pub use error::GcalcError;
pub use models::GcalcResult;
