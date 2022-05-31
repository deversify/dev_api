use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use self::{keys::Keys, tokens::Tokens};

const JWT_EXP_MINUTES: i64 = 15;
const REFRESH_TOKEN_EXP_WEEKS: i64 = 52;

pub fn get_access_expiration_seconds() -> i64 {
    Duration::minutes(JWT_EXP_MINUTES).num_seconds()
}

use crate::{Error, Result};

pub mod keys;
pub mod tokens;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TokenType {
    access,
    refresh,
}

pub trait Claims {
    fn set_expiration(&mut self, expiration: i64);
    fn set_type(&mut self, token_type: TokenType);
    fn get_type(&self) -> TokenType;
}

#[derive(Clone)]
pub struct Jwt {
    keys: Keys,
}

impl Jwt {
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            keys: Keys::new(bytes),
        }
    }

    pub fn create_tokens<T: Claims + Serialize>(&self, mut claims: T) -> Result<Tokens> {
        let access_token = Self::create_jwt(&mut claims, &self.keys, TokenType::access)?;
        let refresh_token = Self::create_jwt(&mut claims, &self.keys, TokenType::refresh)?;

        let response = Tokens {
            access_token,
            token_type: "Bearer",
            expires_in: get_access_expiration_seconds(),
            refresh_token,
        };

        Ok(response)
    }

    fn create_jwt<T: Claims + Serialize>(
        claims: &mut T,
        keys: &Keys,
        r#type: TokenType,
    ) -> Result<String> {
        Self::set_claims(claims, r#type);

        let token = jsonwebtoken::encode(&Header::default(), &claims, keys.get_encoding_key())
            .map_err(|e| {
                println!("{:?}", e);
                Error::internal_error()
            })?;

        Ok(token)
    }

    fn set_claims<T: Claims + Serialize>(claims: &mut T, r#type: TokenType) {
        let expiration_duration = match r#type {
            TokenType::access => Duration::minutes(JWT_EXP_MINUTES),
            TokenType::refresh => Duration::weeks(REFRESH_TOKEN_EXP_WEEKS),
        };

        claims.set_type(r#type);
        claims.set_expiration((Utc::now() + expiration_duration).timestamp());
    }

    pub fn validate_jwt<T: Claims + DeserializeOwned>(
        &self,
        jwt: &str,
        r#type: TokenType,
    ) -> Result<T> {
        let claims: T = jsonwebtoken::decode(
            jwt,
            self.keys.get_decoding_key(),
            &Validation::new(Algorithm::default()),
        )
        .map_err(|e| match e.into_kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => match r#type {
                TokenType::access => Error::access_token_expired(),
                TokenType::refresh => Error::refresh_token_expired(),
            },
            _ => Error::authentication_failed(),
        })?
        .claims;

        if r#type == claims.get_type() {
            Ok(claims)
        } else {
            Err(Error::bad_request("Invalid token type"))
        }
    }
}
