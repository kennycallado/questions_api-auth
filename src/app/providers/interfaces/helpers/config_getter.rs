use serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ConfigGetter {
    pub fcm_url: Option<String>,
    pub form_url: Option<String>,
    pub migrations_run: Option<bool>,
    pub origin_url: Option<String>,
    pub profile_url: Option<String>,
    pub secret_key: Option<String>,
    pub user_url: Option<String>,
}

impl ConfigGetter {
    pub fn get_fcm_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .fcm_url
    }

    pub fn get_form_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .form_url
    }

    pub fn get_migrations_run() -> Option<bool> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .migrations_run
    }

    pub fn get_origin_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .origin_url
    }

    pub fn get_profile_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .profile_url
    }

    pub fn get_secret_key() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .secret_key
    }

    pub fn get_user_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .user_url
    }
}
