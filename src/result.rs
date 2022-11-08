use actix_web::HttpResponse;
use derive_more::Display;
use serde::Serialize;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    NotFound,
    InternalError,
    IdentityInvalid,
    AccessTokenExpired,
    RefreshTokenExpired,
    SignInTokenExpired,
    SignInTokenInvalid,
    //Forbidden,
    BadRequest,
    NotAuthorized,
    AccessDenied,
}

#[derive(Debug, Display, Serialize, PartialEq)]
#[display(fmt = "[{}]:{}", code, message)]
pub struct Error {
    message: String,
    code: ErrorCode,
    cid: Uuid,
}

impl Error {
    pub fn not_found(id: &str) -> Self {
        Self {
            code: ErrorCode::NotFound,
            message: format!("Could not find {}.", id),
            cid: Uuid::new_v4(),
        }
    }

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

    pub fn identity_invalid() -> Self {
        Self {
            code: ErrorCode::IdentityInvalid,
            message: "Identity invalid. Try signing in again.".into(),
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

    pub fn sign_in_token_expired() -> Self {
        Self {
            code: ErrorCode::SignInTokenExpired,
            message: "Sign in token has expired.".into(),
            cid: Uuid::new_v4(),
        }
    }

    pub fn sign_in_token_invalid() -> Self {
        Self {
            code: ErrorCode::SignInTokenInvalid,
            message: "Sign in token is invalid. The token might already have been used. Try initiating a sign in again.".into(),
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

    pub(crate) fn not_authorized(role_required: &str) -> Self {
        Self {
            code: ErrorCode::NotAuthorized,
            message: format!("Missing required role {role_required}."),
            cid: Uuid::new_v4(),
        }
    }

    pub fn access_denied(actual_role: &str, expected_role: &str) -> Self {
        Self {
            code: ErrorCode::AccessDenied,
            message: format!(
                "Access denied. Found role {actual_role}, expected role {expected_role}"
            ),
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

        let cid_string = self.cid.to_string();
        let cid = cid_string.as_str();

        tracing::event!(
            tracing::Level::WARN,
            application_cid = cid,
            "Application CID"
        );

        response.json(self)
    }
}
