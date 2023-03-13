use super::*;
use rocket::http::{Accept, ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn test_health() {
    let client = Client::tracked(app::server::rocket()).unwrap();
    let response = client
        .get("/health")
        .header(Accept::JSON)
        // .header(Header::new("Authorization", format!("Bearer {bearer}")))
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("OK".into()));
}
