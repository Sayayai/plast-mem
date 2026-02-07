use std::fmt::Display;

use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};

// TODO: https://github.com/launchbadge/realworld-axum-sqlx/blob/main/src/http/error.rs
#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    // Log the actual error for internal debugging
    tracing::error!("Internal server error: {:?}", self.0);

    (
      StatusCode::INTERNAL_SERVER_ERROR,
      "An internal server error occurred",
    )
      .into_response()
  }
}

impl Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl<E> From<E> for AppError
where
  E: Into<anyhow::Error>,
{
  fn from(err: E) -> Self {
    Self(err.into())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use http_body_util::BodyExt;

  #[tokio::test]
  async fn test_error_no_leak() {
    let secret = "sensitive database details";
    let err = AppError(anyhow::anyhow!(secret));
    let response = err.into_response();

    let body_bytes = response
      .into_body()
      .collect()
      .await
      .unwrap()
      .to_bytes();
    let body_str = String::from_utf8_lossy(&body_bytes);

    assert!(
      !body_str.contains(secret),
      "Response body should NOT contain the secret error message"
    );
    assert!(
      body_str.contains("An internal server error occurred"),
      "Response body should contain a generic error message"
    );
  }
}
