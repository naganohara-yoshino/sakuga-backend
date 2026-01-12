use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "cuts_tags";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(uuid("cut_id"))
                    .col(uuid("tag_id"))
                    .primary_key(Index::create().col("cut_id").col("tag_id")) // specify primary key
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cuts_tags_cut_id")
                            .from_col("cut_id")
                            .to("cuts", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cuts_tags_tag_id")
                            .from_col("tag_id")
                            .to("tags", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cuts_tags_tag_cut")
                    .table(TABLE_NAME)
                    .col("tag_id")
                    .col("cut_id")
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
