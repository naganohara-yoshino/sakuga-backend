use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "users_roles";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(ColumnDef::new("user_id").uuid().not_null())
                    .col(ColumnDef::new("role_id").integer().not_null())
                    .col(timestamp_with_time_zone("assigned_at").default(Expr::current_timestamp()))
                    .primary_key(Index::create().col("user_id").col("role_id")) // specify primary key
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_roles_user_id")
                            .from(TABLE_NAME, "user_id")
                            .to("users", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_roles_role_id")
                            .from(TABLE_NAME, "role_id")
                            .to("roles", "id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
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
