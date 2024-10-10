use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignUpPayload {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}