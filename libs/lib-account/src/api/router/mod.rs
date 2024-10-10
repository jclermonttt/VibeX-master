/*!
 * Provide Router for the micro-service `Account`
*/
use crate::config::state::ApiState;
use axum::{routing::post, Extension, Router};


pub async fn create_router() -> Router {
    Router::new()
      //  .route("/api/auth/signup", post())
        .with_state(ApiState)
}
