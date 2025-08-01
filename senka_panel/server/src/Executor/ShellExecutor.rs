use std::process;

use crate::Protocol::{SenkaError::{SenkaError, SenkaErrorCode}, ShellCommandsResponse::ShellCommandsResponse};

pub struct ShellExecutor {

}

impl ShellExecutor {

    pub fn execute() -> Result<ShellCommandsResponse, SenkaError> {
        return Err(SenkaError::from_err(SenkaErrorCode::NoImplementation));
    }

}