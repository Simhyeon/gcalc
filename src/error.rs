use std::num::ParseFloatError;
use std::string::FromUtf8Error;

use cindex::CIndexError;

#[derive(Debug)]
pub enum GcalcError {
    FormatFail(Box<dyn std::error::Error>),
    InvalidArgument(String),
    StdIo(std::io::Error),
    CsvError(String),
    InvalidProb(String),
    InvalidConversion(String),
    InvalidConditional(String),
    InvalidStringConversion(FromUtf8Error),
    ParseError(String),
    Unknown(String),
    #[cfg(feature = "plotters")]
    PlotError(String),
    CIndexError(CIndexError),
}

impl std::fmt::Display for GcalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FormatFail(err) => write!(f, "Foramtting fail : {}", err),
            Self::InvalidArgument(err) => write!(f, "Invalid argument : {}", err),
            Self::StdIo(err) => write!(f, "Standard IO error\n= {}", err),
            Self::CsvError(err) => write!(f, "Invalid csv error\n= {}", err), // This is logic error
            Self::InvalidProb(err) => write!(f, "Invalid probability form\n= {}", err),
            Self::InvalidConversion(err) => write!(f, "Invalid conversion\n= {}", err),
            Self::InvalidConditional(err) => {
                write!(f, "Invalid conditional calculation\n= {}", err)
            }
            Self::InvalidStringConversion(err) => {
                write!(f, "Invalid characters in input\n= {}", err)
            }
            Self::ParseError(err) => write!(f, "Failed to parse a value \n= {}", err),
            Self::Unknown(err) => write!(f, "Unknown error \n= {}", err),
            #[cfg(feature = "plotters")]
            Self::PlotError(err) => write!(f, "Failed to create plot image \n= {}", err),
            Self::CIndexError(err) => write!(f, "{}", err),
        }
    }
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
