use std::path::PathBuf;

use serde::Serialize;
use tabled::Tabled;

use crate::GcalcError;
pub type GcalcResult<T> = Result<T, GcalcError>;

pub struct ColumnMap {
    pub count: usize,
    pub probability: usize,
    pub constant: usize,
    pub cost: usize,
}

impl ColumnMap {
    pub fn new( count: usize, probability: usize, constant: usize, cost: usize) -> Self {
        Self { count, probability, constant, cost, }
    }
}

#[derive(Serialize, Tabled)]
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

#[derive(Serialize, Tabled)]
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

#[derive(PartialEq)]
pub enum CsvRef {
    Raw(String),
    File(PathBuf),
    None,
}

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

pub enum OutOption {
    Console,
    File(PathBuf),
}

pub enum RecordCursor {
    Next,
    Stay,
}
