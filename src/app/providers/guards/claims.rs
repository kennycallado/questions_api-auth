use rocket::request::{FromRequest, Outcome, Request};

use crate::app::providers::interfaces::helpers::claims::{Claims, ClaimsError};
use crate::app::providers::interfaces::helpers::token::Token;

pub struct AccessClaims(pub Claims);
pub struct RefreshClaims(pub Claims);

#[async_trait]
impl<'r> FromRequest<'r> for RefreshClaims {
    type Error = ClaimsError;

    // This claims is created from the private cookie
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token: Token = match Token::from_cookie(request) {
            Some(token) => token,
            None => {
                return Outcome::Failure((
                    rocket::http::Status::BadRequest,
                    ClaimsError::MissingToken,
                ));
            }
        };

        let claims = match token.decode() {
            Ok(claims) => claims.claims,
            Err(e) => {
                println!("Error: {:?}", e);
                return Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    ClaimsError::InvalidToken,
                ));
            }
        };

        Outcome::Success(RefreshClaims(claims))
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for AccessClaims {
    type Error = ClaimsError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match Token::from_header(request) {
            Some(token) => token,
            None => return Outcome::Forward(()),
        };

        let claims = match token.decode() {
            Ok(claims) => claims.claims,
            Err(e) => {
                println!("Error: {:?}", e);
                return Outcome::Failure((
                    rocket::http::Status::Unauthorized,
                    ClaimsError::InvalidToken,
                ));
            }
        };

        Outcome::Success(AccessClaims(claims))
    }
}
