pub use sea_orm_migration::prelude::*;

mod m20241008_204022_add_profiles_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241008_204022_add_profiles_table::Migration),
        ]
    }
}
