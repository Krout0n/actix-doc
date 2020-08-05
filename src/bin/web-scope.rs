use actix_web::{web, App, HttpServer, Responder};

async fn show_users() -> impl Responder {
    "Hello, world!"
}

async fn ok() -> impl Responder {
    "ok"
}

async fn asai_ok() -> impl Responder {
    "ok!!!!!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/users")
                    .route("/show", web::get().to(show_users))
                    .route("/ok", web::get().to(ok)),
            )
            .route("/ok", web::get().to(asai_ok))
    })
    .bind("localhost:3000")?
    .run()
    .await
}
