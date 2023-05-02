use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubQuestion {
    pub id: i32,
    pub question_type: QuestionType,
    pub question: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct PubNewQuestion {
    pub question_type: QuestionType,
    pub question: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QuestionType {
    Checkbox,
    Input,
    Radio,
    Range,
}

impl fmt::Display for QuestionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                QuestionType::Checkbox => "checkbox",
                QuestionType::Input => "input",
                QuestionType::Radio => "radio",
                QuestionType::Range => "range",
            }
        )
    }
}
