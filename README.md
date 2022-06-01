# dev_api

A set of pre-configured modules for Rust web APIs. Currently in early stage development.

## Purpose

We want to open source how we do backend development in Rust and offer a fast and simple way to get started writing web APIs without locking you in into another custom framework.

### Example

Note: given how early stage this project is, the example is not complete. You need to install actix-web and create your own configs and dependencies that you want to inject into app_data.

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let server = HttpServer::new(move || {
        // The configure function must be a ServiceConfig factory function: https://docs.rs/actix-web/latest/actix_web/web/struct.ServiceConfig.html
        let configs: Vec<fn(&mut web::ServiceConfig)> = vec![
            users::configure,
            products::configure
        ];

        // dev_api will mount your services, configure the app, and send the app back so you can extend it further.
        let app = dev_api::http::new(configs);

        // Extend the app with your own dependencies.
        app
            .app_data(web::Data::new(jwt.clone()))
            .app_data(web::Data::new(users_repo.clone()))
            .app_data(web::Data::new(products_repo.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    println!("Server started!");
    server.await
```

Please look at the source code for more information. `src/http.rs` will show how we setup the server app.

When it works, you can verify it by going to `localhost:8080` in your browser. You should get back an empty 200 success response in your network tab.
