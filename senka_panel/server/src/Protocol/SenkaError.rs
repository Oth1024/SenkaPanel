use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub enum SenkaErrorCode {
    Unknown,

    Unexpected,

    NoImplementation,

    Format,

    OutOfRange,

    Inner,
}

impl Display for SenkaErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SenkaErrorCode::Unknown => write!(f, "{}", "Unknown"),
            SenkaErrorCode::Unexpected => write!(f, "{}", "Unexpected"),
            SenkaErrorCode::NoImplementation => write!(f, "{}", "NoImplementation"),
            SenkaErrorCode::Format => write!(f, "{}", "Format"),
            SenkaErrorCode::OutOfRange => write!(f, "{}", "OutOfRange"),
            SenkaErrorCode::Inner => write!(f, "{}", "Inner"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SenkaError {
    pub senka_error_code: SenkaErrorCode,
    pub error_message: String,
}

impl SenkaError {
    pub fn new(senka_error_code: SenkaErrorCode, error_message: String) -> Self {
        SenkaError {
            senka_error_code,
            error_message: String::from(error_message),
        }
    }

    pub fn from_message(error_message: String) -> Self {
        SenkaError {
            senka_error_code: SenkaErrorCode::Inner,
            error_message: String::from(error_message),
        }
    }

    pub fn from_err(senka_error_code: SenkaErrorCode) -> Self {
        SenkaError {
            error_message: senka_error_code.to_string(),
            senka_error_code,
        }
    }

    pub fn null() -> Self {
        let error = SenkaErrorCode::Unknown;
        SenkaError {
            error_message: error.to_string(),
            senka_error_code: error,
        }
    }
}

impl Display for SenkaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]{}", self.senka_error_code, self.error_message)
    }
}
