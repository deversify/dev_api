use super::RepoPostgresImpl;
use actix_web::{web, HttpResponse, Responder};

#[tracing::instrument(name = "migrations:list:controller", skip(repo, authorized))]
pub async fn controller(
    repo: web::Data<RepoPostgresImpl>,
    authorized: crate::Authorized,
) -> crate::Result<impl Responder> {
    let roles = authorized
        .get_claims()
        .get("roles")
        .ok_or_else(|| crate::Error::not_authorized("admin"))?
        .as_array()
        .ok_or_else(crate::Error::identity_invalid)?;

    let is_admin = roles.iter().any(|r| r == "admin");

    if !is_admin {
        return Err(crate::Error::not_authorized("admin"));
    }

    let result = repo.list().await?;

    Ok(HttpResponse::Ok().json(result))
}
