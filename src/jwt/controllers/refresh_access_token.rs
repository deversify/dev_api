use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use crate::result::Result;

use crate::jwt::{Jwt, TokenType, Claims};

#[derive(Deserialize)]
pub struct Args {
    refresh_token: String,
}

trait NewTrait: Claims + DeserializeOwned {}

pub async fn controller(args: web::Json<Args>, jwt: web::Data<Jwt>) -> Result<impl Responder> {
    let claims: Box<dyn NewTrait> = jwt.validate_jwt(&args.refresh_token, TokenType::refresh)?;
    //let response = jwt.create_tokens(Claims::new(claims.get_subject()))?;

    //Ok(HttpResponse::Ok().json(response))
    Ok(HttpResponse::Ok())
}
