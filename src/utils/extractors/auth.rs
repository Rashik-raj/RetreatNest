use std::any::Any;

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    entities_helper::{UserColumn, UserEntity, UserModel},
    serializers::auth::TokenClaim,
    state::AppState,
    utils::jwt::get_access_token_claim,
};

#[derive(Clone)]
pub struct AuthUser(pub UserModel);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync + std::fmt::Debug + Clone + 'static,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header: &str = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value: &header::HeaderValue| value.to_str().ok())
            .ok_or((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid Header".to_string(),
            ))?;

        let splitted_auth_header: Vec<&str> = auth_header.split(" ").collect();

        let (_schema, access_token) = (splitted_auth_header[0], splitted_auth_header[1]);

        let token_claim: TokenClaim = get_access_token_claim(access_token)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid Token".to_string()))?;

        let email: String = token_claim.email;
        let user_id: i64 = token_claim.user_id;
        let name: String = token_claim.name;

        let state: AppState = (state as &dyn Any)
            .downcast_ref::<AppState>()
            .ok_or_else(|| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to type cast app state".to_string(),
                )
            })
            .unwrap()
            .clone();

        let user: UserModel = UserEntity::find()
            .filter(UserColumn::Email.eq(email))
            .filter(UserColumn::UserId.eq(user_id))
            .filter(UserColumn::Name.eq(name))
            .one(&state.database)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .ok_or_else(|| (StatusCode::NOT_FOUND, "User not found".to_string()))?;

        Ok(AuthUser(user))
    }
}
