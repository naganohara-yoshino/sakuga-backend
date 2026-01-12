use crate::m20260110_180525_create_type_wiki_status::{
    ENUM_WIKI_STATUS_NAME, ENUM_WIKI_STATUS_VALUES,
};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "persons";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(pk_uuid("id").default(Expr::cust("uuidv7()")))
                    .col(string("name"))
                    .col(enumeration(
                        "wiki_status",
                        ENUM_WIKI_STATUS_NAME,
                        ENUM_WIKI_STATUS_VALUES,
                    ))
                    .col(uuid_null("image_resource_id")) // cover image
                    .col(string_null("summary"))
                    .col(json_binary_null("info"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persons_image_resource_id")
                            .from_col("image_resource_id")
                            .to("resources", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_persons_name")
                    .table(TABLE_NAME)
                    .col("name")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_persons_image_resource_id")
                    .table(TABLE_NAME)
                    .col("image_resource_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_persons_info")
                    .table(TABLE_NAME)
                    .col("info")
                    .index_type(IndexType::Custom("GIN".into())) // gin index for jsonb
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_persons_created_at")
                    .table(TABLE_NAME)
                    .col("created_at")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_persons_updated_at")
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
