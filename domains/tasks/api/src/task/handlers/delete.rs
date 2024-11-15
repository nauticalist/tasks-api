use actix_web::{HttpRequest, HttpResponse};
use core::errors::{ServiceError, ServiceErrorStatus};
use service::task::actions::delete::TaskDeleterService;

pub async fn delete_task<T: TaskDeleterService>(req: HttpRequest) -> Result<HttpResponse, ServiceError> {
    match req.match_info().get("id") {
        Some (id) => {
            T::delete_task(id.parse::<i64>().unwrap()).await?;
            Ok(HttpResponse::NoContent().finish())

        },
        None => {
            Err(
                ServiceError::new(
                    "Proper task id not provided".to_string(),
                    ServiceErrorStatus::BadRequest
                )
            )
        }
    }
}