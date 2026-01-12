use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "roles_permissions";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(integer("role_id"))
                    .col(integer("permission_id"))
                    .primary_key(Index::create().col("role_id").col("permission_id")) // specify primary key
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_roles_permissions_role_id")
                            .from_col("role_id")
                            .to("roles", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_roles_permissions_permission_id")
                            .from_col("permission_id")
                            .to("permissions", "id")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_roles_permissions_permission_role")
                    .table(TABLE_NAME)
                    .col("permission_id")
                    .col("role_id")
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
