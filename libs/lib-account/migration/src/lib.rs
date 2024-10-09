pub use sea_orm_migration::prelude::*;

pub mod m20241009_163449_add_users_table;
mod m20241009_163516_add_profiles_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241009_163449_add_users_table::Migration),
            Box::new(m20241009_163516_add_profiles_table::Migration),
        ]
    }
}
