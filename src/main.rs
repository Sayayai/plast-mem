use std::env;

use axum::{Router, response::Html, routing::get};
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;

mod api;

mod core;

mod utils;
use utils::{AppError, shutdown_signal};

#[axum::debug_handler]
async fn handler() -> Html<&'static str> {
  Html("<h1>Plast Mem</h1>")
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
  dotenvy::dotenv().ok();
  tracing_subscriber::fmt::init();

  let db: DatabaseConnection = Database::connect(
    env::var("DATABASE_URL") // TODO: unwrap_or_else
      .expect("plast-mem: invalid database url")
      .as_str(),
  )
  .await?;

  let app = Router::new()
    .route("/", get(handler))
    .merge(api::app())
    .with_state(db);
  let listener = TcpListener::bind("0.0.0.0:3000").await?;

  tracing::info!("server started at http://0.0.0.0:3000");

  Ok(
    axum::serve(listener, app)
      .with_graceful_shutdown(shutdown_signal())
      .await?,
  )
}
