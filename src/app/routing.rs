pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket.mount("/", routes![health])
    })
}

#[get("/health")]
fn health() -> &'static str {
    "OK"
}
