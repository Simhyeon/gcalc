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
}

impl From<std::io::Error> for GcalcError {
    fn from(err : std::io::Error) -> Self {
        Self::StdIo(err)
    }
}
