use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    println!("Norm Developer Server started");

    let app = Router::new().route("/", get(|| async { "Norm Developer Server" }));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
