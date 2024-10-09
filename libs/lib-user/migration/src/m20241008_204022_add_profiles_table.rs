use migration_auth::m20241008_203354_add_users_table::User;
use sea_orm::sqlx::types::chrono;
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
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(pk_auto(Profile::Id))
                    .col(integer(Profile::UserId).not_null())
                    .col(string(Profile::Username).not_null().unique_key())
                    .col(string(Profile::ProfilePicture))
                    .col(string(Profile::Bio))
                    .col(
                        timestamp(Profile::CreatedAt)
                            .not_null()
                            .default(chrono::Utc::now()),
                    )
                    .col(
                        timestamp(Profile::UpdatedAt)
                            .not_null()
                            .default(chrono::Utc::now()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Profile::Table, Profile::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Profile {
    Table,
    Id,
    UserId,
    Username,
    ProfilePicture,
    Bio,
    CreatedAt,
    UpdatedAt,
}
