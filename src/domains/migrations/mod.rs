use actix_web::web;

mod run;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/migrations").route("/run", web::get().to(run::controller)));
}
