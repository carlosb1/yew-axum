use axum::{
    extract::WebSocketUpgrade,
    response::IntoResponse,
    routing::get,
    Router,
};
use axum::extract::ws::{Message, WebSocket};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;
use futures::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    // CORS config para permitir cualquier origen (útil para dev)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("WebSocket connection established");

    while let Some(Ok(msg)) = socket.next().await {
        if let Message::Text(txt) = msg {
            println!("Received: {}", txt);
            let _ = socket.send(Message::Text(format!("Echo: {}", txt))).await;
        }
    }

    println!("WebSocket connection closed");
}
