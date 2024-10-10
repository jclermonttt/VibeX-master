use crate::models::{prelude::*, user, SignUpInput};
use sea_orm::{ActiveModelTrait, DatabaseTransaction, DbConn, Set, Transaction};
use validator::Validate;

use super::error::{Error, Result};

pub struct Mutation;

impl Mutation {
    pub async fn insert_user(
        txn: &DatabaseTransaction,
        input: &SignUpInput,
    ) -> Result<UserActiveModel> {
        input
            .validate()
            .map_err(|e| Error::InvalidInput(format!("Validation error: {:?}", e)))?;

        let user = UserActiveModel {
            first_name: Set(input.first_name.clone()),
            last_name: Set(input.last_name.clone()),
            username: Set(input.username.clone()),
            email: Set(input.email.clone()),
            password_hash: Set(input.password.clone()),
            ..Default::default()
        }
        .insert(txn)
        .await
        .map_err(Error::DbErr)?;

        Ok(user.into())
    }
}
