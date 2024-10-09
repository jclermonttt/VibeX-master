pub use sea_orm_migration::prelude::*;
use migration_auth::m20241008_203354_add_users_table::Migration as AuthMigration;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(AuthMigration),
        ]
    }
}
