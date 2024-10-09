use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(uuid_uniq(User::Id))
                    .col(string_uniq(User::Username))
                    .col(string_uniq(User::Email).not_null())
                    .col(string(User::PasswordHash).not_null())
                    .col(string(User::Firstname).not_null())
                    .col(string(User::Lastname).not_null())
                    .col(boolean(User::IsActive).default(true))
                    .col(timestamp(User::CreatedAt).not_null())
                    .col(timestamp(User::UpdatedAt).not_null())
                    .col(timestamp(User::LastLoginAt))
                    .col(string_null(User::PasswordResetToken))
                    .col(timestamp_null(User::PasswordResetExpiresAt))
                    .col(string_null(User::EmailVerificationToken))
                    .col(timestamp_null(User::EmailVerifiedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    CreatedAt,
    Email,
    EmailVerificationToken,
    EmailVerifiedAt,
    Firstname,
    Id,
    IsActive,
    LastLoginAt,
    Lastname,
    PasswordHash,
    PasswordResetExpiresAt,
    PasswordResetToken,
    UpdatedAt,
    Username,
}
