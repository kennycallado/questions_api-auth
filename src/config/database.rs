// use rocket::{Build, Rocket};
// use rocket_sync_db_pools::{database, diesel};

// use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;

// #[database("questions")]
// pub struct Db(diesel::PgConnection);

// pub async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
//     embed_migrations!("src/database/migrations");

//     if ConfigGetter::get_migrations_run().unwrap_or(true) {
//         let conn = Db::get_one(&rocket)
//             .await
//             .expect("Error: run_migrations; database connection");

//         conn.run(|conn| embedded_migrations::run(conn))
//             .await
//             .expect("Error: run_migrations; running migrations");
//     }

//     rocket
// }
