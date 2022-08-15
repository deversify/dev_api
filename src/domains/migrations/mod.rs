use actix_web::web;

mod repo;
mod run;
mod run_mysql;

pub use repo::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/migrations").route("/run", web::get().to(run::controller)));

    cfg.service(
        web::scope("/mysql_migrations").route("/run", web::get().to(run_mysql::controller)),
    );
}
