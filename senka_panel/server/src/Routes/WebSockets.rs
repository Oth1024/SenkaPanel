use rocket::{self, get};

use rocket_ws as ws;

#[get("/channel?ssh")]
pub fn ssh_stream(ws: ws::WebSocket) -> ws::Stream![] {
    ws::Stream! { ws =>
        for await message in ws {
            yield message?;
            yield "foo".into();
            yield vec![1, 2, 3, 4].into();
        }
    }
}