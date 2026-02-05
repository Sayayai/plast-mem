use axum::{Json, http::StatusCode};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
  core::{Message, MessageRole},
  utils::AppError,
};

#[derive(Deserialize)]
pub struct AddMessage {
  pub conversation_id: Uuid,
  pub message: AddMessageMessage,
}

#[derive(Deserialize)]
pub struct AddMessageMessage {
  pub role: MessageRole,
  pub content: String,
  #[serde(
    with = "chrono::serde::ts_milliseconds_option",
    skip_serializing_if = "Option::is_none"
  )]
  pub timestamp: Option<DateTime<Utc>>,
}

#[axum::debug_handler]
pub async fn add_message(
  Json(payload): Json<AddMessage>,
) -> Result<(StatusCode, Json<Message>), AppError> {
  let timestamp = if let Some(timestamp) = payload.message.timestamp {
    timestamp
  } else {
    Utc::now()
  };

  let message = Message {
    role: payload.message.role,
    content: payload.message.content,
    timestamp,
  };

  Ok((StatusCode::NOT_IMPLEMENTED, Json(message)))
}
