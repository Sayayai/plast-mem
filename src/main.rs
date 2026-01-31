use anyhow::Result;
use axum::{Router, response::Html, routing::get};
use tokio::net::TcpListener;

mod utils;
use utils::shutdown_signal;

async fn handler() -> Html<&'static str> {
  Html("<h1>Plast Mem</h1>")
}

#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt().init();

  let app = Router::new().route("/", get(handler));
  let listener = TcpListener::bind("0.0.0.0:3000").await?;

  tracing::info!("server started at http://0.0.0.0:3000");

  Ok(
    axum::serve(listener, app)
      .with_graceful_shutdown(shutdown_signal())
      .await?,
  )
}
