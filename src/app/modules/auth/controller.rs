use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::app::providers::guards::claims::RefreshClaims;

use crate::app::modules::auth::services::helpers;
use crate::app::providers::interfaces::helpers::claims::UserInClaims;

pub fn routes() -> Vec<rocket::Route> {
    routes![auth_bypass, auth, login_options, login, logout]
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthUser {
    pub user: UserInClaims,
    pub access_token: String,
}

// WARNING: This is only for testing purposes
#[get("/bypass/<id>")]
pub async fn auth_bypass(cookie: &CookieJar<'_>, id: i32) -> Result<Json<AuthUser>, Status> {
    let user_in_claims = helpers::user_request(id).await;
    if let Err(_) = user_in_claims {
        return Err(Status::InternalServerError);
    }
    let user_in_claims = user_in_claims.unwrap();

    let tokens = helpers::token_generator(user_in_claims.clone()).await;

    if let Err(_) = tokens {
        return Err(Status::NotFound);
    }
    let (access_token, refresh_token) = tokens.unwrap();

    cookie.add_private(Cookie::new("refresh_token", refresh_token));

    let auth_user = AuthUser {
        user: user_in_claims,
        access_token,
    };
    Ok(Json(auth_user))
}

#[get("/")]
pub async fn auth(cookie: &CookieJar<'_>, claims: RefreshClaims) -> Result<Json<AuthUser>, Status> {
    let user_in_claims = helpers::user_request(claims.0.user.id).await;
    if let Err(_) = user_in_claims {
        return Err(Status::InternalServerError);
    }
    let user_in_claims = user_in_claims.unwrap();

    match helpers::token_generator(user_in_claims.clone()).await {
        Ok((refresh_token, access_token)) => {
            cookie.add_private(Cookie::new("refresh_token", refresh_token));

            let auth_user = AuthUser {
                user: user_in_claims,
                access_token,
            };

            Ok(Json(auth_user))
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
pub async fn login(cookie: &CookieJar<'_>, token: String) -> Result<Json<AuthUser>, Status> {
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

    match helpers::token_generator(user_in_claims.clone()).await {
        Ok((refresh_token, access_token)) => {
            cookie.add_private(Cookie::new("refresh_token", refresh_token.clone()));

            let auth_user = AuthUser {
                user: user_in_claims,
                access_token,
            };

            Ok(Json(auth_user))
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[get("/logout")]
pub async fn logout(cookie: &CookieJar<'_>, claims: RefreshClaims) -> Status {
    if let Err(_) = helpers::fcm_token_delete(claims.0.user.id).await {
        println!("AUTH: logout: fcm_token_delete failed");
    };

    cookie.remove_private(Cookie::named("refresh_token"));
    Status::Ok
}
