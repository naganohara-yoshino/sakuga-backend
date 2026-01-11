use sea_orm_migration::prelude::{extension::postgres::Type, *};

#[derive(DeriveMigrationName)]
pub struct Migration;

pub const ENUM_WIKI_STATUS_NAME: &str = "wiki_status";
pub const ENUM_WIKI_STATUS_VALUES: [&str; 5] =
    ["draft", "published", "locked", "hidden", "deleted"];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ENUM_WIKI_STATUS_NAME)
                    .values(ENUM_WIKI_STATUS_VALUES)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().name(ENUM_WIKI_STATUS_NAME).to_owned())
            .await
    }
}
