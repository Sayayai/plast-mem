use std::{env, time::Duration};

use axum::{Router, extract::State, response::Html, routing::get};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;

mod api;

mod core;

mod utils;
use utils::{AppError, shutdown_signal};

#[axum::debug_handler]
async fn handler() -> Html<&'static str> {
  Html("<h1>Plast Mem</h1>")
}

#[axum::debug_handler]
async fn using_connection_pool_extractor(State(pool): State<PgPool>) -> Result<String, AppError> {
  Ok(
    sqlx::query_scalar("select 'hello world from pg'")
      .fetch_one(&pool)
      .await?,
  )
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
  dotenvy::dotenv().ok();
  tracing_subscriber::fmt::init();

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

  let app = Router::new()
    .route("/", get(handler).post(using_connection_pool_extractor))
    .merge(api::app())
    .with_state(pool);
  let listener = TcpListener::bind("0.0.0.0:3000").await?;

  tracing::info!("server started at http://0.0.0.0:3000");

  Ok(
    axum::serve(listener, app)
      .with_graceful_shutdown(shutdown_signal())
      .await?,
  )
}
