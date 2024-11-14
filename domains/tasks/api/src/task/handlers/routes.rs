use actix_web::web::{get, scope, ServiceConfig};
use service::task::service::TaskService;
use crate::task::handlers::get::get_all;

pub fn task_routes(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1/tasks")
            .route("", get().to(get_all::<TaskService>))
    );
}