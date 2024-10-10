use crate::models::{prelude::*, user, SignInPayload, SignUpPayload};
use sea_orm::{ActiveModelTrait, DatabaseTransaction, DbConn, Set, Transaction};
use validator::Validate;

use super::error::{Error, Result};

pub struct Mutation;

impl Mutation {
    pub async fn insert_user(
        txn: &DatabaseTransaction,
        payload: &SignUpPayload,
    ) -> Result<UserActiveModel> {
        payload
            .validate()
            .map_err(|e| Error::InvalidInput(format!("Validation error: {:?}", e)))?;

        let user = UserActiveModel {
            first_name: Set(payload.first_name.clone()),
            last_name: Set(payload.last_name.clone()),
            username: Set(payload.username.clone()),
            email: Set(payload.email.clone()),
            password_hash: Set(payload.password.clone()),
            ..Default::default()
        }
        .insert(txn)
        .await
        .map_err(Error::DbErr)?;

        Ok(user.into())
    }
}
