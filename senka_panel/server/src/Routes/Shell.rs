use crate::{
    Executor::ShellExecutor::ShellExecutor,
    Protocol::ShellCommand::{ShellCommand, ShellCommandResponse},
};
use rocket::{self, get};
use rocket_ws as ws;
use serde_json::to_string_pretty;
use rocket_ws::Message::Text;

#[get("/channel?ssh")]
pub fn ssh_stream(ws: ws::WebSocket) -> ws::Stream![] {
    let mut shell_executor = ShellExecutor::new();
    ws::Stream! { ws =>
        for await message in ws {
            if let Ok(raw_message) = &message {
                if let Ok(raw_request) = ShellCommand::from_message(&raw_message) {
                    if let Ok(response) = shell_executor.execute(&raw_request) {
                        yield Text(to_string_pretty(&response).unwrap());
                    }
                }
            }
            yield message?;
        }
    }
}
