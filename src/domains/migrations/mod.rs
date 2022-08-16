use actix_web::web;

mod list;
mod list_mysql;
mod repo;
mod run;
mod run_mysql;
mod types;

pub use repo::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/migrations").route("/run", web::post().to(run::controller)))
        .route("/list", web::get().to(list::controller));

    cfg.service(
        web::scope("/mysql_migrations")
            .route("/run", web::post().to(run_mysql::controller))
            .route("/list", web::get().to(list_mysql::controller)),
    );
}
