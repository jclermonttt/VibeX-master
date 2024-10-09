pub use sea_orm_migration::prelude::*;

pub mod m20241008_203354_add_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241008_203354_add_users_table::Migration),
        ]
    }
}
