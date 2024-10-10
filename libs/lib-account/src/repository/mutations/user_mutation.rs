use sea_orm::{ActiveModelTrait, DatabaseTransaction, Set};

use crate::models::payloads::SignUpPayload;
use crate::models::prelude::*;
use crate::repository::{Result, Error};

pub struct UserMutation;

impl UserMutation {
    pub async fn insert_user(
        txn: &DatabaseTransaction,
        payload: &SignUpPayload,
    ) -> Result<UserActiveModel> {
       let insert_user = UserActiveModel {
        first_name: Set(payload.first_name.clone()),
        last_name: Set(payload.last_name.clone()),
        username: Set(payload.username.clone()),
        email: Set(payload.email.clone()),
        password_hash: Set(payload.password.clone()),
        ..Default::default()
       }
       .save(txn)
       .await?;

       Ok(insert_user)
    }
}