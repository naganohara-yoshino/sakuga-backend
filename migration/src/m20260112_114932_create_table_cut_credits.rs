use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "cut_credits";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(pk_uuid("id").default(Expr::cust("uuidv7()")))
                    .col(uuid("cut_id"))
                    .col(uuid("person_id"))
                    .col(string("job").default("key frame")) // default to key frame
                    .col(boolean("is_sure").default(false)) // default not sure
                    .col(string_null("summary"))
                    .col(json_binary_null("info"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cut_credits_cut_id")
                            .from_col("cut_id")
                            .to("cuts", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cut_credits_person_id")
                            .from_col("person_id")
                            .to("persons", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cut_credits_cut_person")
                    .table(TABLE_NAME)
                    .col("cut_id")
                    .col("person_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cut_credits_person_cut")
                    .table(TABLE_NAME)
                    .col("person_id")
                    .col("cut_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cut_credits_info")
                    .table(TABLE_NAME)
                    .col("info")
                    .index_type(IndexType::Custom("GIN".into())) // gin index for jsonb
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cut_credits_person")
                    .table(TABLE_NAME)
                    .col("person_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cut_credits_created_at")
                    .table(TABLE_NAME)
                    .col("created_at")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cut_credits_updated_at")
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
