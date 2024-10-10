use super::{
    error::{Error, Result},
    Mutation, Query,
};
use crate::models::{SignUpInput, UserActiveModel};
use sea_orm::DatabaseTransaction;

#[derive(Clone)]
pub struct AuthRepoService;

impl AuthRepoService {
    pub async fn create_user(
        &self,
        txn: &DatabaseTransaction,
        input: SignUpInput,
    ) -> Result<UserActiveModel> {
        Query::check_email_use(txn, &input.email).await?;

        Query::check_username_use(txn, &input.username).await?;

        Mutation::insert_user(txn, &input).await
    }
}
