use super::error::{Error, Result};
use crate::models::prelude::*;
use sea_orm::{ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};

pub struct Query;

impl Query {
    pub async fn check_email_use(txn: &DatabaseTransaction, email: &str) -> Result<bool> {
        let user_exist = User::find()
            .filter(UserColumn::Email.eq(email))
            .one(txn)
            .await
            .map_err(Error::DbErr)?
            .into_iter()
            .next()
            .is_some();

        Ok(user_exist)
    }

    pub async fn check_username_use(txn: &DatabaseTransaction, username: &str) -> Result<bool> {
        let username_in_use = User::find()
            .filter(UserColumn::Username.eq(username))
            .one(txn)
            .await
            .map_err(Error::DbErr)?
            .into_iter()
            .next()
            .is_some();

        Ok(username_in_use)
    }
}
