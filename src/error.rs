use std::num::ParseFloatError;
use std::string::FromUtf8Error;

use cindex::CIndexError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GcalcError {
    #[error("Foramtting fail : {0}")]
    FormatFail(Box<dyn std::error::Error>),
    #[error("Invalid argument : {0}")]
    InvalidArgument(String),
    #[error("Standard IO error\n= {0}")]
    StdIo(std::io::Error),
    #[error("Invalid csv error\n= {0}")] // This is logic error
    CsvError(String),
    #[error("Invalid probability form\n= {0}")]
    InvalidProb(String),
    #[error("Invalid conversion\n= {0}")]
    InvalidConversion(String),
    #[error("Invalid conditional calculation\n= {0}")]
    InvalidConditional(String),
    #[error("Invalid characters in input\n= {0}")]
    InvalidStringConversion(FromUtf8Error),
    #[error("Failed to parse a value \n= {0}")]
    ParseError(String),
    #[error("Unknown error \n= {0}")]
    Unknown(String),
    #[cfg(feature = "plotters")]
    #[error("Failed to create plot image \n= {0}")]
    PlotError(String),
    #[error("{0}")]
    CIndexError(CIndexError),
}

impl From<std::io::Error> for GcalcError {
    fn from(err: std::io::Error) -> Self {
        Self::StdIo(err)
    }
}

impl From<FromUtf8Error> for GcalcError {
    fn from(err: FromUtf8Error) -> Self {
        Self::InvalidStringConversion(err)
    }
}

impl From<ParseFloatError> for GcalcError {
    fn from(err: ParseFloatError) -> Self {
        Self::ParseError(err.to_string())
    }
}

impl From<CIndexError> for GcalcError {
    fn from(err: CIndexError) -> Self {
        Self::CIndexError(err)
    }
}
