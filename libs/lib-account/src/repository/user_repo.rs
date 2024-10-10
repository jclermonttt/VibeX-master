//! User-related data operations
//!
//!
use sea_orm::{DatabaseTransaction, TransactionTrait};

use crate::models::payloads::SignUpPayload;
use crate::models::prelude::*;
use crate::repository::{Error, Result};

use super::mutations::UserMutation;
use super::queries::UserQuery;

#[derive(Clone)]
pub struct UserRepo;

impl UserRepo {
    pub async fn create_user(
        &self,
        txn: &DatabaseTransaction,
        payload: SignUpPayload,
    ) -> Result<UserActiveModel> {
        let txn = txn.begin().await.map_err(Error::DbErr)?;

        if UserQuery::is_email_taken(&txn, &payload.email).await? {
            txn.rollback().await.map_err(Error::DbErr)?;
            return Err(Error::new("Email is already in use"));
        }

        if UserQuery::is_username_taken(&txn, &payload.username).await? {
            txn.rollback().await.map_err(Error::DbErr)?;
            return Err(Error::new("Username is already in use"));
        }

        let user = UserMutation::insert_user(&txn, &payload).await?;

        txn.commit().await.map_err(Error::DbErr)?;

        Ok(user)
    }
}
