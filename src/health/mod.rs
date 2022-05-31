use actix_web::{web, Responder, HttpResponse};

async fn status() -> impl Responder {
    "ready"
}

async fn head_status() -> impl Responder {
    HttpResponse::Ok()
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::head().to(head_status));
    cfg.route("/", web::get().to(status));
    cfg.service(web::scope("/health").route("/status", web::get().to(status)));
}
