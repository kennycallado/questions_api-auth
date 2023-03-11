use rocket::{Build, Rocket};
use rocket_sync_db_pools::{database, diesel};

#[database("questions")]
pub struct Db(diesel::PgConnection);

pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("src/database/migrations");

    let conn = Db::get_one(&rocket)
        .await
        .expect("Error: run_migrations; database connection");
    conn.run(|conn| embedded_migrations::run(conn))
        .await
        .expect("Error: run_migrations; running migrations");

    rocket
}
