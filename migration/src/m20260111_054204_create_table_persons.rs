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
                    .col(string_null("description"))
                    .col(json_binary_null("info"))
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persons_image_resource_id")
                            .from(TABLE_NAME, "image_resource_id")
                            .to("resources", "id")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TABLE_NAME).to_owned())
            .await
    }
}
