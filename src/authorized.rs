use actix_web::{dev::Payload, http::header::Header, web, FromRequest, HttpRequest};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use futures::future::{err, ok, Ready};
use serde::de::DeserializeOwned;

use crate::{
    jwt::{self, Jwt},
    Error, Result,
};

use super::jwt::Claims;

pub struct Authorized<T: Claims + DeserializeOwned> {
    claims: T,
}

impl<T: Claims + DeserializeOwned> Authorized<T> {
    pub fn get_claims(&self) -> &T {
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
            claims: *jwt_manager.validate_jwt(jwt, jwt::TokenType::access)?,
        })
    }
}

impl<T: Claims + DeserializeOwned> FromRequest for Authorized<T> {
    type Error = Error;
    type Future = Ready<Result<Self>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match Authorized::authorize(req) {
            Ok(auth) => ok(auth),
            Err(e) => err(e),
        }
    }
}
