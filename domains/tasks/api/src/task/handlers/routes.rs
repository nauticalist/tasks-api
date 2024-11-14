use actix_web::web::{get, post, scope, ServiceConfig};
use service::task::service::TaskService;
use crate::task::handlers::create::create;
use crate::task::handlers::get::{get_all, get_by_id};

pub fn task_routes(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1/tasks")
            .route("", get().to(get_all::<TaskService>))
            .route("/{id}", get().to(get_by_id::<TaskService>))
            .route("", post().to(create::<TaskService>))
    );
}