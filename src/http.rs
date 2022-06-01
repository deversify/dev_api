use crate::ensure_env;
use crate::health;
use actix_cors::Cors;
use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    http, web, Error,
};
use tracing_actix_web::TracingLogger;

pub fn new(
    configs: Vec<fn(&mut web::ServiceConfig)>,
) -> actix_web::App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        InitError = (),
        Error = Error,
    >,
> {
    let frontend_host = ensure_env("FRONTEND_HOST");

    let cors = Cors::default()
        .allowed_origin(&frontend_host)
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .max_age(3600);

    let app = actix_web::App::new()
        .wrap(TracingLogger::default())
        .wrap(cors);

    let mut scope = web::scope("");

    for config in configs.clone() {
        scope = scope.configure(config);
    }

    scope = scope.configure(health::configure);
    app.service(scope)
}
