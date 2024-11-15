use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::Json;
use core::errors::{ServiceError, ServiceErrorStatus};
use service::task::actions::update::TaskUpdaterService;
use service::task::models::TaskDto;

pub async fn update_task<T: TaskUpdaterService>(req: HttpRequest, body: Json<TaskDto>) -> Result<HttpResponse, ServiceError> {
    match req.match_info().get("id") {
        Some(_id) => {
            let updated = T::update_task(body.into_inner()).await?;
            Ok(HttpResponse::Ok().json(updated))
        },
        None => {
            return Err(
                ServiceError::new(
                    "Proper task id not provided".to_string(),
                    ServiceErrorStatus::BadRequest
                )
            )
        }
    }
}
