use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "resources";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(pk_uuid("id").default(Expr::cust("uuidv7()")))
                    .col(string_uniq("storage_key"))
                    .col(string("bucket_name"))
                    .col(string("category"))
                    .col(string("mime_type"))
                    .col(big_integer("file_size_bytes"))
                    .col(json_binary("metadata").default("{}"))
                    .col(string_null("summary"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(TABLE_NAME)
                    .name("idx_resources_category")
                    .col("category")
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
