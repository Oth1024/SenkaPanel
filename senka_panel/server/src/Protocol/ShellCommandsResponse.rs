use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ShellCommandsResponse {
    pub message: String,
    pub hold_on: bool,
}