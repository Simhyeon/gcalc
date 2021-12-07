use std::path::PathBuf;

use serde::Serialize;
use tabled::Tabled;

use crate::GcalcError;
pub type GcalcResult<T> = Result<T, GcalcError>;

pub struct ColumnMap {
    pub count: usize,
    pub probability: usize,
    pub added: usize,
    pub cost: usize,
}

impl ColumnMap {
    pub fn new( count: usize, probability: usize, added: usize, cost: usize) -> Self {
        Self { count, probability, added, cost, }
    }
}

#[derive(Serialize, Tabled)]
pub(crate) struct Qualficiation {
    pub count: usize,
    pub cost : f32,
}

impl Qualficiation {
    pub fn new(count: usize, cost: f32) -> Self {
        Self {
            count,
            cost,
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
pub enum RefCsv {
    Raw(String),
    File(PathBuf),
    None,
}
