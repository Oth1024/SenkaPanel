use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum SenkaErrorCode {

    Unknown,

    Format,
    
    OutOfRange,

    Internal

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
            error_message
        }
    }
}