use axum::{routing::get, Router};


#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, 332world!"}));

    axum::Server::bind(&"0.0.0.0:3002".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

