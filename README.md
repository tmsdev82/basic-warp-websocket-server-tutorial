# Rust websocket server tutorial

This repository to the article on my blog describing how to write a websocket server with Rust. The article can be found on my blog here: [TMS Blog - Rust Warp WebSocket server](https://tms-dev-blog.com/build-basic-rust-websocket-server/)

WebSocket clients will be able to connect to 127.0.0.1:8000/ws and send a "ping" text message to receive a "pong" message back.

## Running the server

Simply use the use the `cargo run` command to run the program. There is no additional configuration needed.

The server will run on 127.0.0.1:8000.