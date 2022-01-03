use std::path::PathBuf;

use serde::Serialize;
#[cfg(feature = "option")]
use serde::Deserialize;
#[cfg(feature = "binary")]
use tabled::Tabled;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use crate::GcalcError;

pub type GcalcResult<T> = Result<T, GcalcError>;

#[cfg_attr(feature = "wasm",wasm_bindgen)]
#[cfg_attr(feature= "option" ,derive(Serialize, Deserialize,Clone,Copy))]
pub struct ColumnMap {
    pub count: usize,
    pub probability: usize,
    pub constant: usize,
    pub cost: usize,
}

impl Default for ColumnMap {
    fn default() -> Self {
        Self {
            count: 0,
            probability: 1,
            constant: 2,
            cost: 3,
        }
    }
}

impl ColumnMap {
    pub fn new( count: usize, probability: usize, constant: usize, cost: usize) -> Self {
        Self { count, probability, constant, cost, }
    }
}

#[derive(Serialize)]
#[cfg_attr(feature= "binary" ,derive(Tabled))]
pub(crate) struct Qualficiation {
    pub count: usize,
    pub probability: String,
    pub cost : f32,
}

impl Qualficiation {
    pub fn new(count: usize, cost: f32, probability: &str) -> Self {
        Self {
            count,
            cost,
            probability: probability.to_owned(),
        }
    }
}

#[derive(Serialize)]
#[cfg_attr(feature= "binary" ,derive(Tabled))]
pub(crate) struct Record {
    pub count: usize,
    pub probability : String,
    pub cost : f32,
}

impl Record {
    pub fn new(count: usize, probability: String, cost: f32) -> Self {
        Self {
            count,
            probability,
            cost,
        }
    }
}

#[cfg_attr(feature= "option" ,derive(Serialize, Deserialize, Clone))]
#[derive(PartialEq)]
pub enum CsvRef {
    Raw(String),
    File(PathBuf),
    None,
}

#[cfg_attr(feature = "wasm",wasm_bindgen)]
#[cfg_attr(feature= "option" ,derive(Serialize, Deserialize,Clone,Copy))]
#[derive(PartialEq)]
pub enum CSVInvalidBehaviour {
    Rollback,
    Ignore,
    None,
}

impl CSVInvalidBehaviour {
    pub fn from_str(text: &str) -> GcalcResult<Self> {
        let varirant = match text.to_lowercase().as_str() {
            "rollback" => Self::Rollback,
            "ignore" => Self::Ignore,
            "none" => Self::None,
            _ => return Err(GcalcError::InvalidConversion(format!("{} is not a valid csv fallback behaviour variant", text))),
        };
        Ok(varirant)
    }
}

#[cfg_attr(feature= "option" ,derive(Serialize, Deserialize,Clone))]
pub enum OutOption {
    Console,
    File(PathBuf),
}

pub enum RecordCursor {
    Next,
    Stay,
}

#[cfg_attr(feature = "wasm",wasm_bindgen)]
#[cfg_attr(feature= "option" ,derive(Serialize, Deserialize,Clone,Copy))]
pub enum ProbType {
    Percentage,
    Fraction,
}

impl ProbType {
    pub fn from_str(string : &str) -> GcalcResult<Self> {
        match string.to_lowercase().as_str() {
            "percentage" | "percent" => Ok(Self::Percentage),
            "float" => Ok(Self::Fraction),
            _ => Err(GcalcError::InvalidConversion(format!("{} is not a valid table format", string))),
        }
    }
}

