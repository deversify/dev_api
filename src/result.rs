use actix_web::HttpResponse;
use derive_more::Display;
use serde::Serialize;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    //NotFound,
    InternalError,
    AuthFailed,
    AccessTokenExpired,
    RefreshTokenExpired,
        //Forbidden,
    BadRequest, /* AccessDenied,
                , */
}

#[derive(Debug, Display, Serialize, PartialEq)]
#[display(fmt = "[{}]:{}", code, message)]
pub struct Error {
    message: String,
    code: ErrorCode,
    cid: Uuid,
}

impl Error {
    /* pub fn not_found(id: &str) -> Self {
        Self {
            code: ErrorCode::NotFound,
            message: format!("Could not find {}.", id),
            cid: Uuid::new_v4(),
        }
    } */

    pub fn bad_request_header(header_name: &str) -> Self {
        Self {
            code: ErrorCode::BadRequest,
            message: format!("Bad header: '{header_name}'."),
            cid: Uuid::new_v4(),
        }
    }

    pub fn bad_request(message: &str) -> Self {
        Self {
            code: ErrorCode::BadRequest,
            message: message.into(),
            cid: Uuid::new_v4(),
        }
    }

    pub fn authentication_failed() -> Self {
        Self {
            code: ErrorCode::AuthFailed,
            message: "Authentication failed.".into(),
            cid: Uuid::new_v4(),
        }
    }

    pub fn access_token_expired() -> Self {
        Self {
            code: ErrorCode::AccessTokenExpired,
            message: "Access token has expired.".into(),
            cid: Uuid::new_v4(),
        }
    }

    pub fn refresh_token_expired() -> Self {
        Self {
            code: ErrorCode::RefreshTokenExpired,
            message: "Refresh token has expired.".into(),
            cid: Uuid::new_v4(),
        }
    }

    pub fn internal_error() -> Self {
        Self {
            code: ErrorCode::InternalError,
            message: "Something went wrong.".into(),
            cid: Uuid::new_v4(),
        }
    } 
}

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let mut response = match self.code {
            ErrorCode::InternalError => HttpResponse::InternalServerError(),
            //ErrorCode::NotFound => HttpResponse::NotFound(),
            _ => HttpResponse::BadRequest(),
        };

        response.json(self)
    }
}