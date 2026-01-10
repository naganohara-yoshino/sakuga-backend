pub use sea_orm_migration::prelude::*;

mod m20260109_154851_create_table_users;
mod m20260110_113127_create_table_roles;
mod m20260110_114928_create_table_permissions;
mod m20260110_115554_create_table_users_roles;
mod m20260110_152253_create_table_roles_permissions;
mod m20260110_174317_create_table_resources;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260109_154851_create_table_users::Migration),
            Box::new(m20260110_113127_create_table_roles::Migration),
            Box::new(m20260110_114928_create_table_permissions::Migration),
            Box::new(m20260110_115554_create_table_users_roles::Migration),
            Box::new(m20260110_152253_create_table_roles_permissions::Migration),
            Box::new(m20260110_174317_create_table_resources::Migration),
        ]
    }
}
