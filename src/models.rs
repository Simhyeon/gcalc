use std::path::PathBuf;
use std::str::FromStr;

use crate::GcalcError;
#[cfg(feature = "option")]
use serde::Deserialize;
#[cfg(feature = "option")]
use serde::Serialize;
#[cfg(feature = "tabled")]
use tabled::Tabled;

pub type GcalcResult<T> = Result<T, GcalcError>;

#[cfg_attr(feature = "tabled", derive(Tabled))]
pub(crate) struct Qualficiation {
    pub count: usize,
    pub probability: String,
    pub cost: f32,
}

impl Qualficiation {
    pub fn new(count: usize, cost: f32, probability: &str) -> Self {
        Self {
            count,
            cost,
            probability: probability.to_owned(),
        }
    }

    pub fn join_as_csv(&self) -> String {
        let mut joined = self.count.to_string();
        joined.push_str(&format!(",{}", self.probability));
        joined.push_str(&format!(",{}", self.cost));
        joined
    }
}

#[cfg_attr(feature = "tabled", derive(Tabled))]
pub(crate) struct FormatRecord {
    pub count: usize,
    pub probability: String,
    pub cost: f32,
    pub constant: f32,
}

impl FormatRecord {
    pub fn from_record(record: &Record) -> Self {
        Self {
            count: record.count,
            probability: record.probability.to_string(),
            cost: record.cost,
            constant: record.constant,
        }
    }
}

pub(crate) struct Record {
    pub count: usize,
    pub probability_src: f32,
    pub probability: String,
    pub cost: f32,
    pub constant: f32,
}

impl Record {
    pub fn new(
        count: usize,
        probability_src: f32,
        probability: String,
        cost: f32,
        constant: f32,
    ) -> Self {
        Self {
            count,
            probability_src,
            probability,
            cost,
            constant,
        }
    }

    pub fn join_as_csv(&self) -> String {
        let mut joined = self.count.to_string();
        joined.push_str(&format!(",{}", self.probability));
        joined.push_str(&format!(",{}", self.cost));
        joined.push_str(&format!(",{}", self.constant));
        joined
    }
}

#[cfg_attr(feature = "option", derive(Serialize, Deserialize, Clone))]
#[derive(PartialEq)]
pub enum CsvRef {
    Raw(String),
    File(PathBuf),
    None,
}

#[cfg_attr(feature = "option", derive(Serialize, Deserialize, Clone, Copy))]
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
            _ => {
                return Err(GcalcError::InvalidConversion(format!(
                    "{} is not a valid csv fallback behaviour variant",
                    text
                )))
            }
        };
        Ok(varirant)
    }
}

#[cfg_attr(feature = "option", derive(Serialize, Deserialize, Clone))]
pub enum OutOption {
    Console,
    File(PathBuf),
}

pub enum RecordCursor {
    Next,
    Stay,
}

#[cfg_attr(feature = "option", derive(Serialize, Deserialize, Clone, Copy))]
pub enum ProbType {
    Percentage,
    Fraction,
}

impl FromStr for ProbType {
    type Err = GcalcError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "percentage" | "percent" => Ok(Self::Percentage),
            "float" => Ok(Self::Fraction),
            _ => Err(GcalcError::InvalidConversion(format!(
                "{} is not a valid table format",
                s
            ))),
        }
    }
}
