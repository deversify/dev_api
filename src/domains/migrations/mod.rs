use actix_web::web;

mod repo;
mod run;

pub use repo::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/migrations").route("/run", web::get().to(run::controller)));
}
