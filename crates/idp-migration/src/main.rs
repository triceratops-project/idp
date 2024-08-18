use async_std::task;
use sea_orm_migration::prelude::*;

fn main() {
    task::block_on(async {
        cli::run_cli(migration::Migrator).await;
    })
}
