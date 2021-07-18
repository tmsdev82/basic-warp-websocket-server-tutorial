use crate::{ws, Result};
use warp::Reply;

pub async fn ws_handler(ws: warp::ws::Ws) -> Result<impl Reply> {
    println!("ws_handler");

    Ok(ws.on_upgrade(move |socket| ws::client_connection(socket)))
}
