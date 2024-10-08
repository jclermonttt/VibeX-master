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
                    .col(
                        ColumnDef::new(User::Id)
                            .uuid()
                            .unique_key()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Firstname)
                            .string()
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Lastname)
                            .string()
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Username)
                            .string()
                            .string_len(50)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::Email)
                            .string()
                            .string_len(255)
                            .unique_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::HashedPassword)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::ProfilePictureUrl)
                            .string()
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(
                        ColumnDef::new(User::CoverPictureUrl)
                            .string()
                            .string_len(255)
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(User::Birthday).date().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT NOW()")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .extra("DEFAULT NOW()")
                            .not_null(),
                    )
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
    Id,
    Firstname,
    Lastname,
    Email,
    Username,
    HashedPassword,
    ProfilePictureUrl,
    CoverPictureUrl,
    Birthday,
    CreatedAt,
    UpdatedAt,
}
