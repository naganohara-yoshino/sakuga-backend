pub use sea_orm_migration::prelude::*;

mod m20260109_154851_create_table_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260109_154851_create_table_users::Migration)]
    }
}
