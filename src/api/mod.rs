use axum::{Router, routing::post};
use sea_orm::DatabaseConnection;

mod add_message;

pub fn app() -> Router<DatabaseConnection> {
  Router::new().route("/api/v0/add_message", post(add_message::add_message))
}
