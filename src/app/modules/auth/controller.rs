use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;

use crate::app::providers::guards::claims::RefreshClaims;

use crate::app::modules::auth::services::helpers;

pub fn routes() -> Vec<rocket::Route> {
    routes![auth_bypass, auth, login_options, login]
}

// WARNING: This is only for testing purposes
#[get("/bypass/<id>")]
pub async fn auth_bypass(cookie: &CookieJar<'_>, id: i32) -> Result<Json<String>, Status> {
    let user_in_claims = helpers::user_request(id).await;
    if let Err(_) = user_in_claims {
        return Err(Status::InternalServerError);
    }
    let user_in_claims = user_in_claims.unwrap();

    let tokens = helpers::token_generator(user_in_claims).await;

    if let Err(_) = tokens {
        return Err(Status::NotFound);
    }
    let (access_token, refresh_token) = tokens.unwrap();

    cookie.add_private(Cookie::new("refresh_token", refresh_token));

    Ok(Json(access_token))
}

#[get("/")]
pub async fn auth(cookie: &CookieJar<'_>, claims: RefreshClaims) -> Result<Json<String>, Status> {
    let user_in_claims = helpers::user_request(claims.0.user.id).await;
    if let Err(_) = user_in_claims {
        return Err(Status::InternalServerError);
    }
    let user_in_claims = user_in_claims.unwrap();

    // En este punto son diferentes
    // if user_in_claims.user_token != claims.user.user_token {
    //     return Err(Status::Unauthorized);
    // }

    match helpers::token_generator(user_in_claims).await {
        Ok((refresh_token, access_token)) => {
            cookie.add_private(Cookie::new("refresh_token", refresh_token));

            Ok(Json(access_token))
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[options("/login")]
pub async fn login_options() -> Status {
    println!("AUTH: login_options");
    Status::Ok
}

#[post("/login", data = "<token>")]
pub async fn login(cookie: &CookieJar<'_>, token: String) -> Result<Json<String>, Status> {
    // Request the user_id from the profile api
    let response = helpers::profile_request(token).await;
    if let Err(e) = response {
        return Err(e);
    }
    let response = response.unwrap();

    let user_in_claims = helpers::user_request(response).await;
    if let Err(_) = user_in_claims {
        return Err(Status::InternalServerError);
    }
    let user_in_claims = user_in_claims.unwrap();

    match helpers::token_generator(user_in_claims).await {
        Ok((refresh_token, access_token)) => {
            cookie.add_private(Cookie::new("refresh_token", refresh_token.clone()));

            Ok(Json(access_token))
        }
        Err(e) => {
            return Err(e);
        }
    }
}
