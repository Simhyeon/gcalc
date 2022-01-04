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

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wasm")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "wasm")]
pub use wasm::{calculate, default_option};
