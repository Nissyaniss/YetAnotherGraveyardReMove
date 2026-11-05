use std::io;

use skim::options::SkimOptionsBuilderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrmError {
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

pub trait BrmErrorUtilsMessage<T> {
    fn to_brm_error_message(self, message: impl ToString) -> Result<T, BrmError>;
}

pub trait BrmErrorUtils<T> {
    fn to_brm_error(self) -> Result<T, BrmError>;
}

impl<T> BrmErrorUtilsMessage<T> for Result<T, io::Error> {
    fn to_brm_error_message(self, message: impl ToString) -> Result<T, BrmError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(BrmError::IOError {
                error: e,
                message: message.to_string(),
            }),
        }
    }
}

impl<T> BrmErrorUtils<T> for Result<T, dialoguer::Error> {
    fn to_brm_error(self) -> Result<T, BrmError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(BrmError::DialogError(e)),
        }
    }
}

impl<T> BrmErrorUtils<T> for Result<T, skim::options::SkimOptionsBuilderError> {
    fn to_brm_error(self) -> Result<T, BrmError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => match e {
                SkimOptionsBuilderError::UninitializedField(msg) => {
                    Err(BrmError::SkimUninitializedField(msg.to_string()))
                }
                SkimOptionsBuilderError::ValidationError(msg) => {
                    Err(BrmError::SkimValidationError(msg))
                }
                _ => Err(BrmError::SkimUnimplementedError(e)),
            },
        }
    }
}

impl<T> BrmErrorUtilsMessage<T> for Result<T, toml::de::Error> {
    fn to_brm_error_message(self, message: impl ToString) -> Result<T, BrmError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(BrmError::DeserializeError {
                error: e,
                message: message.to_string(),
            }),
        }
    }
}

impl<T> BrmErrorUtilsMessage<T> for Result<T, toml::ser::Error> {
    fn to_brm_error_message(self, message: impl ToString) -> Result<T, BrmError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(BrmError::SerializeError {
                error: e,
                message: message.to_string(),
            }),
        }
    }
}

impl<T> BrmErrorUtils<T> for Result<T, eyre::Report> {
    fn to_brm_error(self) -> Result<T, BrmError> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(BrmError::SkimEyreReport(e)),
        }
    }
}
