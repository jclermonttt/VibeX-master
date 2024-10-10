use crate::models::prelude::*;
use crate::repository::{Error, Result};
use sea_orm::{ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter};

pub struct UserQuery;

impl UserQuery {
    pub async fn is_email_taken(txn: &DatabaseTransaction, email: &str) -> Result<bool> {
        let email_taken = User::find()
            .filter(UserColumn::Email.eq(email))
            .one(txn)
            .await
            .map_err(Error::DbErr)?
            .is_some();

        Ok(email_taken)
    }

    pub async fn is_username_taken(txn: &DatabaseTransaction, username: &str) -> Result<bool> {
        let username_taken = User::find()
        .filter(UserColumn::Username.eq(username))
        .one(txn)
        .await
        .map_err(Error::DbErr)?
        .is_some();

    Ok(username_taken)
    }
}
