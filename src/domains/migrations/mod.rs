use actix_web::web;

mod repo;
mod run_mysql;
mod run_postgres;

pub use repo::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/mysql_migrations").route("/run", web::get().to(run_mysql::controller)),
    );

    cfg.service(
        web::scope("/postgres_migrations").route("/run", web::get().to(run_postgres::controller)),
    );
}
