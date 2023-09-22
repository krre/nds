use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use sqlx;
use sqlx::postgres::PgPoolOptions;

use axum::{routing::get, Router};
use log::info;

pub struct Config {}

pub struct Application {
    config: Config,
}

impl Application {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = Config {};
        Ok(Self { config })
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let database_url = std::env::var("DATABASE_URL")?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        sqlx::migrate!().run(&pool).await?;

        let port = std::env::var("PORT")
            .ok()
            .map(|p| p.parse::<u16>())
            .unwrap()?;

        info!("Norm Developer Server started on port {}", port);

        let app = Router::new().route("/", get(|| async { "Norm Developer Server" }));

        axum::Server::bind(&SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            port,
        ))
        .serve(app.into_make_service())
        .await?;
        Ok(())
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
