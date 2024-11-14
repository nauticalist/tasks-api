use actix_web::web::ServiceConfig;

pub mod handlers;

pub fn tasks_api(app: &mut ServiceConfig) {
    handlers::routes::task_routes(app)
}