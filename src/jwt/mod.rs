use std::{collections::HashMap, fmt, str::FromStr};

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, Header, Validation};
use serde::{Deserialize, Serialize};

use self::{keys::Keys, tokens::Tokens};

const JWT_EXP_MINUTES: i64 = 15;
const REFRESH_TOKEN_EXP_WEEKS: i64 = 52;

pub fn get_access_expiration_seconds() -> i64 {
    Duration::minutes(JWT_EXP_MINUTES).num_seconds()
}

use crate::{result, Error, Result};

pub mod controllers;
pub mod keys;
pub mod tokens;

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TokenType {
    access,
    refresh,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for TokenType {
    type Err = result::Error;

    fn from_str(input: &str) -> Result<TokenType> {
        match input {
            "access" => Ok(TokenType::access),
            "refresh" => Ok(TokenType::refresh),
            _ => Err(Error::identity_invalid()),
        }
    }
}

#[derive(Clone)]
pub struct Jwt {
    keys: Keys,
}

impl Jwt {
    /// # Example
    /// ```rust
    /// use dev_api::jwt::Jwt;
    ///
    /// let jwt = Jwt::new(b"secret");
    ///
    pub fn new(bytes: &[u8]) -> Self {
        Self {
            keys: Keys::new(bytes),
        }
    }

    /// # Example
    /// ```rust
    /// use std::collections::HashMap;
    /// use dev_api::jwt::Jwt;
    /// use std::str::FromStr;
    ///
    /// let jwt = Jwt::new(b"secret");
    /// let tokens = jwt.create_tokens_from_str(HashMap::from([("sub", "2ca44c87-f9a9-4cb8-bffc-7e5d2e6350cb")]));
    /// ```
    pub fn create_tokens_from_str(&self, extra_claims: HashMap<&str, &str>) -> Result<Tokens> {
        self.create_tokens(
            extra_claims
                .into_iter()
                .map(|(k, v)| (k.to_string(), serde_json::Value::String(v.to_string())))
                .collect(),
        )
    }

    pub fn create_tokens(
        &self,
        extra_claims: HashMap<String, serde_json::Value>,
    ) -> Result<Tokens> {
        let access_token = Self::create_jwt(&extra_claims, &self.keys, TokenType::access)?;
        let refresh_token = Self::create_jwt(&extra_claims, &self.keys, TokenType::refresh)?;

        let response = Tokens {
            access_token,
            token_type: "Bearer".to_string(),
            expires_in: get_access_expiration_seconds(),
            refresh_token,
        };

        Ok(response)
    }

    fn create_jwt(
        extra_claims: &HashMap<String, serde_json::Value>,
        keys: &Keys,
        r#type: TokenType,
    ) -> Result<String> {
        let mut claims = extra_claims.clone();
        let expiration_duration = match r#type {
            TokenType::access => Duration::minutes(JWT_EXP_MINUTES),
            TokenType::refresh => Duration::weeks(REFRESH_TOKEN_EXP_WEEKS),
        };
        claims.insert(
            "type".to_string(),
            serde_json::Value::String(r#type.to_string()),
        );
        claims.insert(
            "exp".to_string(),
            serde_json::Value::Number(serde_json::Number::from(
                (Utc::now() + expiration_duration).timestamp(),
            )),
        );

        let token = jsonwebtoken::encode(&Header::default(), &claims, keys.get_encoding_key())
            .map_err(|e| {
                println!("{:?}", e);
                Error::internal_error()
            })?;

        Ok(token)
    }

    pub fn validate_jwt(
        &self,
        jwt: &str,
        r#type: TokenType,
    ) -> Result<HashMap<String, serde_json::Value>> {
        let claims: HashMap<String, serde_json::Value> = jsonwebtoken::decode(
            jwt,
            self.keys.get_decoding_key(),
            &Validation::new(Algorithm::default()),
        )
        .map_err(|e| match e.into_kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => match r#type {
                TokenType::access => Error::access_token_expired(),
                TokenType::refresh => Error::refresh_token_expired(),
            },
            _ => Error::identity_invalid(),
        })?
        .claims;

        if let Some(raw_token_type) = claims.get("type") {
            if let Some(token_type_str) = raw_token_type.as_str() {
                if let Ok(token_type) = token_type_str.parse() {
                    let result = if r#type == token_type {
                        Ok(claims)
                    } else {
                        Err(Error::bad_request("Invalid token type"))
                    };

                    return result;
                }
            }
        }

        Err(Error::identity_invalid())
    }
}
