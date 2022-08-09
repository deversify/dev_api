use super::RepoMySqlImpl;
use actix_web::{web, HttpResponse, Responder};

pub async fn controller(repo: web::Data<RepoMySqlImpl>) -> crate::Result<impl Responder> {
    repo.run().await?;

    Ok(HttpResponse::Ok())
}
