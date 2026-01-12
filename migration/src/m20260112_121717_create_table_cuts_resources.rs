use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "cuts_resources";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(uuid("cut_id"))
                    .col(uuid("resource_id"))
                    .col(string("usage").default("video")) // video, cover, thumbnail, gallery, banner, ...
                    .col(integer("sort_order").default(0)) // 0 means primary
                    .col(integer_null("votes"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    // compound primary key, few reverse lookups so no reverse index
                    .primary_key(Index::create().col("cut_id").col("resource_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cuts_resources_cut_id")
                            .from_col("cut_id")
                            .to("cuts", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cuts_resources_resource_id")
                            .from_col("resource_id")
                            .to("resources", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cuts_resources_created_at")
                    .table(TABLE_NAME)
                    .col("created_at")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cuts_resources_updated_at")
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
