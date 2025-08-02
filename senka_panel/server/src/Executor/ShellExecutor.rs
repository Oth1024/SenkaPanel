use std::process;

use crate::Protocol::{SenkaError::{SenkaError, SenkaErrorCode}, ShellCommand::{ShellCommand, ShellCommandResponse}};

pub struct ShellExecutor {

}

impl ShellExecutor {

    pub fn new() -> Self {
        ShellExecutor {}
    }

    pub fn execute(&mut self, shell_command: &ShellCommand) -> Result<ShellCommandResponse, SenkaError> {
        return Err(SenkaError::from_err(SenkaErrorCode::NoImplementation));
    }

}