use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20260110_180525_create_type_wiki_status::{ENUM_WIKI_STATUS_NAME, ENUM_WIKI_STATUS_VALUES},
    m20260111_053000_create_type_work_scope::{ENUM_WORK_SCOPE_NAME, ENUM_WORK_SCOPE_VALUES},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "cuts";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(pk_uuid("id").default(Expr::cust("uuidv7()")))
                    .col(boolean("is_nsfw").default(false))
                    .col(enumeration(
                        "wiki_status",
                        ENUM_WIKI_STATUS_NAME,
                        ENUM_WIKI_STATUS_VALUES,
                    ))
                    .col(uuid_null("work_id"))
                    .col(
                        enumeration_null(
                            "scope_type",
                            ENUM_WORK_SCOPE_NAME,
                            ENUM_WORK_SCOPE_VALUES,
                        )
                        .default(ENUM_WORK_SCOPE_VALUES[1]),
                    ) // default scope EP
                    .col(integer_null("scope_number").default(1)) // use 1 for movies and shorts
                    .col(interval_null("start_time", None, None))
                    .col(interval_null("end_time", None, None))
                    .col(json_binary_null("summary"))
                    .col(json_binary_null("info"))
                    .col(uuid_null("posted_by"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cuts_work_id")
                            .from_col("work_id")
                            .to("works", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cuts_posted_by")
                            .from_col("posted_by")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cuts_work_id")
                    .table(TABLE_NAME)
                    .col("work_id")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cuts_info")
                    .table(TABLE_NAME)
                    .col("info")
                    .index_type(IndexType::Custom("GIN".into())) // gin index for jsonb
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cuts_posted_by")
                    .table(TABLE_NAME)
                    .col("posted_by")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cuts_created_at")
                    .table(TABLE_NAME)
                    .col("created_at")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_cuts_updated_at")
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
            .await?;

        Ok(())
    }
}
