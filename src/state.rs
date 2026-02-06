use std::sync::Arc;

use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

use crate::core::{EpisodicMemory, MessageQueue};

#[derive(Clone, Debug, Default)]
pub struct AppState {
  pub message_queues: Arc<RwLock<Vec<MessageQueue>>>,
  pub memories: Arc<RwLock<Vec<EpisodicMemory>>>,
  pub db: DatabaseConnection,
}

impl AppState {
  pub fn new(db: DatabaseConnection) -> Self {
    Self {
      db,
      ..Self::default()
    }
  }
}
