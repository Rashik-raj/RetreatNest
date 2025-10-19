use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use crate::{entities::users::Model as UserModel, utils::serializer::deserialize_some};
use validator::{Validate, ValidationError};

fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if phone.len() < 9 {
        return Err(ValidationError::new("Validation").with_message(Cow::from("Invalid phone number")))
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUserSerializer{
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
    #[validate(custom(function="validate_phone"))]
    pub phone: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ReadUserSerializer{
    user_id: i64,
    name: String,
    email: String,
    phone: Option<String>

}

impl From<UserModel> for ReadUserSerializer{
    fn from(value: UserModel) -> Self {
        ReadUserSerializer { user_id: value.user_id, name: value.name, email: value.email, phone: value.phone }
    }
}


#[derive(Debug, Clone, Deserialize, Validate, Serialize)]
pub struct UpdateUserSerializer{
    pub name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(custom(function="validate_phone"))]
    #[serde(default, deserialize_with = "deserialize_some")]
    pub phone: Option<Option<String>>,
}