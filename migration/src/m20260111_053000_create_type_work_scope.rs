use sea_orm_migration::prelude::{extension::postgres::Type, *};

#[derive(DeriveMigrationName)]
pub struct Migration;

pub const ENUM_WORK_SCOPE_NAME: &str = "work_scope";
pub const ENUM_WORK_SCOPE_VALUES: [&str; 4] = ["full", "episode", "opening", "ending"];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ENUM_WORK_SCOPE_NAME)
                    .values(ENUM_WORK_SCOPE_VALUES)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(ENUM_WORK_SCOPE_NAME).to_owned())
            .await
    }
}
