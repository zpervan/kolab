use axum::{
    routing::get, 
    Router, Json
};
use tokio::net::TcpListener;
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    text: String,
}

async fn hello_world() -> Json<Message> {
    Json(Message { text: "Hello from Rust Backend!".to_string() })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let listener = TcpListener::bind("127.0.0.1:9090").await.unwrap();
    println!("Server running at http://127.0.0.1:9090");
    axum::serve(listener, app).await.unwrap();
}