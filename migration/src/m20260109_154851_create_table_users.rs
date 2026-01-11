use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLE_NAME: &str = "users";
const ENUM_NAME: &str = "user_status";
const ENUM_VALUES: [&str; 3] = ["inactive", "active", "banned"];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ENUM_NAME)
                    .values(ENUM_VALUES)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TABLE_NAME)
                    .if_not_exists()
                    .col(pk_uuid("id").default(Expr::cust("uuidv7()")))
                    // Credentials
                    .col(string_uniq("username"))
                    .col(string_uniq("email"))
                    .col(string("password_hash")) // argon2 hash
                    // 2FA
                    .col(boolean("totp_enabled").default(false))
                    .col(string_null("totp_secret_enc")) // base32 encoded wiht aes
                    .col(json_binary_null("recover_codes_enc")) // aes encrypted array
                    // Refresh Session Management
                    .col(integer("token_version")) // for mass revocation
                    // Account State
                    .col(boolean("email_verified").default(false))
                    .col(enumeration("user_status", ENUM_NAME, ENUM_VALUES).default(ENUM_VALUES[0]))
                    // Timestamps
                    .col(timestamp_with_time_zone("created_at").default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone("updated_at").default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_created_at")
                    .table(TABLE_NAME)
                    .col("created_at")
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_users_updated_at")
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
            .drop_type(Type::drop().name(ENUM_NAME).to_owned())
            .await?;

        Ok(())
    }
}
