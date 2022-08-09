use super::RepoImpl;
use actix_web::{web, HttpResponse, Responder};

pub async fn controller(repo: web::Data<RepoImpl>) -> crate::Result<impl Responder> {
    repo.run().await?;

    Ok(HttpResponse::Ok())
}
