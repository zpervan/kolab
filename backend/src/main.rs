use axum::{routing::get, Json, Router};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::cors::{Any, Cors, CorsLayer};

#[derive(Serialize)]
struct Message {
    text: String,
}

async fn hello_world() -> Json<Message> {
    let text = format!("From rust backend: {}", rand::random::<u8>());
    Json(Message { text })
}

#[tokio::main]
async fn main() {
    // TODO: Just a WIP step to check out the front and backend communication
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new().route("/", get(hello_world)).layer(cors);

    let listener = TcpListener::bind("127.0.0.1:9090").await.unwrap();
    println!("Server running at http://127.0.0.1:9090");

    axum::serve(listener, app).await.unwrap();
}
