use super::{
    error::{Error, Result},
    Mutation, Query,
};
use crate::models::{SignUpPayload, UserActiveModel};
use sea_orm::DatabaseTransaction;

#[derive(Clone)]
pub struct AuthRepoService;

impl AuthRepoService {
    pub async fn create_user(
        &self,
        txn: &DatabaseTransaction,
        payload: SignUpPayload,
    ) -> Result<UserActiveModel> {
        Query::check_email_use(txn, &payload.email).await?;

        Query::check_username_use(txn, &payload.username).await?;

        Mutation::insert_user(txn, &payload).await
    }
}
