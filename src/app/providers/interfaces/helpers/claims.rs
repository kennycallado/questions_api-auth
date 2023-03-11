use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::app::providers::constants::{
    ACCESS_TOKEN_EXPIRATION, REFRESH_TOKEN_EXPIRATION, ROBOT_TOKEN_EXPIRATION,
};

use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;

#[derive(Debug)]
pub enum ClaimsError {
    MissingToken,
    InvalidToken,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RoleInClaims {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInClaims {
    pub id: i32,
    pub depends_on: i32,
    pub role: RoleInClaims,
    pub user_token: Option<String>,
}

impl Default for UserInClaims {
    fn default() -> Self {
        UserInClaims {
            id: 0,
            depends_on: 0,
            role: RoleInClaims {
                id: 6,
                name: String::from("Guest"),
            },
            user_token: None,
        }
    }
}

impl From<UserInClaims> for Claims {
    fn from(user: UserInClaims) -> Self {
        let iat = chrono::Utc::now().timestamp();
        let exp = iat + ROBOT_TOKEN_EXPIRATION;

        Claims {
            sub: String::new(),
            user,
            iat,
            exp,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub user: UserInClaims,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    pub fn enconde_for_robot(&mut self) -> Result<String, Error> {
        let iat = chrono::Utc::now().timestamp();
        let exp = iat + ROBOT_TOKEN_EXPIRATION;

        self.user.role.id = 5;
        self.user.role.name = String::from("robot");

        self.iat = iat;
        self.exp = exp;
        self.user.user_token = None;

        let secret_key = match ConfigGetter::get_secret_key() {
            None => panic!("secret_key is mandatory"),
            Some(secret_key) => secret_key,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &self,
            &EncodingKey::from_secret(secret_key.as_ref()),
        )
    }

    pub fn encode_for_access(&mut self) -> Result<String, Error> {
        let iat = chrono::Utc::now().timestamp();
        let exp = iat + ACCESS_TOKEN_EXPIRATION;

        self.iat = iat;
        self.exp = exp;
        self.user.user_token = None;

        let secret_key = match ConfigGetter::get_secret_key() {
            None => panic!("secret_key is mandatory"),
            Some(secret_key) => secret_key,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &self,
            &EncodingKey::from_secret(secret_key.as_ref()),
        )
    }

    pub fn encode_for_refresh(&mut self) -> Result<String, Error> {
        let iat = chrono::Utc::now().timestamp();
        let exp = iat + REFRESH_TOKEN_EXPIRATION;

        self.iat = iat;
        self.exp = exp;

        if let None = self.user.user_token {
            // user_token is mandatory for refresh token
            panic!("user_token is mandatory for refresh token");
        }

        let secret_key = match ConfigGetter::get_secret_key() {
            None => panic!("secret_key is mandatory"),
            Some(secret_key) => secret_key,
        };

        encode(
            &Header::new(Algorithm::HS256),
            &self,
            &EncodingKey::from_secret(secret_key.as_ref()),
        )
    }
}
