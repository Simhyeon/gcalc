use thiserror::Error;

#[derive(Error, Debug)]
pub enum GcalcError {
    #[error("Foramtting fail : {0}")]
    FormatFail(Box<dyn std::error::Error>),
    #[error("Invalid argument : {0}")]
    InvalidArgument(String),
    #[error("Standard IO error\n= {0}")]
    StdIo(std::io::Error),
    #[error("Invalid csv error\n= {0}")]
    CsvError(String),
    #[error("Invalid probabilty form\n= {0}")]
    InvalidProb(String),
    #[error("Invalid conversion\n= {0}")]
    InvalidConversion(String),
    #[error("Invalid conditional calculation\n= {0}")]
    InvalidConditional(String),
}

impl From<std::io::Error> for GcalcError {
    fn from(err : std::io::Error) -> Self {
        Self::StdIo(err)
    }
}
