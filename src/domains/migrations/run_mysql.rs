use super::RepoMySqlImpl;
use actix_web::{web, HttpResponse, Responder};

#[tracing::instrument(name = "mysql_migrations:run:controller", skip(repo, authorized))]
pub async fn controller(
    repo: web::Data<RepoMySqlImpl>,
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

    repo.run().await?;

    Ok(HttpResponse::Ok())
}
