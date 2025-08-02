use rocket_ws::Message;
use serde::{Serialize, Deserialize};
use serde_json;

use crate::Protocol::SenkaError::{SenkaError, SenkaErrorCode};

#[derive(Serialize, Deserialize)]
pub struct  ShellCommand {
    
    pub command: String

}

impl ShellCommand {

    pub fn from_str(message: &str) -> Self {
        ShellCommand { 
            command: String::from(message)
        }
    }

    pub fn from_string(message: String) -> Self {
        ShellCommand
        { 
            command: String::from(message)
        }
    }

    pub fn from_message(message: &Message) -> Result<Self, SenkaError> {
        if let Message::Text(message_raw_text) = message
        {
            if let Ok(message_shell_command) = serde_json::from_str::<ShellCommand>(&message_raw_text) {
                return Ok(message_shell_command);
            }
            else {
                return Err(SenkaError::new(SenkaErrorCode::Format,format!("Can not parse message:{}", message)))
            }
        }
        Err(SenkaError::new(SenkaErrorCode::Format,format!("Message is not text:{}", message)))
    }
    
    pub fn is_sudo(&self) -> bool {
        self.command.contains("sudo")
    }
}

#[derive(Serialize, Deserialize)]
pub struct ShellCommandResponse {
    pub message: String,
    pub hold_on: bool,
}