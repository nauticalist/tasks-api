use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use persistence::migrations::run_migrations;

mod task;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run_migrations().await;
    HttpServer::new(|| {
        App::new().configure(task::tasks_api)
    }).workers(4)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("I am alive")
}
