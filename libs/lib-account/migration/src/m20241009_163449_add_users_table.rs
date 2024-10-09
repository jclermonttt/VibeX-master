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
                    .if_not_exists()
                    .col(uuid_uniq(User::Id).not_null().primary_key())
                    .col(string(User::FirstName).not_null())
                    .col(string(User::LastName).not_null())
                    .col(string_uniq(User::Username))
                    .col(string_uniq(User::Email).not_null())
                    .col(string_null(User::EmailVerificationToken))
                    .col(timestamp_null(User::EmailVerifiedAt))
                    .col(boolean(User::IsActive).default(true))
                    .col(timestamp(User::LastLoginAt))
                    .col(string(User::PasswordHash).not_null())
                    .col(timestamp_null(User::PasswordResetExpiresAt))
                    .col(string_null(User::PasswordResetToken))
                    .col(timestamp(User::CreatedAt).not_null())
                    .col(timestamp(User::UpdatedAt).not_null())
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
pub enum User {
    Table,
    Id,
    FirstName,
    LastName,
    Username,
    Email,
    EmailVerificationToken,
    EmailVerifiedAt,
    IsActive,
    LastLoginAt,
    PasswordHash,
    PasswordResetExpiresAt,
    PasswordResetToken,
    CreatedAt,
    UpdatedAt,
}
