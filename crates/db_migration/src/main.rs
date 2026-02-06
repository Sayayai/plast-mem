use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
  cli::run_cli(plast_mem_db_migration::Migrator).await
}
