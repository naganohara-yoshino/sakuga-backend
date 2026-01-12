use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
};

use crate::m20260110_180525_create_type_wiki_status::{
    ENUM_WIKI_STATUS_NAME, ENUM_WIKI_STATUS_VALUES,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "cuts";
const ENUM_SEGEMENT_NAME: &str = "segment_type";
const ENUM_SEGEMENT_VALUES: [&str; 4] = ["episode", "opening", "ending", "full"];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ENUM_SEGEMENT_NAME)
                    .values(ENUM_SEGEMENT_VALUES)
                    .to_owned(),
            )
            .await?;

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
                    .col(enumeration(
                        "segment_type",
                        ENUM_SEGEMENT_NAME,
                        ENUM_SEGEMENT_VALUES,
                    ))
                    .col(integer("segment_number").default(1)) // use 1 for movies and shorts
                    .col(interval_null("start_time", None, None))
                    .col(interval_null("end_time", None, None))
                    .col(string("name"))
                    .col(uuid_null("thumbnail_resource_id")) // no index because it's not used for search
                    .col(string_null("summary"))
                    .col(json_binary_null("info"))
                    .col(uuid_null("posted_by"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cuts_work_id")
                            .from(TABLE_NAME, "work_id")
                            .to("works", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cuts_thumbnail_resource_id")
                            .from(TABLE_NAME, "thumbnail_resource_id")
                            .to("resources", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_cuts_posted_by")
                            .from(TABLE_NAME, "posted_by")
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
                    .name("idx_cuts_name")
                    .table(TABLE_NAME)
                    .col("name")
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

        manager
            .drop_type(Type::drop().name(ENUM_SEGEMENT_NAME).to_owned())
            .await?;

        Ok(())
    }
}
