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

pub use calc::{Calculator, TableFormat};
pub use error::GcalcError;
pub use models::{GcalcResult, ProbType};

#[cfg(feature = "wasm")]
pub use wasm::{calculate, default_option};
