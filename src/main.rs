use std::{env, time::Duration};

use anyhow::Result;
use axum::{Router, response::Html, routing::get};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod utils;
use utils::shutdown_signal;

async fn handler() -> Html<&'static str> {
  Html("<h1>Plast Mem</h1>")
}

#[tokio::main]
async fn main() -> Result<()> {
  dotenvy::dotenv().ok();
  tracing_subscriber::fmt().init();

  // TODO: DatabaseConnection
  // https://github.com/tokio-rs/axum/blob/main/examples/sqlx-postgres/src/main.rs
  let pool = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(3))
    .connect(
      env::var("DATABASE_URL")
        // TODO: unwrap_or_else
        .expect("plast-mem: invalid database url")
        .as_str(),
    )
    .await?;

  let app = Router::new().route("/", get(handler)).with_state(pool);
  let listener = TcpListener::bind("0.0.0.0:3000").await?;

  tracing::info!("server started at http://0.0.0.0:3000");

  Ok(
    axum::serve(listener, app)
      .with_graceful_shutdown(shutdown_signal())
      .await?,
  )
}
