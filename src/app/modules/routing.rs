use super::auth::controller as auth_controller;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket.mount("/auth", auth_controller::routes())
    })
}
