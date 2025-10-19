pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users;
mod m20250914_055232_updated_at_trigger;
mod m20250914_055447_add_updated_at_trigger_in_users;
mod m20251008_141025_retreat_and_retreat_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users::Migration),
            Box::new(m20250914_055232_updated_at_trigger::Migration),
            Box::new(m20250914_055447_add_updated_at_trigger_in_users::Migration),
            Box::new(m20251008_141025_retreat_and_retreat_users::Migration),
        ]
    }
}
