use std::net::SocketAddr;
use axum::{routing::get, routing::post,Router};
use axum::response::Html;
use serde::Serialize;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(hello_json))
        .route("/post", post(hello_post));
    let addr = SocketAddr::from(([127,0,0,1], 3001));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Serialize)]
struct HelloJson{
    message: String,
}

async fn hello_post() -> Html<String>{
    Html("Hello from post".to_string())
}
async fn hello_json() -> axum::Json<HelloJson>{
    let message = HelloJson { message: "Hello from JSON".to_string()};
    axum::Json(message)
}
async fn say_hello_text() -> Html<&'static str>{
    Html("<h1>Hello, world!</h1>")
}