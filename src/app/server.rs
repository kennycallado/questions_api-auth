use rocket::fairing::AdHoc;

use crate::config::cors;
use crate::config::database;

use super::modules::routing as modules_routing;
use super::routing as service_routing;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(service_routing::router())
        .attach(modules_routing::router())
        .attach(database::Db::fairing())
        .attach(AdHoc::on_ignite(
            "Diesel Migrations",
            database::run_migrations,
        ))
        .attach(cors::Cors)
}
