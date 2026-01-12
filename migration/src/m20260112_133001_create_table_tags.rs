use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "tags";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(pk_uuid("id").default(Expr::cust("uuidv7()")))
                    .col(string_uniq("name")) // implicit index
                    .col(integer("ref_count").default(0))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(TABLE_NAME)
                    .name("idx_tags_ref_count")
                    .col("ref_count")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(TABLE_NAME)
                    .name("idx_tags_created_at")
                    .col("created_at")
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
