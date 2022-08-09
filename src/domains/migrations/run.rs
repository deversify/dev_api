use actix_web::Responder;

pub async fn controller() -> crate::Result<impl Responder> {
    Ok("TODO")
}
