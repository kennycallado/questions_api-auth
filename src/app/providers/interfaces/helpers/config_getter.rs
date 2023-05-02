use serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ConfigGetter {
    pub origin_url: Option<String>,
    pub secret_key: Option<String>,
    pub migrations_run: Option<bool>,
    //
    pub profile_url: Option<String>,
    pub user_url: Option<String>,
    pub auth_url: Option<String>,
    //
    pub fcm_url: Option<String>,
    //
    pub question_url: Option<String>,
    pub answer_url: Option<String>,
    //
    pub slide_url: Option<String>,
    pub form_url: Option<String>,
    pub external_url: Option<String>,
    //
    pub resource_url: Option<String>,
    pub paper_url: Option<String>,
}

impl ConfigGetter {
    pub fn get_entity_url(entity: String) -> Option<String> {
        match entity.as_str() {
            "profile" => ConfigGetter::get_profile_url(),
            "user" => ConfigGetter::get_user_url(),
            "auth" => ConfigGetter::get_auth_url(),
            //
            "fcm" => ConfigGetter::get_fcm_url(),
            //
            "question" => ConfigGetter::get_question_url(),
            "answer" => ConfigGetter::get_answer_url(),
            //
            "slide" => ConfigGetter::get_slide_url(),
            "form" => ConfigGetter::get_form_url(),
            "external" => ConfigGetter::get_external_url(),
            //
            "resource" => ConfigGetter::get_resource_url(),
            "paper" => ConfigGetter::get_paper_url(),
            _ => None,
        }
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

    pub fn get_secret_key() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .secret_key
    }
}

impl ConfigGetter {
    fn get_profile_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .profile_url
    }

    fn get_user_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .user_url
    }

    fn get_auth_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .auth_url
    }

    fn get_fcm_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .fcm_url
    }

    fn get_question_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .question_url
    }

    fn get_answer_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .answer_url
    }

    fn get_slide_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .slide_url
    }

    fn get_form_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .form_url
    }

    fn get_external_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .external_url
    }

    fn get_resource_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .resource_url
    }

    fn get_paper_url() -> Option<String> {
        rocket::Config::figment()
            .extract::<ConfigGetter>()
            .unwrap()
            .paper_url
    }
}
