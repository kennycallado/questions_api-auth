use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, diesel};

use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;

#[database("questions")]
pub struct Db(diesel::PgConnection);

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/database/migrations");

    if ConfigGetter::get_migrations_run().unwrap_or(true) {
        Db::get_one(&rocket)
            .await
            .expect("ERROR: database.run_migrations(); database connection")
            .run(|conn| {
                conn.run_pending_migrations(MIGRATIONS)
                    .expect("ERROR: database.run_migrations(); diesel migrations");
            })
            .await;
    }

    rocket
}
