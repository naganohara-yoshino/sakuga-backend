use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "work_credits";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(pk_uuid("id").default(Expr::cust("uuidv7()")))
                    .col(uuid("work_id"))
                    .col(uuid("person_id"))
                    .col(string("position"))
                    .col(string_null("summary"))
                    .col(json_binary_null("info"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_work_credits_work_id")
                            .from(TABLE_NAME, "work_id")
                            .to("works", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_work_credits_person_id")
                            .from(TABLE_NAME, "person_id")
                            .to("persons", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_work_credits_work_person")
                    .table(TABLE_NAME)
                    .col("work_id")
                    .col("person_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_work_credits_person_work")
                    .table(TABLE_NAME)
                    .col("person_id")
                    .col("work_id")
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
