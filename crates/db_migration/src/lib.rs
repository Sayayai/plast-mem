pub use sea_orm_migration::*;

mod m20260206_01_create_message_queue_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![Box::new(m20260206_01_create_message_queue_table::Migration)]
  }
}
