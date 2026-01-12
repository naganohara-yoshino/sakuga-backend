use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260110_180525_create_type_wiki_status::{
    ENUM_WIKI_STATUS_NAME, ENUM_WIKI_STATUS_VALUES,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "works";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(pk_uuid("id").default(Expr::cust("uuidv7()")))
                    .col(string("name")) // canonical name
                    .col(json_binary_null("name_translations")) // json for i18n
                    .col(enumeration(
                        "wiki_status",
                        ENUM_WIKI_STATUS_NAME,
                        ENUM_WIKI_STATUS_VALUES,
                    ))
                    .col(boolean("is_nsfw").default(false))
                    .col(string_null("category")) // tv, movie, short, ...
                    .col(json_binary_null("summary")) // json for i18n
                    .col(timestamp_with_time_zone_null("release_datetime"))
                    .col(json_binary_null("info")) // extra info
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_works_name")
                    .table(TABLE_NAME)
                    .col("name")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_works_name_translations")
                    .table(TABLE_NAME)
                    .col("name_translations")
                    .index_type(IndexType::Custom("GIN".into())) // gin index for jsonb
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_works_release_datetime")
                    .table(TABLE_NAME)
                    .col("release_datetime")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_works_info")
                    .table(TABLE_NAME)
                    .col("info")
                    .index_type(IndexType::Custom("GIN".into())) // gin index for jsonb
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_works_created_at")
                    .table(TABLE_NAME)
                    .col("created_at")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_works_updated_at")
                    .table(TABLE_NAME)
                    .col("updated_at")
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TABLE_NAME).to_owned())
            .await
    }
}
