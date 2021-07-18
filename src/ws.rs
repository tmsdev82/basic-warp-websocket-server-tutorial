use warp::ws::WebSocket;

pub async fn client_connection(ws: WebSocket) {
    println!("establishing client connection... {:?}", ws);
}
