use std::collections::HashMap;

use actix_web::{dev::Payload, http::header::Header, web, FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use futures::future::{err, ok, Ready};

use crate::{
    jwt::{self, Jwt},
    Error, Result,
};

pub struct Authorized {
    claims: HashMap<String, serde_json::Value>,
}

impl Authorized {
    pub fn has_role(&self, role: &str) -> Result<()> {
        let roles = self
            .claims
            .get("roles")
            .ok_or_else(|| crate::Error::not_authorized(role))?
            .as_array()
            .ok_or_else(crate::Error::identity_invalid)?;

        let has_role = roles.iter().any(|r| r == role);

        if !has_role {
            return Err(crate::Error::not_authorized(role));
        }
        Ok(())
    }

    pub fn get_claims(&self) -> &HashMap<String, serde_json::Value> {
        &self.claims
    }

    fn authorize(req: &HttpRequest) -> Result<Self> {
        let auth = Authorization::<Bearer>::parse(req).map_err(|e| {
            println!("{:?}", e);
            Error::bad_request_header("Authorization")
        })?;

        let jwt = auth.as_ref().token();

        let jwt_manager = match req.app_data::<web::Data<Jwt>>() {
            Some(jwt) => jwt,
            None => {
                println!("Could not load JWT manager.");
                return Err(Error::internal_error());
            }
        };

        Ok(Authorized {
            claims: jwt_manager.validate_jwt(jwt, jwt::TokenType::access)?,
        })
    }
}

impl FromRequest for Authorized {
    type Error = Error;
    type Future = Ready<Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match Authorized::authorize(req) {
            Ok(auth) => ok(auth),
            Err(e) => err(e),
        }
    }
}
