pub use sea_orm_migration::prelude::*;

mod m20260109_154851_create_table_users;
mod m20260110_113127_create_table_roles;
mod m20260110_114928_create_table_permissions;
mod m20260110_115554_create_table_users_roles;
mod m20260110_152253_create_table_roles_permissions;
mod m20260110_174317_create_table_resources;
mod m20260110_180525_create_type_wiki_status;
mod m20260110_181101_create_table_works;
mod m20260111_051446_create_table_works_resources;
mod m20260111_054204_create_table_persons;
mod m20260111_145342_create_table_work_credits;
mod m20260111_171227_create_table_cuts;
mod m20260112_114932_create_table_cut_credits;
mod m20260112_121717_create_table_cuts_resources;

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
            Box::new(m20260110_180525_create_type_wiki_status::Migration),
            Box::new(m20260110_181101_create_table_works::Migration),
            Box::new(m20260111_051446_create_table_works_resources::Migration),
            Box::new(m20260111_054204_create_table_persons::Migration),
            Box::new(m20260111_145342_create_table_work_credits::Migration),
            Box::new(m20260111_171227_create_table_cuts::Migration),
            Box::new(m20260112_114932_create_table_cut_credits::Migration),
            Box::new(m20260112_121717_create_table_cuts_resources::Migration),
        ]
    }
}
