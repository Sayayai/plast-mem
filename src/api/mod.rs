use axum::{Router, routing::post};
use sqlx::{Pool, Postgres};

mod add_message;

pub fn app() -> Router<Pool<Postgres>> {
  Router::new().route("/api/v0/add_message", post(add_message::add_message))
}
