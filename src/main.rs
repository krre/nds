use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT")
        .ok()
        .map(|p| p.parse::<u16>())
        .unwrap()?;

    println!("Norm Developer Server started on port {}", port);

    let app = Router::new().route("/", get(|| async { "Norm Developer Server" }));

    axum::Server::bind(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        port,
    ))
    .serve(app.into_make_service())
    .await?;

    Ok(())
}
