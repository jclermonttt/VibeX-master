/*!
 * Provide all input needed for the micro-service `Account`
*/

use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SignUpInput {
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignInInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: String,
}