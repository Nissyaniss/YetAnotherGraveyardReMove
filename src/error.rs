use std::io;

use skim::options::SkimOptionsBuilderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YagError {
    #[error("IOError: {message} : {error}")]
    IOError { error: io::Error, message: String },

    #[error("Failed to dialog : {0}")]
    DialogError(#[from] dialoguer::Error),

    #[error("Skim Error: Some field are uninitialized : {0}")]
    SkimUninitializedField(String),

    #[error("Skim Error: Some field are invalid : {0}")]
    SkimValidationError(String),

    #[error("Skim Error: unimplemented error: {0}")]
    SkimUnimplementedError(#[from] SkimOptionsBuilderError),

    #[error("SkimError: {0}")]
    SkimEyreReport(#[from] eyre::Report),

    #[error("Serialize Error: {message} : {error}")]
    SerializeError {
        error: toml::ser::Error,
        message: String,
    },

    #[error("Deserialize Error: {message} : {error}")]
    DeserializeError {
        error: toml::de::Error,
        message: String,
    },
}

pub trait YagErrorUtilsMessage<T> {
    fn to_yag_error_message(self, message: impl ToString) -> Result<T, YagError>;
}

pub trait YagErrorUtils<T> {
    fn to_yag_error(self) -> Result<T, YagError>;
}

impl<T> YagErrorUtilsMessage<T> for Result<T, io::Error> {
    fn to_yag_error_message(self, message: impl ToString) -> Result<T, YagError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(YagError::IOError {
                error: e,
                message: message.to_string(),
            }),
        }
    }
}

impl<T> YagErrorUtils<T> for Result<T, dialoguer::Error> {
    fn to_yag_error(self) -> Result<T, YagError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(YagError::DialogError(e)),
        }
    }
}

impl<T> YagErrorUtils<T> for Result<T, skim::options::SkimOptionsBuilderError> {
    fn to_yag_error(self) -> Result<T, YagError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => match e {
                SkimOptionsBuilderError::UninitializedField(msg) => {
                    Err(YagError::SkimUninitializedField(msg.to_string()))
                }
                SkimOptionsBuilderError::ValidationError(msg) => {
                    Err(YagError::SkimValidationError(msg))
                }
                _ => Err(YagError::SkimUnimplementedError(e)),
            },
        }
    }
}

impl<T> YagErrorUtilsMessage<T> for Result<T, toml::de::Error> {
    fn to_yag_error_message(self, message: impl ToString) -> Result<T, YagError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(YagError::DeserializeError {
                error: e,
                message: message.to_string(),
            }),
        }
    }
}

impl<T> YagErrorUtilsMessage<T> for Result<T, toml::ser::Error> {
    fn to_yag_error_message(self, message: impl ToString) -> Result<T, YagError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(YagError::SerializeError {
                error: e,
                message: message.to_string(),
            }),
        }
    }
}

impl<T> YagErrorUtils<T> for Result<T, eyre::Report> {
    fn to_yag_error(self) -> Result<T, YagError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(YagError::SkimEyreReport(e)),
        }
    }
}
