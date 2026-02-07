## 2026-02-07 - Generic Error Responses
**Vulnerability:** Internal error messages were leaked to clients via `AppError`'s `IntoResponse` implementation.
**Learning:** Returning `anyhow::Error` directly in the response body can expose sensitive system details like database schemas or environment configuration.
**Prevention:** Implement `IntoResponse` for custom error types by logging the detailed error with `tracing::error!` and returning a generic message (e.g., "An internal server error occurred") to the client.
