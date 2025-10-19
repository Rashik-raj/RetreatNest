use std::error::Error;

use jwt_simple::prelude::*;
use jwt_simple::{
    claims::{Claims, JWTClaims},
    prelude::{Duration, HS256Key},
    reexports::ct_codecs::{Decoder, Encoder, Hex},
};

use crate::{env, serializers::auth::TokenClaim};

#[allow(unused)]
pub async fn generate_jwt_key() -> String {
    let key: Vec<u8> = HS256Key::generate().to_bytes();
    // Each byte becomes 2 hex chars → 32 bytes = 64 chars
    let mut encoded_buf = [0u8; 64];
    // Perform encoding
    let encoded_str = Hex::encode_to_str(&mut encoded_buf, key).unwrap();
    encoded_str.to_string()
}

pub async fn get_jwt_key(key_str: &str) -> Vec<u8> {
    let mut encoded_buf = [0u8; 64];
    let key: Vec<u8> = Hex::decode(&mut encoded_buf, key_str.as_bytes(), None)
        .unwrap()
        .into();
    key
}

pub async fn get_access_jwt_key() -> HS256Key {
    let access_key: String = env::ENV.jwt_access_key.to_string();
    let byte_access_key: Vec<u8> = get_jwt_key(&access_key).await;
    let h256_access_key: HS256Key = HS256Key::from_bytes(&byte_access_key);
    h256_access_key
}

pub async fn get_refresh_jwt_key() -> HS256Key {
    let refresh_key: String = env::ENV.jwt_refresh_key.to_string();
    let byte_refresh_key: Vec<u8> = get_jwt_key(&refresh_key).await;
    let h256_refresh_key: HS256Key = HS256Key::from_bytes(&byte_refresh_key);
    h256_refresh_key
}

pub async fn generate_access_token(token_claim: TokenClaim) -> Result<String, Box<dyn Error>> {
    let h256_access_key: HS256Key = get_access_jwt_key().await;

    let access_claims: JWTClaims<TokenClaim> = Claims::with_custom_claims(
        token_claim,
        Duration::from_mins(env::ENV.jwt_access_lifetime_in_min),
    );
    let access_token: String = h256_access_key.authenticate(access_claims)?;
    Ok(access_token)
}

pub async fn generate_refresh_token(token_claim: TokenClaim) -> Result<String, Box<dyn Error>> {
    let h256_refresh_key: HS256Key = get_refresh_jwt_key().await;

    let refresh_claims: JWTClaims<TokenClaim> = Claims::with_custom_claims(
        token_claim,
        Duration::from_mins(env::ENV.jwt_refresh_lifetime_in_min),
    );
    let refresh_token: String = h256_refresh_key.authenticate(refresh_claims)?;
    Ok(refresh_token)
}

pub async fn get_access_token_claim(access_token: &str) -> Result<TokenClaim, Box<dyn Error>> {
    let h256_access_key: HS256Key = get_access_jwt_key().await;

    let mut options: VerificationOptions = VerificationOptions::default();
    // Accept tokens that will only be valid in the future.
    options.accept_future = true;
    // Accept tokens even if they have expired up to 15 minutes after the deadline,
    // and/or they will be valid within 15 minutes.
    // Note that 15 minutes is the default, since it is very common for clocks to be slightly off.
    options.time_tolerance = Some(Duration::from_mins(0));
    // Reject tokens if they were issued more than 1 hour ago
    options.max_validity = Some(Duration::from_hours(1));

    let claims: JWTClaims<TokenClaim> =
        h256_access_key.verify_token::<TokenClaim>(access_token, Some(options))?;
    Ok(claims.custom)
}

pub async fn get_refresh_token_claim(refresh_token: &str) -> Result<TokenClaim, Box<dyn Error>> {
    let h256_refresh_key: HS256Key = get_refresh_jwt_key().await;

    let mut options: VerificationOptions = VerificationOptions::default();
    // Accept tokens that will only be valid in the future.
    options.accept_future = true;
    // Accept tokens even if they have expired up to 15 minutes after the deadline,
    // and/or they will be valid within 15 minutes.
    // Note that 15 minutes is the default, since it is very common for clocks to be slightly off.
    options.time_tolerance = Some(Duration::from_mins(0));
    // Reject tokens if they were issued more than 1 hour ago
    options.max_validity = Some(Duration::from_hours(1));

    let claims: JWTClaims<TokenClaim> =
        h256_refresh_key.verify_token::<TokenClaim>(refresh_token, Some(options))?;
    Ok(claims.custom)
}
