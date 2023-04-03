use rocket::http::Status;
use serde::{Serialize, Deserialize};

use crate::app::providers::interfaces::helpers::claims::{Claims, UserInClaims};
use crate::app::providers::interfaces::helpers::config_getter::ConfigGetter;

pub async fn fcm_token_delete(user_id: i32) -> Result<(), Status> {
    #[derive(Serialize, Deserialize)]
    struct NewFcmToken {
        pub user_id: i32,
        pub token: Option<String>,
    }

    let robot_token = robot_token_generator().await;
    if let Err(_) = robot_token {
        return Err(Status::InternalServerError);
    }
    let robot_token = robot_token.unwrap();

    let fcm_api_url = ConfigGetter::get_fcm_url()
        .unwrap_or("http://localhost:8005/api/v1/fcm".to_string())
        + "/token/"
        + user_id.to_string().as_str()
        + "/user";

    let client = reqwest::Client::new();
    let res = client
        .put(&fcm_api_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .header("Content-Type", "application/json")
        .json(&NewFcmToken {
            user_id,
            token: None,
        })
        .send()
        .await;

    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn profile_request(token: String) -> Result<i32, Status> {
    let robot_token = robot_token_generator().await;
    if let Err(_) = robot_token {
        return Err(Status::InternalServerError);
    }
    let robot_token = robot_token.unwrap();

    let profile_api_url = ConfigGetter::get_profile_url()
        .unwrap_or("http://localhost:8001/api/v1/profile".to_string())
        + "/token";

    let client = reqwest::Client::new();
    let res = client
        .post(&profile_api_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .header("Content-Type", "application/json")
        .json(&token)
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status() != 200 {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }

            Ok(res.json::<i32>().await.unwrap())
        }
        Err(_) => return Err(Status::InternalServerError),
    }
}

pub async fn user_request(user_id: i32) -> Result<UserInClaims, Status> {
    // Prepare the robot token
    let robot_token = robot_token_generator().await;
    if let Err(_) = robot_token {
        return Err(Status::InternalServerError);
    }
    let robot_token = robot_token.unwrap();

    // Prepare the url
    let user_url = ConfigGetter::get_user_url()
        .unwrap_or("http://localhost:8002/api/v1/user".to_string())
        + "/"
        + user_id.to_string().as_str()
        + "/userinclaims";

    // Make the request
    let client = reqwest::Client::new();
    let res = client
        .get(&user_url)
        .header("Accept", "application/json")
        .header("Authorization", robot_token)
        .header("Content-Type", "application/json")
        .send()
        .await;

    match res {
        Ok(res) => {
            if res.status() != 200 {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }

            Ok(res.json::<UserInClaims>().await.unwrap())
        }
        Err(_) => return Err(Status::InternalServerError),
    }
}

pub async fn token_generator(user_in_claims: UserInClaims) -> Result<(String, String), Status> {
    // let user_in_claims = user_request(user_id).await;
    // if let Err(_) = user_in_claims {
    //     return Err(Status::InternalServerError);
    // }
    // let user_in_claims = user_in_claims.unwrap();

    let mut claims: Claims = Claims::from(user_in_claims);

    let refresh_token = claims.encode_for_refresh();
    if let Err(_) = refresh_token {
        return Err(Status::InternalServerError);
    }
    let refresh_token = refresh_token.unwrap();

    // encode_for_access removes claims.user.user_token
    let access_token = claims.encode_for_access();
    if let Err(_) = access_token {
        return Err(Status::InternalServerError);
    }
    let access_token = access_token.unwrap();

    Ok((refresh_token, access_token))
}

pub async fn robot_token_generator() -> Result<String, Status> {
    let mut claims: Claims = Claims::from(UserInClaims::default());

    let access_token = claims.enconde_for_robot();
    if let Err(_) = access_token {
        return Err(Status::InternalServerError);
    }

    match access_token {
        Ok(access_token) => Ok(access_token),
        Err(_) => {
            return Err(Status::InternalServerError);
        }
    }
}
