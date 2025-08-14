use crate::{
    Protocol::ShellCommand::{ShellCommand, ShellCommandResponse},
};
use rocket::{self, get};
use rocket_ws as ws;
use serde_json::to_string_pretty;
use rocket_ws::Message::Text;

#[get("/channel?ssh")]
pub fn ssh_stream(ws: ws::WebSocket) -> ws::Stream![] {
    ws::Stream! { ws =>
        for await message in ws {

            yield message?;
        }
    }
}
