use actix_web::{HttpRequest, HttpResponse};
use service::task::actions::get::TaskFetcherService;

use core::errors::{ServiceError, ServiceErrorStatus};

pub async fn get_all<T: TaskFetcherService>() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(T::get_tasks().await?))
}

pub async fn get_by_id<T: TaskFetcherService>(req: HttpRequest) -> Result<HttpResponse, ServiceError> {
    // TODO: Add validation for id

    match req.match_info().get("id") {
        Some(id) => {
            Ok(HttpResponse::Ok().json(T::get_tasks_by_id(id.parse::<i64>().unwrap()).await?))
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