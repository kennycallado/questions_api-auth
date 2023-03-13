mod app;
mod config;
mod database;

#[cfg(test)]
mod test;

extern crate openssl;

#[allow(unused_imports)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate rocket;
extern crate rocket_sync_db_pools;

fn main() {
    app::server::main();
}
