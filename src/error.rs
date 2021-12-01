use thiserror::Error;

#[derive(Error, Debug)]
pub enum GcalcError {
    #[error("Foramtting fail : {0}")]
    FormatFail(Box<dyn std::error::Error>),
    #[error("Invalid argument : {0}")]
    InvalidArgument(String)
}
