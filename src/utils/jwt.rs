use std::env;

use async_graphql::{Context, Error};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: String,
    pub iat: i64,
    pub exp: i64,
}

fn get_secret() -> String {
    env::var("JWT_SECRET").unwrap()
}

pub fn make_jwt(id: &str) -> String {
    let header = Header::new(Algorithm::HS512);
    let now = Utc::now();
    let claims = Claims {
        id: String::from(id),
        iat: now.timestamp(),
        exp: (now + Duration::hours(8)).timestamp(),
    };

    let secret = get_secret();
    encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

fn decode_jwt(jwt: &str) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    let secret = get_secret();
    decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS512),
    )
}

fn decode_bearer_token(token: &str) -> Result<TokenData<Claims>, String> {
    let mut split_token = token.split_whitespace();
    match split_token.next() {
        Some(schema_type) => {
            if schema_type != "Bearer" {
                return Err(format!("invalid schema type"));
            }
        }
        None => return Err(format!("not found schema type")),
    };
    let jwt = match split_token.next() {
        Some(jwt) => jwt,
        None => return Err(format!("not found jwt token")),
    };
    match decode_jwt(jwt) {
        Ok(v) => Ok(v),
        Err(err) => Err(format!("decode error: {}", err.to_string())),
    }
}

pub fn decode_context_token(ctx: &Context<'_>) -> async_graphql::Result<TokenData<Claims>> {
    let token = match ctx.data_opt::<String>() {
        Some(v) => Ok(v),
        None => Err(Error::new("no token")),
    }?;
    match decode_bearer_token(token) {
        Ok(v) => Ok(v),
        Err(err) => Err(Error::new(err)),
    }
}
