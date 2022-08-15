use super::RepoPostgresImpl;
use actix_web::{web, HttpResponse, Responder};

#[tracing::instrument(name = "migrations:run:controller", skip(repo))]
pub async fn controller(repo: web::Data<RepoPostgresImpl>) -> crate::Result<impl Responder> {
    repo.run().await?;

    Ok(HttpResponse::Ok())
}
